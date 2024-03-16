#[allow(dead_code)]
pub fn iter_tes() {
    froms();
    // deferent();
}

// .iter()は参照
// .into_iter()は所有権を奪う
#[allow(dead_code)]
fn deferent() {
    let v = vec![0, 1, 2, 3, 4, 5];

    for i in v.iter() {
        println!("{}", i);
    }

    for i in v.into_iter() {
        println!("{}", i);
    }

    // vはムーブ.into_iter()でしたのでもう使えない
    // println!("{:?}", v);
}

// from_fnはOptionを返すクロージャFnMut()を実行する
#[allow(dead_code)]
fn froms() {
    let gen = || Some(128);

    let buf: Vec<i32> = std::iter::from_fn(gen).take(5).collect();

    println!("{:?}", buf);
}

pub mod other {
    use std::vec;

    #[allow(dead_code)]
    pub fn range() {
        let buf: Vec<i32> = (0..=10).collect();
        println!("{:?}", buf);
    }

    // split()は引数のクロージャを評価して分ける
    #[allow(dead_code)]
    pub fn split_vec() {
        let v = vec![1, 1, 2, 3, 4];

        let buf = v.split(|x| x % 2 == 0).into_iter();

        for i in buf {
            println!("{:?}", i);
        }
    }

    // 文字列を任意の文字で分ける.split()
    #[allow(dead_code)]
    pub fn split_str() {
        let text = "Hello World Rustacean".to_string();

        let v: Vec<String> = text.split(" ").map(|s| s.to_string()).collect();
        println!("{:?}", v);
    }

    // 文字列をbyte列に変換する.byte()
    pub fn bytes_str() {
        let text = "Hello World Rustacean".to_string();

        let v: Vec<u8> = text.bytes().collect();
        println!("{:?}", v);
        let x = String::from_utf8(v).unwrap();
        println!("{}", x);
    }

    // Optionをアンラップする
    pub fn filter() {
        let v = vec![Ok(32), Ok(128), Err(3), Ok(256)];

        let s: Vec<i32> = v.iter().filter_map(|x| x.ok()).collect();
        println!("{:?}", s);
    }

    // ある条件のときに値を返すイテレータアダプタ
    pub fn selected() {
        let v: Vec<i32> = (0..=10).collect();

        let filtered: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
        println!("{:?}", filtered);
    }

    // zipはenumrate()を一般化したようなもの
    pub fn zip() {
        let i = vec![16, 32, 64, 128, 512];

        let text = vec!["first", "second", "thired", "fourth", "fifth"];

        let v: Vec<_> = i.iter().zip(text).collect();

        for (i, element) in v {
            println!("{}: {}", i, element);
        }
    }
}
