fn main() {
    let number_list = vec![34, 50, 35, 100, 65];
    println!("The largest number is {}", largest_i32(&number_list));

    let character_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest character is {}", largest_char(&character_list));
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}