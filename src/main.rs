use std::sync::Arc;

use wallets_rebalance_executor::{AppContext, RebalanceEventSubscriber, SettingsReader, APP_NAME};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".reachpay").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(&settings_reader).await);

    app.sb_client
        .subscribe(
            APP_NAME.to_string(),
            my_service_bus_abstractions::subscriber::TopicQueueType::Permanent,
            Arc::new(RebalanceEventSubscriber::new(app.clone())),
        )
        .await;

    app.sb_client.start().await;
    app.nosql_connection.start().await;
    app.app_states.wait_until_shutdown().await;
}
