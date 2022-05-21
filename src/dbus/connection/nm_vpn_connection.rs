use zbus::dbus_proxy;

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.VPN.Connection"
)]
pub trait NetworkManagerVpnConnection {
    #[dbus_proxy(property)]
    fn vpn_state(&self) -> zbus::Result<u32>;

    #[dbus_proxy(property)]
    fn banner(&self) -> zbus::Result<String>;
}