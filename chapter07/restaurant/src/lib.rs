mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    
    let appetizer = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheet");
    println!("I'd like {} toast please", meal.toast);
}