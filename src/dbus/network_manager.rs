use futures_util::stream::StreamExt;
use zbus::proxy;

#[proxy(interface = "org.freedesktop.NetworkManager", assume_defaults = true)]
trait NetworkManager {
    #[zbus(signal)]
    fn state_changed(&self, state: u32) -> zbus::Result<()>;
}

// NOTE: not working at the moment
pub async fn listen() -> anyhow::Result<()> {
    let conn = zbus::Connection::system().await?;
    let proxy = NetworkManagerProxy::new(&conn).await?;
    let mut stream = proxy.receive_state_changed().await?;

    println!("Listening NetworkManager for signals...");
    while let Some(msg) = stream.next().await {
        dbg!(&msg);

        let args: StateChangedArgs = msg.args().expect("Error parsing message");

        println!("StateChanged received: state={}", args.state);
    }

    anyhow::bail!("Stream ended unexpectedly");
}
