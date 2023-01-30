use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_service_bus_tcp_client::MyServiceBusSettings;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "SbHostPort")]
    pub sb_hostport: String,

    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,

    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,

    #[serde(rename = "NoSqlTcp")]
    pub no_sql_tcp: String,

    #[serde(rename = "FireblocksBridgeGrpc")]
    pub fireblocks_bridge_grpc: String,

    #[serde(rename = "RebalanceFireblocksVaultId")]
    pub rebalance_fireblocks_vault_id: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.no_sql_tcp.clone()
    }
}

#[async_trait::async_trait]
impl MyServiceBusSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.sb_hostport.clone()
    }
}
