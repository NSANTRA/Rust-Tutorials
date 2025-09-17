use crate::back_of_house::Breakfast;

fn deliver_home() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
        }
        fn seat_at_table() {}
    }
    
    pub mod serving {
        fn take_order() {}
        fn server_order() {}
        pub fn take_payment() {}
    }
}

mod back_of_house {
    fn eat_at_home() {
        super::deliver_home();
    }

    pub struct Breakfast {
        pub bread: String,
        fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                bread: String::from(toast),
                fruit: String::from("Mango")
            }
        }
    }
}

fn func() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::serving::take_payment();

    let mut meal = back_of_house::Breakfast::summer(String::from("Wheat"));
    meal.bread = String::from("Whole Grain");

    // The below line will not compile since the fruit field of the Breakfast struct is private.
    // meal.fruit = "Blue berry";
}