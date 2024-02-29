use std::fmt::Display;

fn main() {
    let data = Data::new(1, "RUST".to_string());
    let not_data = NotImplData::new(2, "GO".to_string());

    printf(data);
    
    // error: not implmented Display
    printf(not_data);
}

#[derive(Clone)]
struct Data {
    id: i32,
    text: String,
}


impl Data {
    fn new(id: i32, text: String) -> Self {
        Self { 
            id,
            text,
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The Id is: {}, Text is: {}", self.id, self.text)
    }
}

struct NotImplData {
    id: i32,
    text: String,
}

impl NotImplData {
    fn new(id: i32, text: String) -> Self {
        Self { 
            id,
            text,
        }
    }
}


fn printf<T: Display>(data: T) {
    println!("{}", data);
}