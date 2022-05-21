use std::{sync::Arc, fmt::Debug};

use zbus::Connection as ZConnection;
use zvariant::OwnedObjectPath;

use crate::{dbus::{nm::NetworkManagerProxy, nm_access_point::NetworkManagerAccessPointProxy}, util::{Result, ToErrString}, device::wifi::WifiDevice};

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

    pub fn connect(&self, wifi_device: &WifiDevice) {

    }
}

impl Debug for AccessPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessPoint")
            .field("path", &self.path)
            .finish()
    }
}