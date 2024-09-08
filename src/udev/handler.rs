use notify_rust::Notification;
use tokio_udev::EventType;

pub struct UdevEventHandler {
    // ...
}

impl UdevEventHandler {
    pub async fn init() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub async fn handle_event(&self, event: tokio_udev::Event) -> anyhow::Result<()> {
        println!(
            "{}: {}\n{}\nsubsystem={},\nsysname={},\ndevtype={}\n\n",
            event.sequence_number(),
            event.event_type(),
            event.syspath().to_str().unwrap_or("---"),
            event
                .subsystem()
                .map_or("", |s| { s.to_str().unwrap_or("") }),
            event.sysname().to_str().unwrap_or(""),
            event.devtype().map_or("", |s| { s.to_str().unwrap_or("") })
        );

        let devtype = event
            .devtype()
            .and_then(|devtype| devtype.to_str())
            .unwrap_or("unknown");

        match devtype {
            "disk" => self.handle_disk_devtype(&event)?, // ┓
            // ┣━ NOTE: these overlap
            "usb_device" => self.handle_usb_devtype(&event)?, // ┛
            "power_supply" => self.handle_power_supply_devtype(&event).await?,
            "unknown" => {
                println!("Warning: unknown device type on event: ");
                dbg!(&event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_disk_devtype(&self, event: &tokio_udev::Event) -> anyhow::Result<()> {
        let dev_name = self.get_usb_device_name(&event);

        let mut noti = Notification::new();
        match event.event_type() {
            EventType::Add => {
                noti.summary("Device Added")
                    .body(&format!("A device was added: {}", dev_name))
                    .icon("device"); // TODO: replace with an actual icon
            }
            EventType::Remove => {
                noti.summary("Device Removed")
                    .body(&format!("A device was removed: {}", dev_name))
                    .icon("device"); // TODO: replace with an actual icon
            }
            EventType::Change => {
                noti.summary("Device Changed")
                    .body(&format!("A device was changed: {}", dev_name))
                    .icon("device"); // TODO: replace with an actual icon
            }
            _ => {}
        }

        noti.show()?;

        Ok(())
    }

    async fn handle_power_supply_devtype(&self, event: &tokio_udev::Event) -> anyhow::Result<()> {
        if event.event_type() == EventType::Change
            && event.attribute_value("type").unwrap().to_str() == Some("Mains")
        {
            let is_charging =
                matches!(event.attribute_value("online").unwrap().to_str(), Some("1"));

            #[rustfmt::skip]
            let noti_summary = format!( "Battery status: {}", if is_charging { "charging" } else { "discharging" });

            Notification::new()
                .id(9999) // WARNING: REPLACE WITH AN ACTUAL ID!!!
                .summary(&noti_summary)
                .icon("battery") // TODO: replace with an actual icon
                .show()?;
        }

        Ok(())
    }

    fn handle_usb_devtype(&self, event: &tokio_udev::Event) -> anyhow::Result<()> {
        dbg!(&event);
        let dev_name = self.get_usb_device_name(&event.device());

        match event.event_type() {
            EventType::Add => {
                Notification::new()
                    .summary("Device Added")
                    .body(&format!("A device was added: {}", dev_name))
                    .icon("device") // TODO: replace with an actual icon
                    .show()?;
            }
            EventType::Remove => {
                Notification::new()
                    .summary("Device Removed")
                    .body(&format!("A device was removed: {}", dev_name))
                    .icon("device") // TODO: replace with an actual icon
                    .show()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn get_usb_device_name(&self, device: &tokio_udev::Device) -> String {
        let model = device
            .property_value("ID_MODEL")
            .map(|v| v.to_string_lossy().into_owned());
        let vendor = device
            .property_value("ID_VENDOR")
            .map(|v| v.to_string_lossy().into_owned());

        let name = match (vendor, model) {
            (Some(vendor), Some(model)) => Some(format!("{} {}", vendor, model)),
            (Some(vendor), None) => Some(vendor),
            (None, Some(model)) => Some(model),
            _ => None,
        };

        match name {
            Some(name) => name,
            None => String::from("Unknown"),
        }
    }
}
