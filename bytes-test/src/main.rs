use std::{
    fs::File,
    io::{self, Read},
};

use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    let response = use_fs_sys().unwrap();
    println!("{:?}", response);

    let mut buf = Vec::new();
    let _ = response.iter().map(|x| buf.push(*x));

    test();
}

fn test() {
    let a = Bytes::from("Hello".to_string());
    println!("{:?}", a);

    let mut b = BytesMut::with_capacity(5);
    b.put(&b"hello"[..]);
    println!("{:?}", b);

    b.clear();
    println!("{:?}", b);
}

fn use_fs_sys() -> Result<BytesMut, io::Error> {
    let mut file = File::open("text.txt")?;

    let mut buf = BytesMut::new();

    let mut src = Vec::new();
    file.read_to_end(&mut src)?;

    buf.put(&*src);

    Ok(buf)
}
