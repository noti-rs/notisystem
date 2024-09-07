use notify_rust::Notification;
use tokio_udev::EventType;

pub fn handle_udev_event(event: tokio_udev::Event) -> anyhow::Result<()> {
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
        "block" => handle_block_subsystem(&event)?,
        "power_supply" => handle_power_supply_subsystem(&event)?,
        "usb" => handle_usb_subsystem(&event)?,
        _ => {}
    }

    Ok(())
}

fn handle_block_subsystem(event: &tokio_udev::Event) -> anyhow::Result<()> {
    let dev_name = get_device_name(&event);

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

fn handle_power_supply_subsystem(event: &tokio_udev::Event) -> anyhow::Result<()> {
    match event.event_type() {
        EventType::Change => {
            if event.attribute_value("type").unwrap().to_str() == Some("Mains") {
                let is_charging = match event.attribute_value("online").unwrap().to_str() {
                    Some("1") => true,
                    _ => false,
                };

                Notification::new()
                    .id(9999) // WARNING: REPLACE WITH ACTUAL ID!!!
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

fn handle_usb_subsystem(event: &tokio_udev::Event) -> anyhow::Result<()> {
    dbg!(&event);
    let dev_name = get_readable_usb_device_name(&event.device()).unwrap_or("Unknown".to_string());

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
        _ => {}
    }

    Ok(())
}

fn get_readable_usb_device_name(device: &tokio_udev::Device) -> Option<String> {
    let model = device
        .property_value("ID_MODEL")
        .map(|v| v.to_string_lossy().into_owned());
    let vendor = device
        .property_value("ID_VENDOR")
        .map(|v| v.to_string_lossy().into_owned());

    match (vendor, model) {
        (Some(vendor), Some(model)) => Some(format!("{} {}", vendor, model)),
        (Some(vendor), None) => Some(vendor),
        (None, Some(model)) => Some(model),
        _ => None, // If neither are available
    }
}

fn get_device_name(event: &tokio_udev::Event) -> String {
    event
        .device()
        .property_value("DEVNAME")
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string()
}
