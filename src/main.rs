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

#[test]
fn tuple(){
    let mut  data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    let (a, b, _) = data;
    println!("{} {}", a, b);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit(){
    println!("Hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array(){
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;
#[test]
fn constant(){
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope(){
    println!("{}", MAXIMUM);

    let sigit = 1;
    {
        println!("{}", sigit);
        let boworaharjo = 2;
        println!("{}", boworaharjo);
    }

    // println!("{}", boworaharjo); // error
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_b(){
    let a = 10;
    let b = String::from("Sigit");

    println!("{} {}", a, b);
}

fn function_a(){
    let a = 10;
    let b = String::from("Boworaharjo");

    println!("{} {}", a, b);
}

#[test]
fn string(){
    let name: &str = " Sigit Boworaharjo ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type(){
    let mut name: String = String::from("Sigit Boworaharjo");
    println!("{}", name);

    name.push_str(" Sukisno");
    println!("{}", name);

    let dhini = name.replace("Sigit", "Dhini");
    println!("{}", dhini);
}

#[test]
fn ownership_rules(){
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy(){
    let a = 10;
    let b = a; // copy data dari a ke b

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement(){
    let name1 = String::from("Sigit Boworaharjo");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses disini

    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone(){
    let name1 = String::from("Sigit Boworaharjo");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}