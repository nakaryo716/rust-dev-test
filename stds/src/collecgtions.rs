pub fn vector() {
    let text = String::from("Hello");

    let v = text.bytes().collect::<Vec<u8>>();

    println!("{:?}", v);
    println!("{}", v.capacity());
}

pub fn string_test() {
    let mut text = "Hello".to_string();

    text.insert_str(5, " world");

    let i = text.find("world").unwrap();

    let new = text.replace("world", "Rust");
    println!("{}", i);
    println!("{}", text);
    println!("{}", new);
}

pub fn strincg_test2() {
    let text = "256".to_string();

    let number = text.parse::<i32>().unwrap();

    println!("{}", number + 10);
}

pub fn string_test3() {
    let number = 256;

    let mut text = number.to_string();
    text.push_str(": this is text");
    println!("{}", text);
}
