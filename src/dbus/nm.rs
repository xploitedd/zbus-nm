use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use zbus::dbus_proxy;
use zvariant::{OwnedObjectPath, ObjectPath, OwnedValue, Value, Type};

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub trait NetworkManager {
    fn reload(&self, flags: u32) -> zbus::Result<()>;
    
    fn get_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn get_all_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<OwnedObjectPath>;

    fn activate_connection(
        &self,
        device: &ObjectPath<'_>,
        specific_object: &ObjectPath<'_>
    ) -> zbus::Result<OwnedObjectPath>;

    fn add_and_activate_connection(
        &self,
        connection: HashMap<&str, HashMap<&str, &Value<'_>>>,
        device: &ObjectPath<'_>,
        specific_object: &ObjectPath<'_>
    ) -> zbus::Result<AddActivateResponse>;

    fn deactivate_connection(&self, active_connection: &ObjectPath<'_>) -> zbus::Result<()>;

    fn sleep(&self, sleep: bool) -> zbus::Result<()>;

    fn enable(&self, enable: bool) -> zbus::Result<()>;

    fn get_permissions(&self) -> zbus::Result<HashMap<String, String>>;

    fn set_logging(&self, level: &str, domains: &str) -> zbus::Result<()>;

    fn get_logging(&self) -> zbus::Result<GetLoggingResponse>;

    fn check_connectivity(&self) -> zbus::Result<u32>;

    fn state(&self) -> zbus::Result<u32>;

    fn checkpoint_create(
        &self,
        devices: Vec<&ObjectPath<'_>>,
        rollback_timeout: u32,
        flags: u32
    ) -> zbus::Result<OwnedObjectPath>;

    fn checkpoint_destroy(&self, checkpoint: &ObjectPath<'_>) -> zbus::Result<()>;

    fn checkpoint_adjust_rollback_timeout(
        &self,
        checkpoint: &ObjectPath<'_>,
        add_timeout: u32
    ) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn check_permissions(&self) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn state_changed(&self, state: u32) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn device_added(&self, device_path: ObjectPath<'_>) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn device_removed(&self, device_path: ObjectPath<'_>) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn checkpoints(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn networking_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn set_wireless_enabled(&self, enabled: bool) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn wireless_hardware_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn wwan_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn set_wwan_enabled(&self, enabled: bool) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn wwan_hardware_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn active_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn primary_connection(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn primary_connection_type(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn metered(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn activating_connection(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn startup(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::Result<Vec<u32>>;

    #[dbus_proxy(property)]
    fn connectivity(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn connectivity_check_available(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn connectivity_check_enabled(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn set_connectivity_check_enabled(&self, enabled: bool) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn connectivity_check_uri(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn global_dns_configuration(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    #[dbus_proxy(property)]
    fn set_global_dns_configuration(&self, configuration: HashMap<&str, Value<'_>>) -> zbus::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AddActivateResponse {
    pub path: OwnedObjectPath,
    pub active_connection: OwnedObjectPath
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct GetLoggingResponse {
    pub level: String,
    pub domains: String
}

#[cfg(test)]
mod tests {
    use zbus::Connection;

    use super::*;

    #[tokio::test]
    async fn test_get_devices() {
        let nm = get_proxy()
            .await;

        let devices = nm.get_devices()
            .await
            .unwrap();
            
        assert!(devices.len() > 0);
    }

    #[tokio::test]
    async fn test_get_all_devices() {
        let nm = get_proxy()
            .await;

        let devices = nm.get_devices()
            .await
            .unwrap();

        let all_devices = nm.get_all_devices()
            .await
            .unwrap();

        assert!(all_devices.len() > 0);
        assert!(all_devices.len() >= devices.len())
    }

    async fn get_proxy() -> NetworkManagerProxy<'static> {
        let conn = Connection::system()
            .await
            .unwrap();

        NetworkManagerProxy::new(&conn)
            .await
            .unwrap()
    }
}