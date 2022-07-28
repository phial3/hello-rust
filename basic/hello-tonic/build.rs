// 这个配置方式不会生成.rs文件，本地无法查看依赖和编码提示，不建议使用
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}