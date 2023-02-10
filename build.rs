fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto/")
        .compile(&["proto/fetch.proto"], &["proto/"])
        .unwrap()
}
