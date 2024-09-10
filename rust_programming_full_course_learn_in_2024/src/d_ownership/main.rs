/*
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

pub fn test() -> () {
    // ------------ 1. Each value in Rust has an owner.
    let str: String = String::from("Rust");
    let str_len: usize = calculate_length(&str);

    println!("Length of {:?} is {} chars.", str, str_len);

    // ------------ 2. There can only be one owner at a time.
    let str2: String = String::from("Rust2");
    let str2_2: String = str2;

    // Can't, the new owner is 'str2_2'
    //println!("{:?}", str2);
    
    println!("{:?}", str2_2);

    // ------------ 3. When the owner goes out of scope, the value will be dropped.
    {
        let str3: String = String::from("Rust3");
        println!("{:?}", str3);
    }

    // Out of scope, str3 does not exist anymore
    //println!("{:?}", str3);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}
