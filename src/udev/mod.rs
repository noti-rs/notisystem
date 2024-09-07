mod handler;

use futures_util::{future::ready, stream::StreamExt};
use std::convert::TryInto;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

pub async fn run() -> anyhow::Result<()> {
    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem_devtype("block", "disk")
        .expect("Failed to add filter for disk devices")
        .match_subsystem_devtype("power_supply", "power_supply")
        .expect("Failed to add filter for power supply devices")
        .match_subsystem_devtype("usb", "usb_device")
        .expect("Failed to add filter for USB devices");

    let monitor: AsyncMonitorSocket = builder
        .listen()
        .expect("Couldn't create MonitorSocket")
        .try_into()
        .expect("Couldn't create AsyncMonitorSocket");
    monitor
        .for_each(|event| {
            if let Ok(event) = event {
                crate::udev::handler::handle_udev_event(event).ok();
            }
            ready(())
        })
        .await;

    Ok(())
}
