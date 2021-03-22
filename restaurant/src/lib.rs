mod back_of_house;
mod front_of_house;
pub use crate::back_of_house::Breakfast;
pub use crate::front_of_house::hosting;

fn serve_order() {}

fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();
    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order1: {:#?} {:#?}", order1, order2);
}

// * Aliasing
// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function1() -> Result {
//     // --snip--
// }
// fn function2() -> IoResult<()> {
//     // --snip--
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
