FROM rust:slim
COPY ./target/release/wallets-rebalance-executor ./target/release/wallets-rebalance-executor
ENTRYPOINT ["./target/release/wallets-rebalance-executor"]