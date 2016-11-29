use std::io::Read;
use hyper::{Client};
use hyper::error::Error;
use hyper::client::response::Response;
use serde_json::{Value,from_str};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Currency {
    pub currency_name: String,
    pub currency_symbol: Option<String>
}

const ALL_CURRENCIES_URL:&'static str = "http://free.currencyconverterapi.com/api/v3/currencies";

const CURRENCY_CONVERSION_URL: &'static str = "http://free.currencyconverterapi.com/api/v3/convert?q={}&compact=ultra";

pub fn load_all_currencies() -> Result<HashMap<String, Currency>, Error>{
    let client = Client::new();
    let mut response: Response = try!(client.get(ALL_CURRENCIES_URL).send());
    let mut response_string: String = String::new();
    response.read_to_string(&mut response_string);
    Ok(parse_data_response(&response_string))
}

fn parse_data_response(json_str: &str) -> HashMap<String, Currency> {

    let json_data: Value = from_str(json_str).expect("Failed to decode json payload");

    let records = json_data.as_object()
                        .and_then(|json_root| json_root.get("results"))
                        .and_then(|result_root| result_root.as_object()).expect("Problem Parsing JSON");

    let mut currency_map: HashMap<String, Currency> = HashMap::new();

    for record in records.values() {
        let r = record.as_object().unwrap();
        let id: String = r.get("id").and_then(|val| val.as_str())
                            .and_then(|val| Some(val.to_owned()))
                            .expect("Invalid Currency Key");

        let currency_name: String = r.get("currencyName").and_then(|val| val.as_str())
                                                 .and_then(|val| Some(val.to_owned()))
                                                 .expect("Invalid Currency Name encountered");

        let currency_symbol: Option<String> =  r.get("currencySymbol")
                                                .and_then(|val| val.as_str())
                                                .and_then(|val| Some(val.to_owned()));

        currency_map.insert(id, Currency{
            currency_name: currency_name,
            currency_symbol: currency_symbol
        });
    }

    return currency_map;
}

pub fn get_conversion_rate(from_symbol: &str, to_symbol: &str) -> Result<f64, Error>{
    let conversion_symbol = format!("{}_{}", from_symbol, to_symbol);
    let conversion_url = format!("http://free.currencyconverterapi.com//api/v3/convert?q={}&compact=ultra", conversion_symbol);

    let client = Client::new();
    let mut response: Response = try!(client.get(&conversion_url).send());
    let mut response_string: String = String::new();
    response.read_to_string(&mut response_string);


    let val: Value  = from_str(&response_string).expect("Unable to parse Currency Conversion Response");

    let final_value = val.as_object().and_then(|root_obj| root_obj.get(&conversion_symbol)).and_then(|obj| obj.as_f64());

    Ok(final_value.unwrap())
}
