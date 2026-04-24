fn main() {
    println!("Hello, world!");

    println!("Hello Sigit");

    println!("Hello Dhini")
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Sigit Boworaharjo";
    println!("Hello, {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Sigit Boworaharjo";
    println!("Hello, {}", name);

    name = "Dhini";
    println!("Hello, {}", name);
}

#[test]
fn static_typing() {
    let name = "Sigit Boworaharjo";
    println!("Hello, {}", name);

    // name = 10;
    println!("Hello, {}", name);
}

#[test]
fn shadowing() {
    let name = "Sigit Boworaharjo";
    println!("Hello, {}", name);

    let name = 10;
    println!("Hello, {}", name);
}

/*
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
*/
#[test]
fn comment() {
    // ini komentar
    println!("Hello"); // ini komentar lagi
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("Hello, {}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c);

    let d: i64 = 100000000;
    let e: i8 = d as i8;
    println!("{}", e);
}