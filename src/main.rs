mod dbus;
mod udev;

use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::spawn(async {
        dbus::network_manager::listen().await.expect("pizda");
    });

    udev::run().await?;

    Ok(())
}
