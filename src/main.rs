mod battery;
mod dbus;
mod network;
mod udev;
mod util;

use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", battery::get_formatted_time_to()?);

    tokio::spawn(async {
        network::listen_nmcli().await.expect("Unexpected error");
    });

    udev::run().await?;

    Ok(())
}
