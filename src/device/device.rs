use std::sync::Arc;

use zbus::Connection as ZConnection;
use zvariant::OwnedObjectPath;

use crate::{dbus::{device::nm_device::NetworkManagerDeviceProxy, nm::NetworkManagerProxy}, util::{Result, ToErrString}};

use super::wifi::WifiDevice;

pub enum DeviceType {
    Wifi(WifiDevice)
}

impl DeviceType {
    async fn from(device: Device) -> Result<DeviceType> {
        let device_type_flag = device.dev_proxy
            .device_type()
            .await
            .to_err_string()?;

        match device_type_flag {
            2 => {
                let wifi_dev = WifiDevice::new(device)
                    .await?;

                Ok(DeviceType::Wifi(wifi_dev))
            },
            _ => Err(String::from("Device type not found!"))
        }
    }
}

pub struct Device {
    pub(crate) path: OwnedObjectPath,
    pub(crate) conn: Arc<ZConnection>,
    pub(crate) nm_proxy: Arc<NetworkManagerProxy<'static>>,
    pub(crate) dev_proxy: Arc<NetworkManagerDeviceProxy<'static>>
}

impl Device {
    pub(crate) async fn new(
        conn: Arc<ZConnection>,
        nm_proxy: Arc<NetworkManagerProxy<'static>>,
        path: OwnedObjectPath
    ) -> Result<Self> {
        let dev_proxy = NetworkManagerDeviceProxy::builder(&*conn)
            .path(path.clone())
            .to_err_string()?
            .build()
            .await
            .to_err_string()?;

        Ok(Self {
            path,
            conn,
            nm_proxy,
            dev_proxy: Arc::new(dev_proxy)
        })
    }

    pub async fn get_device_type(self) -> Result<DeviceType> {
        DeviceType::from(self).await
    }

    pub async fn disconnect(&self) -> Result<()> {
        self.dev_proxy.disconnect()
            .await
            .to_err_string()
    }

    pub async fn delete(&self) -> Result<()> {
        self.dev_proxy.delete()
            .await
            .to_err_string()
    }
}