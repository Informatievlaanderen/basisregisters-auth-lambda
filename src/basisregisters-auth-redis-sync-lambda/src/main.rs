// #![allow(dead_code)]
// #[macro_use]
// extern crate lambda_runtime as lambda;
// extern crate redis;
// extern crate serde;
// extern crate serde_dynamo;
// extern crate base64;

mod structs;
mod app_config;
mod api_key_repository;

use crate::app_config::AppConfig;
use crate::api_key_repository::ApiKeyRepository;

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let x_api_key = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("x-api-key"));

    if let Some(api_key) = x_api_key {
        println!("received apikey {}", api_key.to_string());
        let app_config = AppConfig::new();
        let repository = ApiKeyRepository::new(&app_config);
        repository.update_or_delete_api_key(api_key).await;
    }

    let message = json!({"request": "accepted"}).to_string();

    let resp = Response::builder()
        .status(202)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}