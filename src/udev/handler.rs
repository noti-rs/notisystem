use notify_rust::Notification;
use udev::EventType;

use crate::dbus;

pub struct Handler<'a> {
    pub dbus_notifications: dbus::notifications::NotificationsClient<'a>,
}

impl<'a> Handler<'a> {
    pub async fn init() -> anyhow::Result<Self> {
        let dbus_notifications = dbus::notifications::NotificationsClient::init().await?;
        Ok(Self { dbus_notifications })
    }

    pub async fn handle_event(&self, event: udev::Event) -> anyhow::Result<()> {
        println!(
            "{}: {} {} (subsystem={}, sysname={}, devtype={})",
            event.sequence_number(),
            event.event_type(),
            event.syspath().to_str().unwrap_or("---"),
            event
                .subsystem()
                .map_or("", |s| { s.to_str().unwrap_or("") }),
            event.sysname().to_str().unwrap_or(""),
            event.devtype().map_or("", |s| { s.to_str().unwrap_or("") })
        );

        let subsystem = event.subsystem().map_or("", |s| s.to_str().unwrap_or(""));

        match subsystem {
            "block" => self.handle_block_subsystem(&event).await?,
            "power_supply" => self.handle_power_supply_subsystem(&event).await?,
            _ => {}
        }

        Ok(())
    }

    pub async fn handle_block_subsystem(&self, event: &udev::Event) -> anyhow::Result<()> {
        let dev_name = self.get_device_name(&event);
        match event.event_type() {
            EventType::Add => {
                Notification::new()
                    .summary("Device Added")
                    .body(&format!("A device was added: {}", dev_name))
                    .icon("device")
                    .show()?;
            }
            EventType::Remove => {
                Notification::new()
                    .summary("Device Removed")
                    .body(&format!("A device was removed: {}", dev_name))
                    .icon("device")
                    .show()?;
            }
            EventType::Change => {
                Notification::new()
                    .summary("Device Changed")
                    .body(&format!("A device was changed: {}", dev_name))
                    .icon("device")
                    .show()?;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn handle_power_supply_subsystem(&self, event: &udev::Event) -> anyhow::Result<()> {
        match event.event_type() {
            EventType::Change => {
                if event.attribute_value("type").unwrap().to_str() == Some("Mains") {
                    let is_charging = match event.attribute_value("online").unwrap().to_str() {
                        Some("1") => true,
                        _ => false,
                    };

                    Notification::new()
                        .id(9999) // WARNING: REPLACE WITH REAL ID!!!
                        .summary(&format!(
                            "Battery status: {}",
                            if is_charging {
                                "charging"
                            } else {
                                "discharging"
                            }
                        ))
                        .icon("battery")
                        .show()?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn get_device_name(&self, event: &udev::Event) -> String {
        event
            .device()
            .property_value("DEVNAME")
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_string()
    }
}
