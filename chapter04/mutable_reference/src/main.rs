fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("string: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
