use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use zbus::dbus_proxy;
use zvariant::{OwnedObjectPath, Value, Type, OwnedValue};

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
pub trait NetworkManagerSettings {
    fn list_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn get_connection_by_uuid(&self, uuid: &str) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn add_connection(&self, connection: HashMap<&str, HashMap<&str, &Value<'_>>>) -> zbus::Result<OwnedObjectPath>;

    fn add_connection_unsaved(&self, connection: HashMap<&str, HashMap<&str, &Value<'_>>>) -> zbus::Result<OwnedObjectPath>;

    fn add_connection_2(
        &self,
        settings: HashMap<&str, HashMap<&str, &Value<'_>>>,
        flags: u32
    ) -> zbus::Result<AddConnection2Result>;

    fn load_connections(&self, filenames: Vec<String>) -> zbus::Result<LoadConnectionsResult>;

    fn reload_connections(&self) -> zbus::Result<bool>;

    fn save_hostname(&self, hostname: &str) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn new_connection(&self, connection: OwnedObjectPath) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn connection_removed(&self, connection: OwnedObjectPath) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn hostname(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn can_modify(&self) -> zbus::Result<bool>;
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AddConnection2Result {
    path: OwnedObjectPath,
    result: HashMap<String, OwnedValue>
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LoadConnectionsResult {
    status: bool,
    failures: Vec<String>
}

#[cfg(test)]
mod tests {
    use zbus::Connection;

    use super::NetworkManagerSettingsProxy;


    #[tokio::test]
    async fn test_list_connections() {
        let proxy = get_proxy()
            .await;

        proxy.list_connections()
            .await
            .unwrap();
    }

    async fn get_proxy() -> NetworkManagerSettingsProxy<'static> {
        let conn = Connection::system()
            .await
            .unwrap();

        NetworkManagerSettingsProxy::new(&conn)
            .await
            .unwrap()
    }

}