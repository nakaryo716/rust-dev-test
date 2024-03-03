pub fn lamda_test() {
    let lamda = |x: i32| {
        x + 1
    }; 
    println!("{}", hoge(lamda)); 

    let lamda_re = |x: i32| {
        x + 100
    };
    println!("{}", hoge(lamda_re));


    let mut data = 5;
    let lamda2 =  |x: i32| {
        data += x;
    };
    hoge2(lamda2, 36);
    println!("{}", data);

}


fn hoge<T: Fn(i32) -> i32>(lamda: T) -> i32 {
    let ans = lamda(4) + lamda(2);

    ans
}

fn hoge2<T: FnMut(i32)>(mut lamda: T, num: i32) {
    lamda(num);
}