#![allow(dead_code)]
#[macro_use]
extern crate lambda_runtime as lambda;
extern crate redis;
extern crate serde;
extern crate serde_dynamo;
extern crate base64;

mod structs;
mod lambda_event_helper;
mod app_config;
mod api_key_repository;
mod policy_document_service;

use crate::app_config::AppConfig;
use crate::api_key_repository::ApiKeyRepository;
use crate::policy_document_service::PolicyDocumentService;

use lambda::LambdaEvent;
use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerResponse,
    ApiGatewayCustomAuthorizerRequestTypeRequest,
};


async fn function_handler(event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>)
                          -> Result<ApiGatewayCustomAuthorizerResponse, lambda_runtime::Error> {
    let app_config = AppConfig::new();
    let repository = ApiKeyRepository::new(&app_config);
    let policy_service = PolicyDocumentService::new(&app_config);
    let fallback_api_key = app_config.anon_api_key.to_string();
    let api_key = lambda_event_helper::get_x_api_key(
        event.payload,
        &fallback_api_key);

    let api_key_option = repository.find_api_key_item(&api_key).await;
    let document = policy_service.generate_policy(&api_key_option);
    return Ok(document);
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    let handler_fn = lambda_runtime::service_fn(function_handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

// fn timed<T>(body: impl FnOnce() -> T) -> (T, std::time::Duration) {
//     let start = std::time::Instant::now();
//     let result = body();
//     let end = std::time::Instant::now();
//     (result, end - start)
// }