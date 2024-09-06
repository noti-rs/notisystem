use notify_rust::Notification;
use udev::EventType;

extern crate libc;
#[cfg(feature = "mio06")]
extern crate mio06;
#[cfg(feature = "mio07")]
extern crate mio07;
#[cfg(feature = "mio08")]
extern crate mio08;
#[cfg(feature = "mio10")]
extern crate mio10;
extern crate udev;

#[cfg(not(any(
    feature = "mio06",
    feature = "mio07",
    feature = "mio08",
    feature = "mio10"
)))]
pub mod poll {
    use std::io;
    use std::ptr;
    use std::thread;
    use std::time::Duration;

    use std::os::unix::io::AsRawFd;

    use libc::{c_int, c_short, c_ulong, c_void};

    #[repr(C)]
    #[allow(non_camel_case_types)]
    struct pollfd {
        fd: c_int,
        events: c_short,
        revents: c_short,
    }

    #[repr(C)]
    #[allow(non_camel_case_types)]
    struct sigset_t {
        __private: c_void,
    }

    #[allow(non_camel_case_types)]
    type nfds_t = c_ulong;

    const POLLIN: c_short = 0x0001;

    extern "C" {
        fn ppoll(
            fds: *mut pollfd,
            nfds: nfds_t,
            timeout_ts: *mut libc::timespec,
            sigmask: *const sigset_t,
        ) -> c_int;
    }

    pub fn poll(socket: udev::MonitorSocket) -> io::Result<()> {
        println!("Use syspoll");
        let mut fds = vec![pollfd {
            fd: socket.as_raw_fd(),
            events: POLLIN,
            revents: 0,
        }];

        loop {
            let result = unsafe {
                ppoll(
                    (&mut fds[..]).as_mut_ptr(),
                    fds.len() as nfds_t,
                    ptr::null_mut(),
                    ptr::null(),
                )
            };

            if result < 0 {
                return Err(io::Error::last_os_error());
            }

            let event = match socket.iter().next() {
                Some(evt) => evt,
                None => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            };

            let _ = super::handle_event(event);
        }
    }
}

fn handle_event(event: udev::Event) -> anyhow::Result<()> {
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

    match event.event_type() {
        EventType::Add => {
            let dev_name = event
                .device()
                .property_value("DEVNAME")
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string();

            Notification::new()
                .summary("Device Added")
                .body(&format!("A device was added: {}", dev_name))
                .icon("device")
                .show()?;
        }
        EventType::Remove => {
            let dev_name = event
                .device()
                .property_value("DEVNAME")
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string();
            Notification::new()
                .summary("Device Removed")
                .body(&format!("A device was removed: {}", dev_name))
                .icon("device")
                .show()?;
        }
        EventType::Change => {
            let dev_name = event
                .device()
                .property_value("DEVNAME")
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string();
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
