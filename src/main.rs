use lazy_static::lazy_static;

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

#[test]
fn if_expression(){
    let value = 2;
    let result: &str;

    if value >= 8 {
        result = "Good!";
    } else if value >= 6 {
        result = "Not Bad!";
    } else if value >= 3 {
        result = "Bad!";
    } else {
        result = "Very Bad!";
    }

    println!("{}", result);
}

#[test]
fn if_expression_let_statement(){
    let value = 2;
    let result: &str = if value >= 8 {
        "Good!"
    } else if value >= 6 {
        "Not Bad!"
    } else if value >= 3 {
        "Bad!"
    } else {
        "Very Bad!"
    };

    println!("{}", result);
}

#[test]
fn loop_expression(){
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label(){
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop(){
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len(){
        println!("Value: {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Value: {}", array[i]);
    }
}

#[test]
fn range_inclusive(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Value: {}", array[i]);
    }
}

fn say_hello(){
    println!("Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {}, {}", first_name, last_name);
}

#[test]
fn test_parameter() {
    say_goodbye("Sigit", "Boworaharjo");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("Result: {}", result);

    let result = factorial_loop(-10);
    println!("Result: {}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Hello"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("Result: {}", result);
}

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("name: {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Sigit");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Sigit");
    let last_name = String::from("Boworaharjo");

    let name = full_name(first_name, last_name);
    println!("{}", name);
    // println!("{}", first_name);
    // println!("{}", last_name);
}

fn full_name_0(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    (first_name, last_name, full_name)
}

#[test]
fn test_full_name_0() {
    let first_name = String::from("Sigit");
    let last_name = String::from("Boworaharjo");

    let (a, b, full_name) = full_name_0(first_name, last_name);
    println!("{}", full_name);
    println!("{}", a);
    println!("{}", b);
}

fn full_name_1(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name_1() {
    let first_name = String::from("Sigit");
    let last_name = String::from("Boworaharjo");

    let full_name = full_name_1(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

// fn change_value(value: &String) {
//     value.push_str("Kang")
// }
//
// #[test]
// fn test_change_value() {
//     let mut value = String::from("Sigit");
//     change_value(&value);
//     println!("{}", value);
// }

fn change_value_0(value: &mut String) {
value.push_str("Sigit")
}

#[test]
fn test_change_value_0() {
    let mut value = String::from("Kang ");
    change_value_0(&mut value);
    println!("{}", value);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Sigit");
    let last_name = String::from("Boworaharjo");

    let full_name = get_full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("Sigit Boworaharjo");
    let first_name: &str = &name[0..5];
    println!("{}", first_name);

    let last_name: &str = &name[6..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Sigit");
    let last_name = String::from("Sukisno");

    let person: Person = Person{
        first_name,
        middle_name: String::from("Boworaharjo"),
        last_name,
        age: 29
    };

    print_person(&person);

    let person2: Person = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);

    print!("{}", person.first_name);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.200000, 106.816666);
    println!("long : {}", geo_point.0);
    println!("lat : {}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing0: Nothing = Nothing;
    let _nothing1: Nothing = Nothing{};
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_method() {
    let person = Person{
        first_name: String::from("Sigit"),
        middle_name: String::from("Boworaharjo"),
        last_name: String::from("Sukisno"),
        age: 29
    };

    person.say_hello("Dhini");

    println!("{}", person.first_name);
}

#[test]
fn test_method_new() {
    let geo_point = GeoPoint::new(-6.200000, 106.816666);
    println!("long : {}", geo_point.0);
    println!("lat : {}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum
}

#[test]
fn test_enum() {
    let level: Level = Level::Regular;
    match level {
        Level::Regular => println!("Regular"),
        Level::Premium => println!("Premium"),
        Level::Platinum => println!("Platinum")
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

#[test]
fn test_payment() {
    let _payment0 = Payment::CreditCard(String::from("637746644"));
    let _payment1 = Payment::BankTransfer(String::from("BCA"), String::from("53663552"));
    let _payment2 = Payment::EWallet(String::from("GoPay"), String::from("63553442522"));
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with EWallet {} {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment_0() {
    let _payment0 = Payment::CreditCard(String::from("637746644"));
    _payment0.pay(10000);
    let _payment1 = Payment::BankTransfer(String::from("BCA"), String::from("53663552"));
    _payment1.pay(300000);
    let _payment2 = Payment::EWallet(String::from("GoPay"), String::from("63553442522"));
    _payment2.pay(10000);
}

#[test]
fn test_match_value() {
    let name: &str = "Suri";

    match name {
        "Eko" => {
            println!("Hello Eko");
        }
        "Budi" => {
            println!("Hello Eko");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Eko" | "Suri" => {
            println!("Hello Boss");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 101;
    match value {
        75..=100 => println!("Great"),
        50..=74 => println!("Good"),
        25..=49 => println!("Not Bad"),
        0..=24 => println!("Bad"),
        other => println!("Invalid value {}", other),
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(0.0, 106.816666);
    match point {
        GeoPoint(long, 0.0) => {
            println!("Long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("Lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("Long: {} Lat: {}", long, lat);
        }
    }

    let person = Person{
        first_name: String::from("Sigit"),
        middle_name: String::from("Boworaharjo"),
        last_name: String::from("Sukisno"),
        age: 29
    };

    match person {
        Person{first_name, middle_name, ..} => {
            println!("{} {}", first_name, middle_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(0.0, 106.816666);
    match point {
        GeoPoint(long, _) => {
            println!("Long: {}", long);
        }
    }
}

#[test]
fn test_ignoring_range() {
    let value = 101;
    match value {
        75..=100 => println!("Great"),
        50..=74 => println!("Good"),
        25..=49 => println!("Not Bad"),
        0..=24 => println!("Bad"),
        _ => println!("Invalid value"),
    }
}

#[test]
fn test_match_expression() {
    let value: i32 = 9;
    let result: &str = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };
    println!("{}", result);
}