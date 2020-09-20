fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
