mod listener;

use listener::poll::poll;
use std::io;

fn main() -> io::Result<()> {
    let socket = udev::MonitorBuilder::new()?
        // .match_subsystem_devtype("usb", "usb_device")?
        .match_subsystem_devtype("block", "disk")?
        .listen()?;

    poll(socket)
}
