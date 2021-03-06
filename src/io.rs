use std::io::{self, BufRead, Error, ErrorKind};
use std::num::{ParseFloatError, ParseIntError};
use std::collections::HashMap;
use currency_data_proxy::Currency;

#[derive(Debug)]
pub struct UserInput {
    pub from_currency_unit: String,
    pub to_currency_unit: String,
    pub amount: f64
}

pub fn print_welcome_banner() {
    println!(r#"
 _____                                        _____                           _
/ ____|                                      / ____|                         | |
| |    _   _ _ __ _ __ ___ _ __   ___ _   _  | |     ___  _ ____   _____ _ __| |_ ___  _ __
| |   | | | | '__| '__/ _ | '_ \ / __| | | | | |    / _ \| '_ \ \ / / _ | '__| __/ _ \| '__|
| |___| |_| | |  | | |  __| | | | (__| |_| | | |___| (_) | | | \ V |  __| |  | || (_) | |
\_____\__,_|_|  |_|  \___|_| |_|\___|\__, |  \_____\___/|_| |_|\_/ \___|_|   \__\___/|_|
                                     __/ |
                                    |___/
    "#);
  println!("\n\n");
}


pub fn accept_user_input(available_currencies: &HashMap<String, Currency>) -> Result<UserInput, Error> {
    println!("Please Enter 3 letter Currency Code you would like to Convert from");

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


pub fn search_country_name<'a>(search_term: &String, country_map: &'a HashMap<String, Currency>) -> Vec<&'a Currency> {

    let mut results: Vec<&Currency> = Vec::new();
    let search_term_upper: String = search_term.to_uppercase();

    for currency in country_map.values(){
        if let Some(_) = currency.currency_name.to_uppercase()
                                    .find(&search_term_upper) {
            results.push(currency);
        }
    }
    return results;
}


pub fn print_search_results(currencies: &Vec<&Currency>){
    println!("Ok, We matched a few currencies, here they are");

    for (idx, currency) in currencies.iter().enumerate() {
        println!("{}. {}", (idx + 1), currency.currency_name);
    }
}




pub fn print_input_options(){
    println!("Ok, We need a 3 digit currency code e:g USD, GBP, EUR etc \n\n" );

    println!("1. I know the 3 digit currency code");
    println!("2. I dont know the currency code, I would like to Search by Country");
}


pub fn read_input_option(max_options: i8) -> Result<i8, io::Error> {

    println!("Please enter an Option: ");

    let mut user_input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    user_input = stdin_handle.read_line(&mut user_input)
        .map(|_| user_input.trim().to_string()).unwrap();

    if let Ok(x) =  user_input.parse::<i8>() {
        if x < 1 || x > max_options {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid selection"));
        }
        else{
            return Ok(x);
        }
    }else{
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid number entered"));
    }
}

 fn read_search_term() -> Result<String, Error> {
    let mut user_input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    stdin_handle.read_line(&mut user_input)
        .map(|_| user_input.trim().to_string()).and_then(
                                                        |input|  if input.len() < 3 {
                                                            Err(Error::new(ErrorKind::InvalidInput, "Need Atleast 3 characters"))
                                                        } else
                                                         { Ok(input)}
                                                     )
}

fn read_valid_search_term() -> String {

    let mut from_inp = read_search_term();
    while let Err(x) = from_inp {
        println!("Need Atleast 3 characters, Try again");
        from_inp = read_search_term();
    }
    from_inp.unwrap()
}


 fn search_until_match<'a>(search_term: &String, available_currencies: &'a HashMap<String, Currency>) -> Vec<&'a Currency>{

    let mut search_results: Vec<&Currency>  = Vec::new();

    while search_results.is_empty() {
        search_results = search_country_name(&search_term, &available_currencies);
    }

    return search_results;
}

fn get_valid_currency_option<'a>(search_currencies: &'a Vec<&Currency>)  -> &'a String {

    let num_opts = search_currencies.len() as i8;
    let mut result = read_input_option(num_opts);

    while let Err(x) = result {
        println!("Invalid Selection, Try again");
        result = read_input_option(num_opts);
    }

    let select_option: usize =  result.unwrap() as usize;
    let select_currency: &Currency = search_currencies.get(select_option - 1).unwrap();

    return &select_currency.currency_name;
}

// ToDo : Pick up Here
pub fn get_currency_from_search(available_currencies: &HashMap<String, Currency>) -> String {
    let search_term = read_valid_search_term();
    let search_results: Vec<&Currency> = search_until_match(&search_term, &available_currencies);
    print_search_results(&search_results);
    let select_name: &String = get_valid_currency_option(&search_results);

    let mut code: String  = String::new();

    for (key, currency) in available_currencies.iter(){
        if currency.currency_name.eq(select_name) {
            code = key.clone();
            break;
        }
    }
    return code;
}


pub fn accept_user_input_from_search(available_currencies: &HashMap<String, Currency>) -> Result<UserInput, Error>{
    println!("Please Enter any three letters of the currency to convert From: ");
    let from_currency: String = get_currency_from_search(&available_currencies);
    println!("Please Enter any three letters of the currency to convert To: ");
    let to_currency: String = get_currency_from_search(&available_currencies);
    println!("How much would like to Convert?");
    let amt: f64 = (read_amount())?;

    let userInput: UserInput = UserInput {
        amount: amt,
        from_currency_unit: from_currency,
        to_currency_unit: to_currency,
    };

    return Ok(userInput);
}
