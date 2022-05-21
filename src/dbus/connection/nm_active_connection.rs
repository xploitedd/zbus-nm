use zbus::dbus_proxy;
use zvariant::OwnedObjectPath;

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Connection.Active"
)]
pub trait NetworkManagerActiveConnection {
    #[dbus_proxy(property)]
    fn connection(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn specific_object(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;

    #[dbus_proxy(name = "type")]
    #[dbus_proxy(property)]
    fn connection_type(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn state_flags(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn default(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn ip4_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn dhcp4_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn default6(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn ip6_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn dhcp6_config(&self) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn vpn(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn master(&self) -> zbus::Result<OwnedObjectPath>;
}