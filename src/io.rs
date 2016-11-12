use std::io::{self, BufRead, Error, ErrorKind};
use std::num::ParseFloatError;
use std::collections::HashMap;
use currency_data_proxy::Currency;

#[derive(Debug)]
pub struct UserInput {
    pub from_currency_unit: String,
    pub to_currency_unit: String,
    pub amount: f64
}


pub fn accept_user_input(available_currencies: &HashMap<String, Currency>) -> Result<UserInput, Error> {
    println!("Please Enter 3 letter Currency Code you would like to Convert from? ");

    let from_currency = (read_currency(&available_currencies))?;

    println!("Please Enter 3 letter Currency Code you would like to Convert to? ");

    let to_currency = (read_currency(&available_currencies))?;


    if from_currency == to_currency {
        let validation_error: Error = Error::new(ErrorKind::InvalidInput,"Currency conversion needs two different currencies");
        return Err(validation_error);
    }


    println!("How much would like to Convert?");

    let amt: f64 = (read_amount())?;


    let userInput: UserInput = UserInput {
        amount: amt,
        from_currency_unit: from_currency,
        to_currency_unit: to_currency,
    };

    return Ok(userInput);
}

fn read_currency(available_currencies: &HashMap<String, Currency>) -> Result<String, Error> {
    let mut from_currency: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    from_currency = stdin_handle.read_line(&mut from_currency)
        .and_then(|_| Ok(from_currency.trim().to_string().to_uppercase()))
        .expect("Could not read From Currency Information");

    if from_currency.len() < 3 {
        return Err(Error::new(ErrorKind::InvalidInput, "Please enter 3 Digits"));
    }

    if !available_currencies.contains_key(&from_currency) {
        return Err(Error::new(ErrorKind::InvalidInput, "This Currency Does not exist"));
    }

    return Ok(from_currency);
}


pub fn read_amount() -> Result<f64, Error> {
    let mut user_input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    user_input = stdin_handle.read_line(&mut user_input)
        .map(|_| user_input.trim().to_string()).unwrap();

    user_input.parse::<f64>().map_err(|e| Error::new(ErrorKind::InvalidInput, "Invalid amount"))
}

pub fn print_formatted_results(user_input: &UserInput, amt: f64, available_currencies: &HashMap<String, Currency>){
    let from_curr_obj = available_currencies.get(&user_input.from_currency_unit).unwrap();
    let to_curr_obj = available_currencies.get(&user_input.to_currency_unit).unwrap();

    print!("{}", user_input.amount);
    if let Some(ref fc_sym) = from_curr_obj.currency_symbol {
        print!(" {} ({})",fc_sym, from_curr_obj.currency_name);
    }else{
        print!(" {}", from_curr_obj.currency_name);
    }

    print!(" is {}", amt );

    if let Some(ref tc_sym) = to_curr_obj.currency_symbol {
        print!(" {} ({})",tc_sym, to_curr_obj.currency_name);
    }else{
        print!(" {}", to_curr_obj.currency_name);
    }

    println!("\nPlease serving you!!");

}
