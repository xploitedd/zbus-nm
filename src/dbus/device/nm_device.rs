use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use zbus::dbus_proxy;
use zvariant::{Value, OwnedValue, Type, OwnedObjectPath};

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device",
)]
pub trait NetworkManagerDevice {
    fn reapply(
        &self,
        connection: HashMap<&str, HashMap<&str, &Value<'_>>>,
        version_id: u64,
        flags: u32
    ) -> zbus::Result<()>;

    fn get_applied_connection(&self, flags: u32) -> zbus::Result<GetAppliedConnectionResponse>;

    fn disconnect(&self) -> zbus::Result<()>;

    fn delete(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn udi(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn path(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn interface(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn ip_interface(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn driver(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn driver_version(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn firmware_version(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn ip4_address(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn state_reason(&self) -> zbus::Result<(u32, u32)>;

    #[dbus_proxy(property)]
    fn active_connection(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn ip4_config(&self) ->  zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn dhcp4_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn ip6_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn dhcp6_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn managed(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn set_managed(&self, managed: bool) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn autoconnect(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn set_autoconnect(&self, autoconnect: bool) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn firmware_missing(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn nm_plugin_missing(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn device_type(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn available_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn physical_port_id(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn mtu(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn metered(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn lldp_neighbors(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    #[dbus_proxy(property)]
    fn readable(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn ip4_connectivity(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn ip6_connectivity(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct GetAppliedConnectionResponse {
    connection: HashMap<String, HashMap<String, OwnedValue>>,
    version_id: u64
}