mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn delivery_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::delivery_order();
    }

    fn cook_order() {}
}

// If we use pub before a struct definition, we make the struct public, but the struct’s fields will
// still be private.

// If we make an enum public, all of its variants are then public.