# Architectural Decision Record: Flatpak vs. Native Packaging

## Status
Accepted

## Context
We are developing an emoji picker app for Linux desktop environments (GNOME, etc.) with the following goals:
- Instant (<100ms) warm start
- System-wide keyboard shortcut to launch the app
- Ability to keep the app running in the background for fast activation

Flatpak is a popular packaging format for Linux desktop apps, providing sandboxing and cross-distro compatibility. Native packaging (e.g., .deb, RPM, PKGBUILD) offers direct integration with the host system and more control over process lifecycle.

## Decision
We will use **native packaging** (e.g., .deb, RPM, PKGBUILD) for the emoji picker app instead of Flatpak.

## Rationale
- Flatpak is designed for sandboxed, user-initiated desktop apps and does not support persistent background daemons or auto-start at login for desktop apps.
- Flatpak cold start (not running) is ~1s due to sandbox and GTK initialization overhead; warm start is instant only if the app is already running.
- Global hotkey integration and process lifecycle control are limited by Flatpak's sandboxing and security model.
- Native packaging allows:
  - Full control over process lifecycle (systemd user services, autostart .desktop files, custom daemon logic)
  - System-wide keyboard shortcuts and instant activation
  - No sandbox overhead; cold and warm starts can be optimized

## Consequences
- We will not provide a Flatpak package for the emoji picker app.
- The app will be distributed using native packaging formats (e.g., .deb, RPM, PKGBUILD).
- We can implement background process logic, global hotkey handling, and autostart features as needed.
- Users will need to install the app using their distribution's native package manager or provided installer.

## Alternatives Considered
- **Flatpak packaging:** Rejected due to incompatibility with background process and performance goals.
- **AppImage packaging:** Not considered, as it does not solve the background process or hotkey integration issues.

## References
- [Nygard ADR Template](https://github.com/joelparkerhenderson/architecture_decision_record)
- [Flatpak Documentation](https://docs.flatpak.org/en/latest/)
- [Systemd User Services](https://wiki.archlinux.org/title/Systemd/User)

---
This ADR is stored in `docs/adr/` following the Nygard template for architectural decision records.
