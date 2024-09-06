mod handler;
mod listener;

use listener::poll::poll;
use std::io;

pub async fn run() -> io::Result<()> {
    let socket = udev::MonitorBuilder::new()?
        // .match_subsystem_devtype("usb", "usb_device")?
        .match_subsystem_devtype("block", "disk")?
        .match_subsystem_devtype("power_supply", "power_supply")?
        .listen()?;

    poll(socket).await
}
