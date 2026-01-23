#!/usr/bin/env bash
set -euo pipefail

# Simple local RPM builder that creates a local rpmbuild tree and builds the package.
# Usage: ./build_rpm.sh [version]

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TOPDIR=$(cd "$SCRIPT_DIR/.." && pwd)
RPMBUILD_DIR="$SCRIPT_DIR/rpmbuild"

VERSION=${1:-0.1.0}
NAME=emoji-picker
TARBALL="$SCRIPT_DIR/${NAME}-${VERSION}.tar.gz"

echo "Preparing rpmbuild tree at $RPMBUILD_DIR"
rm -rf "$RPMBUILD_DIR"
mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS,tmp}

echo "Creating source tarball $TARBALL"
# Create a reproducible tarball from repository root excluding target and .git
pushd "$TOPDIR" >/dev/null
tar --exclude-vcs --exclude=target --exclude=packaging/rpm/rpmbuild -czf "$TARBALL" .
popd >/dev/null

echo "Copying to SOURCES"
cp "$TARBALL" "$RPMBUILD_DIR/SOURCES/"

echo "Building RPM with rpmbuild (local topdir)"
rpmbuild -ba "$SCRIPT_DIR/emoji-picker.spec" --define "_topdir $RPMBUILD_DIR" --define "_sourcedir $RPMBUILD_DIR/SOURCES"

echo "Built RPMs are under $RPMBUILD_DIR/RPMS"
