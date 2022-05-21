use zbus::dbus_proxy;

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.AccessPoint"
)]
pub trait NetworkManagerAccessPoint {
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    #[dbus_proxy(property)]
    fn frequency(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn max_bitrate(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn strength(&self) -> zbus::Result<u8>;

    #[dbus_proxy(property)]
    fn last_seen(&self) -> zbus::Result<i32>;
}