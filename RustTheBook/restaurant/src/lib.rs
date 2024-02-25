mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {

    pub struct Breakfast {
        //構造体は公開，フィールドは非公開
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub use self::front_of_house::hosting; //hostingまで記載するのは慣例

// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist(); //関数がローカルで定義されていないことを明示できる
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt;
use std::io::Result as IoResult;
//これは例外
fn function1() -> fmt::Result {
    // --snip--
    // （略）
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    // （略）
    IoResult::Ok(())
}
