mod dbus;
mod udev;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    crate::udev::run().await
}
