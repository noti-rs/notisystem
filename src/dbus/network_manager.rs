use futures_util::stream::StreamExt;
use notify_rust::Notification;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use zbus::proxy;

#[proxy(interface = "org.freedesktop.NetworkManager", assume_defaults = true)]
trait NetworkManager {
    #[zbus(signal)]
    fn state_changed(&self, state: u32) -> zbus::Result<()>;

    #[zbus(signal)]
    fn check_permissions(&self) -> zbus::Result<()>;

    #[zbus(name = "state")]
    fn state(&self) -> zbus::Result<u32>;
}

pub async fn listen() -> anyhow::Result<()> {
    let conn = zbus::Connection::system().await?;
    let proxy = NetworkManagerProxy::new(&conn)
        .await
        .expect("Connection failed");

    let mut stream = proxy.receive_state_changed().await?;

    println!("Listening NetworkManager for signals...");
    while let Some(msg) = stream.next().await {
        dbg!(&msg);

        let args: StateChangedArgs = msg.args().expect("Error parsing message");

        println!("StateChanged received: state={}", args.state);
    }

    anyhow::bail!("Stream ended unexpectedly");
}

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
                    if output.contains("wlan0: disconnected") {
                        send_notification("Network Status", "Disconnected from the internet");
                    } else if output.contains("wlan0: connected") {
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
        .id(9998) // WARNING: REPLACE WITH ACTUAL ID!!!
        .summary(summary)
        .body(body)
        .show()
        .unwrap();
}
