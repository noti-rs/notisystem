use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    async fn get_unique_id(&self) -> anyhow::Result<u32>;
}

pub async fn get_unique_id() -> anyhow::Result<u32> {
    let conn = zbus::Connection::session().await?;
    let proxy = NotificationsProxy::new(&conn).await?;

    Ok(proxy.get_unique_id().await?)
}
