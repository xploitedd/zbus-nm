use std::sync::Arc;

use zbus::Connection as ZConnection;
use zvariant::OwnedObjectPath;

use crate::{dbus::{nm_connection::NetworkManagerConnectionProxy, nm::NetworkManagerProxy}, util::{Result, ToErrString}};

pub struct Connection {
    path: OwnedObjectPath,
    conn: Arc<ZConnection>,
    nm_proxy: Arc<NetworkManagerProxy<'static>>,
    con_proxy: NetworkManagerConnectionProxy<'static>
}

impl Connection {
    pub(crate) async fn new(
        conn: Arc<ZConnection>, 
        nm_proxy: Arc<NetworkManagerProxy<'static>>,
        path: OwnedObjectPath
    ) -> Result<Self> {
        let con_proxy = NetworkManagerConnectionProxy::builder(&*conn)
            .path(path.clone())
            .to_err_string()?
            .build()
            .await
            .to_err_string()?;

        Ok(Self {
            path,
            conn,
            nm_proxy,
            con_proxy
        })
    }
}