
## protobuf rust
```shell
cargo add protobuf-codegen
cargo install protobuf-codegen #默认安装到~/.cargo/bin目录中
cargo add grpc-compiler
cargo install grpc-compiler   #默认安装到~/.cargo/bin目录中
cargo add protobuf
cargo install protobuf #默认安装到~/.cargo/bin目录中

protoc --rust_out=. *.proto
protoc --rust-grpc_out=. *.proto
```