use zbus::{proxy, Connection};

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    async fn get_unique_id(&self) -> anyhow::Result<u32>;
}

pub struct NotificationsClient<'a> {
    proxy: NotificationsProxy<'a>,
}

impl<'a> NotificationsClient<'a> {
    pub async fn init() -> anyhow::Result<Self> {
        let connection = Connection::session().await?;
        let proxy = NotificationsProxy::new(&connection).await?;

        Ok(Self { proxy })
    }

    pub async fn get_unique_id(&self) -> anyhow::Result<u32> {
        Ok(self.proxy.get_unique_id().await?)
    }
}
