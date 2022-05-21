pub mod nm;
pub mod nm_settings;
pub mod nm_connection;
pub mod device;

bitflags::bitflags! {
    pub struct NmReloadFlags : u32 {
        const ALL = 0x0;
        const CONF_FROM_DISK = 0x1;
        const UPDATE_DNS = 0x2;
        const RESTART_DNS = 0x4;
    }

    pub struct NmConnectivityStateFlags : u32 {
        const UNKNOWN = 0x0;
        const NONE = 0x1;
        const PORTAL = 0x2;
        const LIMITED = 0x3;
        const FULL = 0x4;
    }

    pub struct NmStateFlags : u32 {
        const UNKNOWN = 0;
        const ASLEEP = 10;
        const DISCONNECTED = 20;
        const DISCONNECTING = 30;
        const CONNECTING = 40;
        const CONNECTED_LOCAL = 50;
        const CONNECTED_SITE = 60;
        const CONNECTED_GLOBAL = 70;
    }

    pub struct NmCheckpointCreateFlags : u32 {
        const NONE = 0x0;
        const DESTROY_ALL = 0x1;
        const DELETE_NEW_CONNECTIONS = 0x2;
        const DISCONNECT_NEW_DEVICES = 0x4;
        const ALLOW_OVERLAPPING = 0x8;
    }

    pub struct NmDeviceType : u32 {
        const UNKNOWN = 0;
        const WIFI = 2;
    }
}