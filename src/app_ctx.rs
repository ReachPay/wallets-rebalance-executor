use std::sync::Arc;

use my_no_sql_contracts::crypto_deposit_settings_entity::CryptoDepositSettingsEntity;
use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_service_bus_tcp_client::MyServiceBusClient;
use rust_extensions::AppStates;

use crate::{FireblocksBridgeGrpcService, SettingsModel};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub sb_client: MyServiceBusClient,
    pub app_states: Arc<AppStates>,
    pub settings_model: Arc<SettingsModel>,
    pub crypto_deposit_settings_reader: Arc<MyNoSqlDataReader<CryptoDepositSettingsEntity>>,
    pub fireblocks_bridge_grpc_client: FireblocksBridgeGrpcService,
    pub nosql_connection: MyNoSqlTcpConnection,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> AppContext {
        let settings = Arc::new(settings_reader.get_settings().await);

        let sb_client = MyServiceBusClient::new(
            APP_NAME,
            APP_VERSION,
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        );

        let nosql_connection =
            MyNoSqlTcpConnection::new(APP_NAME.to_string(), settings_reader.clone());

        let crypto_deposit_settings_reader = nosql_connection
            .get_reader::<CryptoDepositSettingsEntity>()
            .await;

        let fireblocks_bridge_grpc_client =
            FireblocksBridgeGrpcService::new(settings.fireblocks_bridge_grpc.clone()).await;

        Self {
            sb_client,
            app_states: Arc::new(AppStates::create_initialized()),
            settings_model: settings,
            crypto_deposit_settings_reader,
            fireblocks_bridge_grpc_client,
            nosql_connection,
        }
    }
}
