use std::ops::Deref;

use crate::{dbus::device::nm_device_wireless::NetworkManagerWirelessDeviceProxy, util::Result, util::ToErrString};

use super::device::Device;

pub struct WifiDevice {
    device: Device,
    wifi_dev_proxy: NetworkManagerWirelessDeviceProxy<'static>
}

impl WifiDevice {
    pub(crate) async fn new(device: Device) -> Result<WifiDevice> {
        let conn = &*device.conn;
        let wifi_dev_proxy = NetworkManagerWirelessDeviceProxy::builder(conn)
            .path(device.path.clone())
            .to_err_string()?
            .build()
            .await
            .to_err_string()?;

        Ok(Self {
            device,
            wifi_dev_proxy
        })
    }

    pub async fn get_access_points(&self) {
        // self.wifi_dev_proxy.get_all_access_points()
    }

    pub async fn connect_to_access_point(&self) {

    }
}

impl Deref for WifiDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}