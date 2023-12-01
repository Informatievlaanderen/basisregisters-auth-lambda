use std::str::FromStr;
use lambda_http::aws_lambda_events::apigw::ApiGatewayCustomAuthorizerRequestTypeRequest;

use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::http::{HeaderMap, HeaderName};

fn get_header_value(headers: &HeaderMap, header_name: &str) -> Option<String> {
    // Define a case-insensitive header name
    let header_name = HeaderName::from_str(header_name).ok().unwrap();
    headers
        .get(&header_name)
        .and_then(|header_value| header_value
            .to_str()
            .ok()
            .map(String::from))
}
fn get_query_param_value(query_params: &QueryMap, key: &str) -> Option<String> {
    return query_params
        .iter()
        .find(|(k, _)| k.to_lowercase() == key.to_lowercase())
        .map(|(_, v)| v.to_lowercase());
}
pub fn get_x_api_key(event: ApiGatewayCustomAuthorizerRequestTypeRequest, fallback_key: &str) -> String {
    //Fallback key
    let mut api_key = "".to_string();
    let mut key_found = false;
    let header_name = "x-api-key";
    let query_name= "apikey";

    // Lookup in headers
    match get_header_value(&event.headers, header_name) {
        Some(value) => {
            key_found = true;
            api_key = value;
        }
        None => println!("x-api-key is not found in Headers")
    }

    // Return when found
    if key_found {
        return api_key;
    }

    let query_params = event.query_string_parameters;
    let param = get_query_param_value(&query_params, query_name);
    match param {
        Some(value) => {
            key_found = true;
            api_key = value;
        }
        None => println!("apikey is not found in QueryParameters")
    }

    if !key_found {
        println!("x-api-key is set to anonymous fallback key!");
        api_key = fallback_key.to_string();
    }

    return api_key;
}
