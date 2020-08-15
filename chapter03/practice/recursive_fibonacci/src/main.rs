use std::io;

fn main() {
    let number = loop {
        println!("Input the number.");
        
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Fail to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again...");
                continue
            },
        };

        break number;
    };

    println!("{}th fibonacci number is {}.", number, fibonacci(number));
}

fn fibonacci(num: u32) -> u32 {
    if num <= 2 {
        return 1;
    }

    fibonacci(num-1) + fibonacci(num-2)
}