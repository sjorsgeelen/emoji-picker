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
    let mut tts: Option<String> = None;
    let mut keywords: Option<Vec<String>> = None;

    let mut in_tts = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"annotation" => {
                cp = e.attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.as_ref() == b"cp")
                    .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
                    .unwrap_or_default();
                // Use raw cp attribute value, split by codepoint, join as space-separated uppercase hex
                // This matches emoji-test.txt format exactly
                let cp_codepoints = cp
                    .chars()
                    .map(|c| format!("{:X}", c as u32))
                    .collect::<Vec<_>>()
                    .join(" ");
                cp = cp_codepoints;
                in_tts = false;
                for a in e.attributes().filter_map(|a| a.ok()) {
                    if a.key.as_ref() == b"type" && a.value.as_ref() == b"tts" {
                        in_tts = true;
                        break;
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if !cp.is_empty() {
                    if in_tts {
                        tts = Some(e.unescape().unwrap().to_string());
                    } else {
                        // Split on | or ; or comma, as CLDR sometimes uses ; or comma for keywords
                        let text = e.unescape().unwrap();
                        let kws = text
                            .split(|c| c == '|' || c == ';' || c == ',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<String>>();
                        keywords = Some(kws);
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"annotation" => {
                if !cp.is_empty() {
                    let entry = map.entry(cp.clone()).or_insert((String::new(), Vec::new()));
                    if in_tts {
                        // Set tts (name)
                        if let Some(ref t) = tts {
                            entry.0 = t.clone();
                        }
                    } else {
                        // Set keywords
                        if let Some(ref kw) = keywords {
                            entry.1 = kw.clone();
                        }
                    }
                }
                cp.clear();
                tts = None;
                keywords = None;
                in_tts = false;
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    map
}

fn main() {

    // Read emoji-test.txt from the new location
    let emoji_test = fs::read_to_string("data/downloaded/emoji-test.txt").expect("Download emoji-test.txt first!");


    // Parse CLDR for English and Dutch from both annotation sources
    let cldr_en_main = parse_cldr_keywords("data/downloaded/common/annotations/en.xml");
    let cldr_en_derived = parse_cldr_keywords("data/downloaded/common/annotationsDerived/en.xml");
    let cldr_nl_main = parse_cldr_keywords("data/downloaded/common/annotations/nl.xml");
    let cldr_nl_derived = parse_cldr_keywords("data/downloaded/common/annotationsDerived/nl.xml");

    // Helper to merge tts and keywords from both sources
    fn merge_cldr(
        main: &HashMap<String, (String, Vec<String>)>,
        derived: &HashMap<String, (String, Vec<String>)>,
        key: &str,
        default_name: &str,
    ) -> (String, Vec<String>) {
        let (tts_main, kw_main) = main.get(key).cloned().unwrap_or((String::new(), vec![]));
        let (tts_derived, kw_derived) = derived.get(key).cloned().unwrap_or((String::new(), vec![]));
        let mut keywords = kw_main;
        for k in kw_derived {
            if !keywords.contains(&k) {
                keywords.push(k);
            }
        }
        let tts = if !tts_main.is_empty() {
            tts_main
        } else if !tts_derived.is_empty() {
            tts_derived
        } else {
            default_name.to_string()
        };
        (tts, keywords)
    }


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
            let codepoint_key = caps[1].to_string();
            let name_en = line.split('#').nth(1).unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join(" ");

            // CLDR lookups (use codepoint string as key)
            // Try direct lookup, then fallback by removing FE0F (VS16) if present
            let lookup_with_fallback = |main: &HashMap<String, (String, Vec<String>)>, derived: &HashMap<String, (String, Vec<String>)>, key: &str, default_name: &str| {
                if main.contains_key(key) || derived.contains_key(key) {
                    merge_cldr(main, derived, key, default_name)
                } else {
                    // Remove FE0F (U+FE0F) from key and try again
                    let fe0f = "FE0F";
                    let parts: Vec<&str> = key.split_whitespace().collect();
                    let filtered: Vec<&str> = parts.iter().filter(|&&cp| cp != fe0f).cloned().collect();
                    let alt_key = filtered.join(" ");
                    if main.contains_key(&alt_key) || derived.contains_key(&alt_key) {
                        merge_cldr(main, derived, &alt_key, default_name)
                    } else {
                        (default_name.to_string(), vec![])
                    }
                }
            };

            let (_name_en_cldr, mut keywords_en) = lookup_with_fallback(&cldr_en_main, &cldr_en_derived, &codepoint_key, &name_en);
            let (mut name_nl, mut keywords_nl) = lookup_with_fallback(&cldr_nl_main, &cldr_nl_derived, &codepoint_key, &name_en);

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
                // Fallback: if keywords_en is empty, try to get from lightest skin tone variant
                if keywords_en.is_empty() {
                    for (i, (mod_cp, _)) in skin_tone_mods.iter().enumerate() {
                        let mut skin_codepoints = codepoints.clone();
                        skin_codepoints.push(u32::from_str_radix(mod_cp, 16).unwrap());
                        let skin_key = skin_codepoints.iter().map(|c| format!("{:X}", c)).collect::<Vec<_>>().join(" ");
                        let (_tts, kws) = merge_cldr(&cldr_en_main, &cldr_en_derived, &skin_key, "");
                        if !kws.is_empty() {
                            keywords_en = kws;
                            break;
                        }
                    }
                }
                if keywords_nl.is_empty() {
                    for (i, (mod_cp, _)) in skin_tone_mods.iter().enumerate() {
                        let mut skin_codepoints = codepoints.clone();
                        skin_codepoints.push(u32::from_str_radix(mod_cp, 16).unwrap());
                        let skin_key = skin_codepoints.iter().map(|c| format!("{:X}", c)).collect::<Vec<_>>().join(" ");
                        let (_tts, kws) = merge_cldr(&cldr_nl_main, &cldr_nl_derived, &skin_key, "");
                        if !kws.is_empty() {
                            keywords_nl = kws;
                            break;
                        }
                    }
                }
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
