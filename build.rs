use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use quick_xml::Reader; // Updated to use the latest quick-xml
use quick_xml::events::Event;

// Helper to parse CLDR XML for a given locale
fn parse_cldr_keywords(path: &str) -> HashMap<String, (String, Vec<String>)> {
    let mut map = HashMap::new();
    let mut reader = Reader::from_file(path).expect("CLDR file not found");
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut cp = String::new();
    let mut tts = None;
    let mut keywords = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"annotation" => {
                cp = e.attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.as_ref() == b"cp")
                    .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
                    .unwrap_or_default();
                tts = None;
                keywords = None;
                for a in e.attributes().filter_map(|a| a.ok()) {
                    if a.key.as_ref() == b"type" && a.value.as_ref() == b"tts" {
                        tts = Some(String::new());
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if tts.is_some() {
                    tts = Some(e.unescape().unwrap().to_string());
                } else if !cp.is_empty() {
                    keywords = Some(
                        e.unescape().unwrap().split('|').map(|s| s.trim().to_string()).collect::<Vec<String>>()
                    );
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"annotation" => {
                if !cp.is_empty() {
                    let name = tts.clone().unwrap_or_default();
                    let kw = keywords.clone().unwrap_or_default();
                    map.insert(cp.clone(), (name, kw));
                }
                cp.clear();
                tts = None;
                keywords = None;
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    map
}

fn main() {

    // Read emoji-test.txt
    let emoji_test = fs::read_to_string("data/emoji-test.txt").expect("Download emoji-test.txt first!");

    // Parse CLDR for English and Dutch
    let cldr_en = parse_cldr_keywords("data/en.xml");
    let cldr_nl = parse_cldr_keywords("data/nl.xml");

    // Parse emoji-test.txt and group skin tone variants
    let skin_tone_mods: [(&str, &str); 5] = [
        ("1F3FB", "Light"),
        ("1F3FC", "MediumLight"),
        ("1F3FD", "Medium"),
        ("1F3FE", "MediumDark"),
        ("1F3FF", "Dark"),
    ];
    use std::collections::HashMap;
    let mut base_to_skin_tones: HashMap<String, [Option<String>; 5]> = HashMap::new();
    let mut emoji_rows = Vec::new();
    let mut current_category = "";
    let re = Regex::new(r"^([0-9A-F ]+); fully-qualified").unwrap();

    for line in emoji_test.lines() {
        if line.starts_with("# group:") {
            current_category = line.trim_start_matches("# group:").trim();
        } else if let Some(caps) = re.captures(line) {
            let codepoints = caps[1].split_whitespace()
                .map(|cp| u32::from_str_radix(cp, 16).unwrap())
                .collect::<Vec<_>>();
            let ch: String = codepoints.iter().filter_map(|&c| char::from_u32(c)).collect();
            let name_en = line.split('#').nth(1).unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
            // CLDR lookups
            let (name_en_cldr, keywords_en) = cldr_en.get(&ch).cloned().unwrap_or((name_en.clone(), vec![]));
            let (name_nl, keywords_nl) = cldr_nl.get(&ch).cloned().unwrap_or((name_en.clone(), vec![]));

            // Detect if this is a skin tone variant
            let mut is_skin_tone = false;
            let mut base = codepoints.clone();
            let mut skin_idx = None;
            if codepoints.len() > 1 {
                for (i, (mod_cp, _)) in skin_tone_mods.iter().enumerate() {
                    let mod_u32 = u32::from_str_radix(mod_cp, 16).unwrap();
                    if codepoints.contains(&mod_u32) {
                        is_skin_tone = true;
                        skin_idx = Some(i);
                        base.retain(|&c| c != mod_u32);
                        break;
                    }
                }
            }
            let base_ch: String = base.iter().filter_map(|&c| char::from_u32(c)).collect();
            if is_skin_tone {
                // Store in base_to_skin_tones
                if let Some(idx) = skin_idx {
                    base_to_skin_tones.entry(base_ch.clone()).or_insert([None, None, None, None, None])[idx] = Some(ch.clone());
                }
            } else {
                emoji_rows.push((ch.clone(), name_en, keywords_en, name_nl, keywords_nl, current_category.to_string(), base_ch.clone()));
            }
        }
    }

    // Generate Rust code with skin tone support
    let mut out = String::from("// This file is @generated by build.rs\n\n#[derive(Clone, Copy, PartialEq, Eq, Debug)]\npub enum SkinTone {\n    Default,\n    Light,\n    MediumLight,\n    Medium,\n    MediumDark,\n    Dark,\n}\n\n#[derive(Clone)]\npub struct Emoji {\n    pub ch: &'static str,\n    pub name_en: &'static str,\n    pub keywords_en: &'static [&'static str],\n    pub name_nl: &'static str,\n    pub keywords_nl: &'static [&'static str],\n    pub category: &'static str,\n    pub skin_tone_variants: Option<[&'static str; 5]>,\n}\n\npub static mut PREFERRED_SKIN_TONE: SkinTone = SkinTone::Default;\n\npub static EMOJIS: &[Emoji] = &[\n");
    for (ch, name_en, keywords_en, name_nl, keywords_nl, category, base_ch) in emoji_rows {
        let kw_en: Vec<String> = keywords_en.iter().map(|k| format!("\"{}\"", k)).collect();
        let kw_nl: Vec<String> = keywords_nl.iter().map(|k| format!("\"{}\"", k)).collect();
        let skin_opt = base_to_skin_tones.get(&ch).or_else(|| base_to_skin_tones.get(&base_ch));
        let skin_str = if let Some(arr) = skin_opt {
            let arr_str: Vec<String> = arr.iter().map(|opt| opt.as_ref().map(|s| format!("\"{}\"", s)).unwrap_or("None".to_string())).collect();
            format!("Some([{}])", arr_str.join(", "))
        } else {
            "None".to_string()
        };
        out.push_str(&format!(
            "    Emoji {{ ch: \"{}\", name_en: \"{}\", keywords_en: &[{}], name_nl: \"{}\", keywords_nl: &[{}], category: \"{}\", skin_tone_variants: {} }},\n",
            ch, name_en, kw_en.join(", "), name_nl, kw_nl.join(", "), category, skin_str
        ));
    }
    out.push_str("];\n");

    fs::write(Path::new("src/emoji/emoji_data.rs"), out).expect("Failed to write emoji_data.rs");
}
