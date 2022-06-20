// use rand::Rng, RngCore, SeedableRng};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("Banana"),
            }
        }
    }
}
mod front_house;

fn eat_restaurant() {
    let mut order = back_house::Breakfast::monday("Phở");
    order.toast = String::from("Chicken");

    println!("Order {:?}", order);

    let order1 = back_house::Breakfast {
        toast: String::from("Wheat"),
        fruit: String::from("Apple"),
    };

    // println!("Order1 {:#?}", order1);
    front_house::hosting::add_to_waitlist();
}

fn main() {
    eat_restaurant();
}
