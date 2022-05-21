use std::collections::HashMap;

use zbus::dbus_proxy;
use zvariant::{Value, OwnedValue};

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings.Connection"
)]
pub trait NetworkManagerConnection {
    fn update(&self, properties: HashMap<&str, HashMap<&str, &Value<'_>>>) -> zbus::Result<()>;

    fn update_2(
        &self,
        settings: HashMap<&str, HashMap<&str, &Value<'_>>>,
        flags: u32,
        args: HashMap<&str, &Value<'_>>
    ) -> zbus::Result<HashMap<String, OwnedValue>>;

    fn update_unsaved(&self, properties: HashMap<&str, HashMap<&str, &Value<'_>>>) -> zbus::Result<()>;

    fn delete(&self) -> zbus::Result<()>;

    fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;

    fn get_secrets(&self, setting_name: &str) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;

    fn clear_secrets(&self) -> zbus::Result<()>;

    fn save(&self) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn updated(&self) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn removed(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn unsaved(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn readable(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn filename(&self) -> zbus::Result<String>;
}