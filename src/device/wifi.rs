use std::{ops::Deref, collections::HashMap};

use futures::{future, StreamExt};

use crate::{dbus::device::nm_device_wireless::NetworkManagerWirelessDeviceProxy, util::{Result, ToOption}, util::ToErrString, access_point::AccessPoint};

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

    pub async fn get_access_points(&self) -> Result<Vec<AccessPoint>> {
        let mut last_scan = self.wifi_dev_proxy.receive_last_scan_changed()
            .await;

        self.wifi_dev_proxy.request_scan(HashMap::new())
            .await
            .to_err_string()?;

        last_scan.next().await;

        let ap_ft = self.wifi_dev_proxy.get_all_access_points()
            .await
            .to_err_string()?
            .into_iter()
            .map(|ap| AccessPoint::new(
                self.conn.clone(),
                self.nm_proxy.clone(),
                ap
            ));

        let access_points = future::join_all(ap_ft)
            .await
            .into_iter()
            .filter_map(|ap| ap.to_option())
            .collect();

        Ok(access_points)
    }
}

impl Deref for WifiDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}