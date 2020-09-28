fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", 4),
        Some(x) => println!("{}", x),
        None => (),
    }
}