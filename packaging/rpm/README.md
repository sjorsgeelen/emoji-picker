RPM packaging for `emoji-picker`

Overview
- This folder contains a simple RPM spec and a helper script to build RPMs locally without installing system-wide build dependencies.

Files
- `emoji-picker.spec`: RPM spec file. Adjust `BuildRequires` as needed for your distro.
- `build_rpm.sh`: helper script that creates a local `rpmbuild` tree and builds the package.

How it works
1. The script creates a source tarball of the repository (excluding `target` and the local rpmbuild tree).
2. It places the tarball in a local `rpmbuild/SOURCES` directory and runs `rpmbuild -ba` with a local `_topdir`.

Requirements
- `rpmbuild` and system-level build dependencies (see `BuildRequires` in the spec): `rust`, `cargo`, `pkgconfig`, `gtk4-devel`, `libadwaita-devel`, `glib2-devel`, etc.

Usage
Run from `packaging/rpm`:

```bash
./build_rpm.sh 0.1.0
```

CI
- For CI, run builds inside a clean container (Fedora/EL) and use `rpmbuild` with the same `_topdir` approach, or use distro-specific build services (OBS, Copr, Koji).
