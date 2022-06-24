fn main() {
    {
        let s = "hello";
        println!("{}", s); // hello
    }

    // s is out of scope

    let s = String::from("hello");
    takes_ownership(s);

    // s cannot be used now
    // println!("S: {}", s);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    let str = String::from("hello");
    let (s2, len) = calculate_length(str);

    println!("The length of '{}' is '{}'", s2, len);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn gives_ownership() -> String {
    let str = String::from("yours");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}
