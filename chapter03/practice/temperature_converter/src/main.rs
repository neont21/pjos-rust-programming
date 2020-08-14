use std::io;

fn main() {
    println!("[1] Convert Fahrenheit to Celsius");
    println!("[2] Convert Celsius to Fahrenheit");
    let choice = loop {
        println!("Input the choice.");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Fail to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input number 1 or 2");
                continue
            },
        };

        if choice > 0 && choice < 3 {
            break choice;
        }
    };

    let temperature = loop {
        println!("Input the temperature to convert without the unit symbol.");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Fail to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input the tempeture without unit symbol");
                continue
            },

        };

        break temperature;
    };

    let temperature = if choice == 1 {
        fahrenheit_to_celsius(temperature)
    } else {
        celsius_to_fahrenheit(temperature)
    };

    println!("result: {}", temperature);
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.) / 1.8
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.
}