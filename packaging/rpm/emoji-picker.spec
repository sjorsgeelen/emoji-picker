Name:           emoji-picker
Version:        0.1.0
Release:        1%{?dist}
Summary:        Small GTK4 emoji picker utility

License:        GPL-3.0-or-later
URL:            https://github.com/sjorsgeelen/emoji-picker
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust, cargo, pkgconfig, gtk4-devel, libadwaita-devel, glib2-devel
Requires:       gtk4, libadwaita, glib2

# Binary builds are architecture specific
BuildArch:      %{_arch}

%description
Emoji picker: a small GTK4 application for selecting emojis.

%prep
%autosetup -n %{name}-%{version}

%build
# build with cargo in release mode; allow target-dir to be within buildroot
export CARGO_HOME="%{_topdir}/.cargo"
cargo build --locked --release

%install
rm -rf %{buildroot}
install -d %{buildroot}%{_bindir}
install -m 0755 ../../.build/flatpak/emoji-picker %{buildroot}%{_bindir}/emoji-picker

# install desktop entry and appdata (packaging/ directory included in source)
install -d %{buildroot}%{_datadir}/applications
install -m 0644 ../../.build/flatpak/emoji-picker.desktop %{buildroot}%{_datadir}/applications/emoji-picker.desktop

install -d %{buildroot}%{_datadir}/metainfo
install -m 0644 ../../.build/flatpak/nl.dibitat.emoji_picker.metainfo.xml %{buildroot}%{_datadir}/metainfo/nl.dibitat.emoji_picker.metainfo.xml

# install main svg icon
install -d %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
install -m 0644 packaging/emoji-picker.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/nl.dibitat.emoji_picker.svg

%files
%license LICENSE
%doc README.md
%{_bindir}/emoji-picker
%{_datadir}/applications/emoji-picker.desktop
%{_datadir}/metainfo/*
%{_datadir}/icons/*

%changelog
* Thu Jan 01 2026 Packager <packager@example.com> - 0.1.0-1
- Initial RPM packaging
