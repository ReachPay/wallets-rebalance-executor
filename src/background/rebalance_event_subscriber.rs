use std::{collections::HashMap, sync::Arc};

use my_sb_contracts::FireblocksRebalanceCommand;
use my_service_bus_abstractions::subscriber::{
    MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
};
use rust_extensions::Logger;

use crate::AppContext;

pub struct RebalanceEventSubscriber {
    app: Arc<AppContext>,
}

impl RebalanceEventSubscriber {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<FireblocksRebalanceCommand> for RebalanceEventSubscriber {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<FireblocksRebalanceCommand>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let message = message.take_message();

            let assets_info = get_fireblocks_asset_info(&self.app).await;
            let asset_info = assets_info.get(&message.asset_id);

            if let None = asset_info {
                my_logger::LOGGER.write_info(
                    "SubscriberCallback<FireblocksRebalanceCommand> -> handle_messages".to_string(),
                    format!("asset info not found event for {:?}. Skip event.", message),
                    None,
                );
                return Ok(());
            };

            let asset_info = asset_info.unwrap();

            //SEND BASE ASSET
            if let Some(base_asset) = &asset_info.base_asset {
                let base_asset_info = assets_info.get(base_asset);

                if let None = base_asset_info {
                    my_logger::LOGGER.write_info(
                        "SubscriberCallback<FireblocksRebalanceCommand> -> handle_messages"
                            .to_string(),
                        format!(
                            "base asset info not found event for {:?}. Skip event.",
                            message
                        ),
                        None,
                    );
                    return Ok(());
                };

                let base_asset_info = base_asset_info.unwrap();

                self.app
                    .fireblocks_bridge_grpc_client
                    .execute_transaction(
                        self.app
                            .settings_model
                            .rebalance_fireblocks_vault_id
                            .clone(),
                        message.vault_id.clone(),
                        base_asset.to_string(),
                        base_asset_info.gas_fee,
                        Some("Rebalance gas fee. Sending base.".to_string()),
                    )
                    .await;
            };

            self.app
                .fireblocks_bridge_grpc_client
                .execute_transaction(
                    message.vault_id.clone(),
                    self.app
                        .settings_model
                        .rebalance_fireblocks_vault_id
                        .clone(),
                    message.asset_id,
                    message.rebalance_amount,
                    Some("Rebalance execute.".to_string()),
                )
                .await;
            return Ok(());
        }
        return Ok(());
    }
}

struct FireblocksAssetInfo {
    pub base_asset: Option<String>,
    pub gas_fee: f64,
}

async fn get_fireblocks_asset_info(app: &Arc<AppContext>) -> HashMap<String, FireblocksAssetInfo> {
    return app
        .crypto_deposit_settings_reader
        .get_table_snapshot()
        .await
        .unwrap()
        .values()
        .flat_map(|x| {
            x.values().map(|x| {
                (
                    x.fireblocks_id.clone(),
                    FireblocksAssetInfo {
                        base_asset: x.base_fireblocks_id.clone(),
                        gas_fee: x.rebalance_gas_fee_amount,
                    },
                )
            })
        })
        .collect();
}
