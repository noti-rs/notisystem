extern crate libc;
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

    use crate::udev::handler::Handler;

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

    pub async fn poll(socket: udev::MonitorSocket) -> io::Result<()> {
        println!("Listening udev socket...");
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

            let handler = Handler::init().await.unwrap();

            let _ = handler.handle_event(event).await;
        }
    }
}
