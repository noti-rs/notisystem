use notify_rust::Notification;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

// NOTE: TEMPORARY SOLUTION, will be refactored as soon as i understand how to listen the NetworkManager DBus interface for signals
pub async fn listen_nmcli() -> anyhow::Result<()> {
    let nmcli_monitor = Command::new("nmcli")
        .arg("monitor")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start nmcli monitor");

    println!("Listening nmcli monitor...");

    if let Some(stdout) = nmcli_monitor.stdout {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(output) => {
                    if output.contains("disconnected") {
                        send_notification("Network Status", "Disconnected from the internet");
                    } else if output.contains("connected") {
                        send_notification("Network Status", "Connected to the internet");
                    }
                }
                Err(e) => eprintln!("Error reading nmcli output: {}", e),
            }
        }
    } else {
        eprintln!("Failed to capture nmcli stdout.");
    }
    Ok(())
}

fn send_notification(summary: &str, body: &str) {
    Notification::new()
        .id(9998) // WARNING: REPLACE WITH AN ACTUAL ID!!!
        .summary(summary)
        .body(body)
        .show()
        .unwrap();
}
