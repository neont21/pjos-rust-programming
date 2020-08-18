fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {}", print_option(five));
    println!("six: {}", print_option(six));
    println!("none: {}", print_option(none));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_option(x: Option<i32>) -> String {
    match x {
        None => String::from("None"),
        Some(i) => format!("{}", i),
    }
}