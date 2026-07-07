# battery_monitor

A lightweight command-line tool written in Rust that monitors your laptop's battery and notifies you with a beep when it's fully charged — so you remember to unplug.

---

## Why

Keeping your laptop plugged in at 100% for extended periods degrades battery health over time. This tool runs quietly in the background and alerts you the moment the battery is full, so you can unplug promptly without having to keep checking manually.

---

## Features

- Monitors battery charge level in real time
- Beeps audibly when battery reaches the configured threshold
- Keeps notifying every 60 seconds until you unplug
- Configurable poll interval and charge threshold via CLI flags
- Optional water-drinking reminders on a configurable interval
- Optional movement reminders to break up long sitting sessions
- Works on Windows, Linux, and macOS
- Tiny binary, near-zero CPU and memory usage

---

## Installation

### Option 1 — Via Cargo (requires Rust)

```sh
cargo install battery_monitor
```

This compiles and installs the binary to `~/.cargo/bin/`, making `battery_monitor` available system-wide.

### Option 2 — Pre-built binary (no Rust required)

Download the binary for your platform from the [Releases](https://github.com/AnshulOP/battery_monitor/releases) page:

| Platform | File |
|----------|------|
| Windows  | `battery_monitor-x86_64-pc-windows-msvc.exe` |
| Linux    | `battery_monitor-x86_64-unknown-linux-gnu` |
| macOS    | `battery_monitor-x86_64-apple-darwin` |

Then move the binary to a directory on your `PATH`:

**Windows (PowerShell):**
```powershell
Move-Item battery_monitor-x86_64-pc-windows-msvc.exe "$env:USERPROFILE\.cargo\bin\battery_monitor.exe"
```

**Linux / macOS:**
```sh
chmod +x battery_monitor-x86_64-*
sudo mv battery_monitor-x86_64-* /usr/local/bin/battery_monitor
```

---

## Usage

Plug in your charger, then run:

```sh
battery_monitor
```

That's it. It will print the current charge every 30 seconds and beep loudly when the battery is full.

```
Battery monitor started.
Checking every 30s — will notify at 99%+

Charge: 87.3%  State: Charging
```

### Options

```
Usage: battery_monitor [OPTIONS]

Options:
  -i, --interval <INTERVAL>              How often to check battery level, in seconds [default: 30]
  -t, --threshold <THRESHOLD>            Charge percentage to treat as full (0–100) [default: 99]
      --remind-water                     Enable water-drinking reminders
      --water-interval <WATER_INTERVAL>  Minutes between water reminders [default: 60]
      --remind-movement                  Enable reminders to get up and move after long sitting
      --movement-interval <MOVEMENT_INTERVAL>  Minutes between movement reminders [default: 45]
  -h, --help                             Print help
  -V, --version                          Print version
```

### Examples

Check every 60 seconds, notify at 95%:
```sh
battery_monitor --interval 60 --threshold 95
```

Aggressive polling, notify only at true 100%:
```sh
battery_monitor -i 10 -t 100
```

Add hydration and movement reminders on top of the default battery monitoring:
```sh
battery_monitor --remind-water --remind-movement
```

Custom reminder intervals — water every 45 minutes, movement every 30 minutes:
```sh
battery_monitor --remind-water --water-interval 45 --remind-movement --movement-interval 30
```

Both reminders are opt-in and disabled unless their `--remind-*` flag is passed. They run independently of the battery check and show a balloon notification (with a short beep on Windows) when due.

---

## Platform Notes

| Platform | Notification method |
|----------|---------------------|
| Windows  | `Beep()` via Win32 API — works even when volume is muted |
| Linux    | BEL character (`\x07`) sent to terminal |
| macOS    | BEL character (`\x07`) sent to terminal |

> On Linux/macOS, whether the BEL character produces an audible beep depends on your terminal emulator's settings.

---

## Running as a Background Service

If you want the monitor to start automatically when you plug in the charger, you can register it as a system task.

### Windows — Task Scheduler

1. Open **Task Scheduler** → Create Task
2. **Trigger:** On event → Log: `System`, Source: `Microsoft-Windows-Kernel-Power`, Event ID: `105` (AC power connected)
3. **Action:** Start a program → path to `battery_monitor.exe`
4. **Settings:** Allow task to be run on demand, stop if it runs longer than 4 hours

### Linux — udev rule

Create `/etc/udev/rules.d/99-battery-monitor.rules`:
```
SUBSYSTEM=="power_supply", ATTR{online}=="1", RUN+="/usr/local/bin/battery_monitor"
```

### macOS — launchd (on AC connect via pmset)

Use a `launchd` plist triggered by `IOPSNotificationCreateRunLoopSource` or wrap the binary in a login item that runs continuously.

---

## Building from Source

```sh
git clone https://github.com/AnshulOP/battery_monitor
cd battery_monitor
cargo build --release
```

Binary will be at `target/release/battery_monitor` (or `.exe` on Windows).

---

## Future Scope

- **Desktop notifications** — Show a native OS popup (toast on Windows, libnotify on Linux, osascript on macOS) alongside the beep, so you get notified even when the terminal is minimized
- **Reminder notifications on Linux/macOS** — Water and movement reminders currently fall back to a terminal beep on non-Windows platforms; native `libnotify`/`osascript` popups would match the Windows balloon experience
- **System tray icon** — Run silently in the system tray with a live battery percentage, removing the need for an open terminal window entirely
- **Auto-start on AC connect** — Built-in `--install` flag that registers the Task Scheduler / udev / launchd entry automatically, no manual setup required
- **Charge limiter mode** — A `--max` flag that also notifies when charging *starts* if battery is already above a set level (e.g. 80%), helping users who follow partial-charge habits for longevity
- **Custom sounds** — Play a user-supplied audio file instead of a system beep, using a cross-platform audio crate like `rodio`
- **Battery health report** — A `--status` flag that prints cycle count, design capacity vs current capacity, and estimated health percentage
- **Multiple battery support** — Laptops with dual batteries (some ThinkPads) currently only monitor the first battery; full support for all detected batteries
- **Low battery alert** — Complementary mode (`--low <percent>`) that alerts when battery drops below a threshold, useful when running on battery

---

## License

MIT — see [LICENSE](LICENSE) for details.
