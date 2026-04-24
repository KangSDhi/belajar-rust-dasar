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

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
    let f = a - b;
    println!("{}", f);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison(){
    let a = 20;
    let b = 20;

    let result : bool = a > b;
    println!("{}", result);
}

#[test]
fn boolean_operator(){
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}