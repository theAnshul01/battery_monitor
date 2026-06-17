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
