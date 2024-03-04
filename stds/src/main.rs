use core::fmt;
use std::fmt::Display;

mod lamda;
mod iter_test;
mod collecgtions;

fn main() {
    let mut a = TypeA::new(1, "ryo".to_string());

    let buf: [u8; 4] = [0, 1, 2, 3];
    let hoge = Vec::from(buf);

}

struct TypeA {
    id: i32,
    name: String,
}

impl TypeA {
    fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}

impl Display for TypeA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}