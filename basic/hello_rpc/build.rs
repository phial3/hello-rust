fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .input("./proto/hello.proto")
        .out_dir("./proto/")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
