extern crate hyper;
extern crate serde_json;

pub mod io;
pub mod currency_data_proxy;


use io::{accept_user_input, print_formatted_results, UserInput};
use currency_data_proxy::{load_all_currencies, get_conversion_rate};
use std::error::Error;


fn main() {
    let currency_map = match load_all_currencies() {
        Ok(curr_map) => curr_map,
        Err(error) => panic!("Failed to initialize application Error: {}", error.description())
    };

    let mut user_input_result = accept_user_input(&currency_map);

    while let Err(x) = user_input_result {
        println!("{}\n\nPlease Try again!  \n\n", x.description());
        user_input_result = accept_user_input(&currency_map);
    }

    let user_input: UserInput = user_input_result.unwrap();

    if let Ok(con_value) = get_conversion_rate(&user_input.from_currency_unit,
                                               &user_input.to_currency_unit) {
        let final_val = user_input.amount * con_value;

        print_formatted_results(&user_input, final_val, &currency_map);
    }
}
