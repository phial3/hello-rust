use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 10];

    // 精确读取 10 个字节
    f.read_exact(&mut buffer)?;

    println!("Buffer data is:{:?}", buffer);

    Ok(())
}
