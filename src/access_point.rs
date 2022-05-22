use std::{sync::Arc, fmt::Debug};

use zbus::Connection as ZConnection;
use zvariant::OwnedObjectPath;

use crate::{dbus::{nm::NetworkManagerProxy, nm_access_point::NetworkManagerAccessPointProxy}, util::{Result, ToErrString}, device::wifi::WifiDevice};

pub struct Ssid {
    ssid: Vec<u8>
}

impl Ssid {
    fn new(ssid: Vec<u8>) -> Self {
        Self { ssid }
    }

    pub fn to_string(self) -> Result<String> {
        String::from_utf8(self.ssid)
            .to_err_string()
    }
}

pub struct AccessPoint {
    pub(crate) path: OwnedObjectPath,
    pub(crate) conn: Arc<ZConnection>,
    pub(crate) nm_proxy: Arc<NetworkManagerProxy<'static>>,
    pub(crate) ap_proxy: NetworkManagerAccessPointProxy<'static>
}

impl AccessPoint {
    pub(crate) async fn new(
        conn: Arc<ZConnection>,
        nm_proxy: Arc<NetworkManagerProxy<'static>>,
        path: OwnedObjectPath
    ) -> Result<Self> {
        let ap_proxy = NetworkManagerAccessPointProxy::builder(&*conn)
            .path(path.clone())
            .to_err_string()?
            .build()
            .await
            .to_err_string()?;

        Ok(Self {
            path,
            conn,
            nm_proxy,
            ap_proxy
        })
    }

    pub async fn connect(&self, wifi_device: &WifiDevice) {

    }

    pub async fn get_ssid(&self) -> Result<Ssid> {
        let ssid = self.ap_proxy.ssid()
            .await
            .to_err_string()?;

        Ok(Ssid::new(ssid))
    }
}

impl Debug for AccessPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessPoint")
            .field("path", &self.path)
            .finish()
    }
}