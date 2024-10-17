
/*
fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
*/

/*
src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn seat_at_table() -> &'static str {
            "sit down please"
        }
    }

    // Реекспортуємо функцію eat_at_restaurant
    pub fn eat_at_restaurant() -> &'static str {
        hosting::seat_at_table(); // Можемо викликати seat_at_table тут, якщо потрібно
        "yummy yummy!"
    }

    // Реекспорт функцій
    pub use hosting::seat_at_table;
}


src/main.rs

fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");

    println!("Success!");
}


*/

/*
pub use hosting::seat_at_table;: Це реекспортує функцію seat_at_table з модуля hosting у кореневий модуль бібліотеки, що дозволяє вам викликати її з hello_package::hosting::seat_at_table().
eat_at_restaurant: Залишається доступною з кореневого модуля.
*/