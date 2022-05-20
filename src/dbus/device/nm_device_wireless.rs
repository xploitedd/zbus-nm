use std::collections::HashMap;

use zbus::dbus_proxy;
use zvariant::{OwnedObjectPath, Value, ObjectPath};

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
)]
pub trait NetworkManagerWirelessDevice {
    fn get_all_access_points(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn request_scan(&self, options: HashMap<&str, &Value<'_>>) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn access_point_added(&self, access_point: ObjectPath<'_>) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn access_point_removed(&self, access_point: ObjectPath<'_>) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn bitrate(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn active_access_point(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn wireless_capabilities(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn last_scan(&self) -> zbus::Result<i64>;
}