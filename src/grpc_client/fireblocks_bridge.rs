use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::crypto_wallets_grpc::{
    crypto_wallets_manager_client::CryptoWalletsManagerClient, ExecuteTransactionGrcpRequest,
};

pub struct FireblocksBridgeGrpcService {
    channel: Channel,
}

impl FireblocksBridgeGrpcService {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self { channel }
    }

    fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> CryptoWalletsManagerClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return CryptoWalletsManagerClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn execute_transaction(
        &self,
        vault_from: String,
        vault_to: String,
        asset: String,
        amount: f64,
        message: Option<String>,
    ) {
        let request = ExecuteTransactionGrcpRequest {
            vault_from,
            vault_to,
            asset,
            amount,
            message,
        };

        let mut client = self.create_grpc_service(&my_telemetry::MyTelemetryContext::new());
        client.execute_transaction(request).await.unwrap();
    }
}
