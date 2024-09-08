mod handler;

use futures_util::TryStreamExt;
use std::convert::TryInto;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

pub async fn run() -> anyhow::Result<()> {
    let handler = crate::udev::handler::UdevEventHandler::init().await?;

    let builder = MonitorBuilder::new()?
        .match_subsystem_devtype("usb", "usb_device")?
        .match_subsystem_devtype("block", "disk")?
        .match_subsystem_devtype("power_supply", "power_supply")?;

    let mut monitor: AsyncMonitorSocket = builder.listen()?.try_into()?;

    while let Some(event) = monitor.try_next().await? {
        handler.handle_event(event).await?;
    }

    Ok(())
}
