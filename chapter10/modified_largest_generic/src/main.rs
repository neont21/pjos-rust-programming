fn main() {
    let number_list = vec![34, 50, 35, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let character_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest character is {}", largest(&character_list));
}

fn largest<T>(list: &[T]) -> T
    where T: PartialOrd
{
    let mut largest = list[0];

    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}