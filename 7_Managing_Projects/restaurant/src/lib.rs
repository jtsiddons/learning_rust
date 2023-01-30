mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    pub enum Appetiser {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // Cannot modify private field `seasonal_fruit`
    // meal.seasonal_fruit = String::from("blueberries");

    // Cannot create `Breakfast` without the `summer` function
    // as we cannot access private field `seasonal_fruit`.
    let meal2 = back_of_house::Breakfast {
        toast: String::from("Sourdough"),
        seasonal_fruit: String::from("Raspberres"),
    };

    let appetiser1 = back_of_house::Appetiser::Soup;
}

use crate::front_of_house::hosting;

mod customer {
    pub fn get_seated() {
        hosting::add_to_waitlist();
    }
}
