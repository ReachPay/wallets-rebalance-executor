fn main() {
    tonic_build::compile_protos("proto/CryptoWalletsManager.proto").unwrap();
}
