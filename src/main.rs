use battery::{Manager, State};
use clap::Parser;
use std::thread;
use std::time::Duration;

/// Monitors battery charge and beeps when full so you remember to unplug.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How often to check battery level, in seconds
    #[arg(short, long, default_value_t = 30)]
    interval: u64,

    /// Charge percentage to treat as full (0–100)
    #[arg(short, long, default_value_t = 99)]
    threshold: u8,

    /// Enable water-drinking reminders
    #[arg(long)]
    remind_water: bool,

    /// Minutes between water reminders
    #[arg(long, default_value_t = 60)]
    water_interval: u64,

    /// Enable reminders to get up and move after long sitting
    #[arg(long)]
    remind_movement: bool,

    /// Minutes between movement reminders
    #[arg(long, default_value_t = 45)]
    movement_interval: u64,
}

fn main() {
    let args = Args::parse();
    let threshold = (args.threshold as f32) / 100.0;

    let manager = Manager::new().expect("Failed to create battery manager");

    println!("Battery monitor started.");
    println!(
        "Checking every {}s — will notify at {}%+\n",
        args.interval, args.threshold
    );

    if args.remind_water {
        println!("Water reminders enabled — every {} min", args.water_interval);
        let interval = args.water_interval;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(interval * 60));
            notify_reminder("Hydration Reminder", "Time to drink some water!");
        });
    }

    if args.remind_movement {
        println!(
            "Movement reminders enabled — every {} min",
            args.movement_interval
        );
        let interval = args.movement_interval;
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(interval * 60));
            notify_reminder("Time to Move", "You've been sitting a while — stand up and stretch!");
        });
    }

    loop {
        match check_battery(&manager, threshold) {
            Ok(true) => {
                println!("\n*** BATTERY FULL — you can unplug the charger! ***");
                notify();
                // Re-check every 60s and keep notifying until unplugged
                thread::sleep(Duration::from_secs(60));
            }
            Ok(false) => {}
            Err(e) => eprintln!("Battery check error: {}", e),
        }
        thread::sleep(Duration::from_secs(args.interval));
    }
}

fn check_battery(manager: &Manager, threshold: f32) -> Result<bool, battery::Error> {
    let mut batteries = manager.batteries()?;
    if let Some(Ok(battery)) = batteries.next() {
        let charge = battery.state_of_charge().value;
        let state = battery.state();
        print!(
            "\rCharge: {:.1}%  State: {:?}        ",
            charge * 100.0,
            state
        );
        let _ = std::io::Write::flush(&mut std::io::stdout());
        return Ok(matches!(state, State::Full) || charge >= threshold);
    }
    Ok(false)
}

fn notify() {
    #[cfg(windows)]
    {
        show_balloon("Battery Full", "Your battery is fully charged - unplug the charger!");

        for _ in 0..5 {
            unsafe {
                winapi::um::utilapiset::Beep(1000, 400);
            }
            thread::sleep(Duration::from_millis(600));
        }
    }

    #[cfg(not(windows))]
    {
        use std::io::Write;
        for _ in 0..5 {
            print!("\x07");
            let _ = std::io::stdout().flush();
            thread::sleep(Duration::from_millis(600));
        }
    }
}

fn notify_reminder(title: &str, message: &str) {
    #[cfg(windows)]
    {
        show_balloon(title, message);
        unsafe {
            winapi::um::utilapiset::Beep(800, 300);
        }
    }

    #[cfg(not(windows))]
    {
        use std::io::Write;
        println!("\n*** {}: {} ***", title, message);
        print!("\x07");
        let _ = std::io::stdout().flush();
    }
}

#[cfg(windows)]
fn show_balloon(title: &str, message: &str) {
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
$balloon = New-Object System.Windows.Forms.NotifyIcon
$balloon.Icon = [System.Drawing.SystemIcons]::Information
$balloon.BalloonTipIcon = [System.Windows.Forms.ToolTipIcon]::Info
$balloon.BalloonTipTitle = '{}'
$balloon.BalloonTipText = '{}'
$balloon.Visible = $true
$balloon.ShowBalloonTip(5000)
Start-Sleep -Seconds 5
$balloon.Dispose()
"#,
        title.replace('\'', "''"),
        message.replace('\'', "''")
    );

    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
        .spawn();
}
