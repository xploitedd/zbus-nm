use std::sync::Arc;

use futures::future;
use zbus::Connection as ZConnection;

use crate::{dbus::{nm::NetworkManagerProxy, nm_settings::NetworkManagerSettingsProxy}, connection::Connection, device::device::{Device, DeviceType}, util::{Result, ToErrString, ToOption}};

pub struct NetworkManager {
    conn: Arc<ZConnection>,
    nm_proxy: Arc<NetworkManagerProxy<'static>>,
    nm_settings_proxy: NetworkManagerSettingsProxy<'static>
}

impl NetworkManager {
    pub async fn new() -> Result<Self> {
        let conn = ZConnection::system()
            .await
            .to_err_string()?;

        let nm_proxy = NetworkManagerProxy::new(&conn)
            .await
            .to_err_string()?;

        let nm_settings_proxy = NetworkManagerSettingsProxy::new(&conn)
            .await
            .to_err_string()?;

        Ok(Self {
            conn: Arc::new(conn),
            nm_proxy: Arc::new(nm_proxy),
            nm_settings_proxy
        })
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let ft_dev = self.nm_proxy.get_all_devices()
            .await
            .to_err_string()?
            .into_iter()
            .map(|v| Device::new(
                self.conn.clone(),
                self.nm_proxy.clone(),
                v
            ));

        let devices: Vec<_> = future::join_all(ft_dev)
            .await
            .into_iter()
            .filter_map(|d| d.to_option())
            .collect();

        Ok(devices)
    }

    pub async fn get_devices_by_type(&self) -> Result<Vec<DeviceType>> {
        let devices_ft = self.get_devices()
            .await?
            .into_iter()
            .map(|d| d.get_device_type());

        let devices: Vec<_> = future::join_all(devices_ft)
            .await
            .into_iter()
            .filter_map(|d| d.to_option())
            .collect();

        Ok(devices)
    }

    pub async fn get_connections(&self) -> Result<Vec<Connection>> {
        let ft_conn = self.nm_settings_proxy.list_connections()
            .await
            .to_err_string()?
            .into_iter()
            .map(|v| Connection::new(
                self.conn.clone(), 
                self.nm_proxy.clone(), 
                v
            ));

        let connections: Vec<_> = future::join_all(ft_conn)
            .await
            .into_iter()
            .filter_map(|c| c.to_option())
            .collect();

        Ok(connections)
    }
}

#[cfg(test)]
mod tests {

    use crate::device::device::DeviceType;

    use super::*;

    #[tokio::test]
    async fn try_connect_wifi() -> Result<()> {
        let nm = NetworkManager::new()
            .await?;

        let wifi_device = nm.get_devices_by_type()
            .await?
            .into_iter()
            .find_map(|d| {
                match d {
                    DeviceType::Wifi(wifi_device) => Some(wifi_device),
                    _ => None
                }
            })
            .unwrap();

        let aps = wifi_device.get_access_points().await?;
        for ap in aps {
            println!("{:?}", ap.get_ssid().await?.to_string().unwrap())
        }
            
        Ok(())
    }

}