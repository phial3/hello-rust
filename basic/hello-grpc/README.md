
## protobuf rust
```shell
#默认安装到~/.cargo/bin目录中
cargo install protobuf 
### Install the protobuf compiler,  默认安装到~/.cargo/bin目录中
cargo install protobuf-codegen
### Install the gRPC compiler ,默认安装到~/.cargo/bin目录中
cargo install grpc-compiler
cargo install grpcio-compiler

protoc --rust_out=. *.proto
protoc --rust-grpc_out=. *.proto
protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin`
```