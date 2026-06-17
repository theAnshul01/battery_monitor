# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - 2026-06-17

### Added
- System tray balloon tip notification on Windows alongside the beep — appears in the bottom-right corner when battery is full

### Changed
- Replaced Windows toast API (required app registration) with `System.Windows.Forms.NotifyIcon` balloon tip which works on all Windows machines without special permissions

---

## [0.1.0] - 2026-06-17

### Added
- Initial release
- Real-time battery charge monitoring with configurable poll interval (`--interval`)
- Configurable full-charge threshold (`--threshold`)
- Audible beep notification on Windows via Win32 `Beep()` API
- BEL character fallback notification for Linux and macOS terminals
- Repeating alert every 60 seconds until charger is unplugged
- Live charge percentage and state display in terminal
