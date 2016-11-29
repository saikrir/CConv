extern crate hyper;
extern crate serde_json;

pub mod io;
pub mod currency_data_proxy;


//use io::{accept_user_input, print_formatted_results, search_country_name, print_search_results, UserInput};
use io::*;
use currency_data_proxy::*;
use std::error::Error;
use std::collections::HashMap;

fn main() {

    print_welcome_banner();
    let currency_map = match load_all_currencies() {
        Ok(curr_map) => curr_map,
        Err(error) => panic!("Failed to initialize application Error: {}", error.description())
    };

    print_input_options();
    let mut user_input = read_input_option(2);
    while let Err(x) = user_input {
        println!("Invalid Option, Please try again");
        user_input = read_input_option(2);
    }


    let user_option = user_input.unwrap();


    if user_option == 1 {
        let mut user_input_result = accept_user_input(&currency_map);

        while let Err(x) = user_input_result {
            println!("{}\n\nPlease Try again!  \n\n", x.description());
            user_input_result = accept_user_input(&currency_map);
        }

        calculate(user_input_result.unwrap(), &currency_map);

    } else{
        let mut user_input_result = accept_user_input_from_search(&currency_map);

        while let Err(x) = user_input_result {
            println!("{}\n\nPlease Try again!  \n\n", x.description());
            user_input_result = accept_user_input(&currency_map);
        }

        calculate(user_input_result.unwrap(), &currency_map);
    }

    /*

    let input: String = String::from("rup");
    let vec = search_country_name( &input , &currency_map);

    print_search_results(&vec);

    */

    /*

    */
}

fn calculate(user_input: UserInput, currency_map: &HashMap<String, Currency>) {

        if let Ok(con_value) = get_conversion_rate(&user_input.from_currency_unit,
                                                   &user_input.to_currency_unit) {
            let final_val = user_input.amount * con_value;

            print_formatted_results(&user_input, final_val, &currency_map);
        }
}
