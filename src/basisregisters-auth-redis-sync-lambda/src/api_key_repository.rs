extern crate redis;
extern crate serde;
extern crate serde_dynamo;

use crate::app_config::AppConfig;
use crate::structs::ApiKeyItem;
use redis::Commands;

use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::json;

#[derive()]
pub struct ApiKeyRepository {
    app_config: AppConfig,
}

impl ApiKeyRepository {
    pub fn new(app_config: &AppConfig) -> Self {
        ApiKeyRepository {
            app_config: app_config.clone(),
        }
    }

    async fn get_item_from_dynamo_db(&self, api_key: &str) -> Option<ApiKeyItem>
    {
        let config = aws_config::load_from_env().await;
        let database_name = self.app_config.dynamo_db_table_name.to_string();
        let client = DynamoDbClient::new(&config);

        client
            .get_item()
            .table_name(database_name)
            .key("ApiKey", AttributeValue::S(api_key.to_string()))
            .send()
            .await
            .map(|data| {
                println!("found api key in dynamodb");
                let api_key_item: ApiKeyItem = serde_dynamo::from_item(data.item.unwrap()).unwrap();
                return api_key_item;
            })
            .map_err(|err| eprintln!("Error: {:?}", err))
            .ok()
    }


    fn update_in_redis(&self, api_key_item: &ApiKeyItem) {
        println!("perform update cache");
        let redis_endpoint = self.app_config.redis_endpoint.to_string();
        let client = redis::Client::open(redis_endpoint).unwrap();
        let mut con = client.get_connection().expect("failed to connect to Redis");
        println!("redis connection success");
        let api_key = api_key_item.api_key.to_string();
        let json_str = serde_json::to_string(api_key_item).unwrap();
        println!("json_str {}", json_str);
        let _: () = redis::cmd("SET")
            .arg(&api_key)
            .arg(&json_str)
            .query(&mut con)
            .expect("Couldn't save to apikey to Redis cache");

        println!("updated redis cache {}", json!(api_key_item));
    }

    fn remove_from_redis(&self, api_key: &str) {
        println!("perform update cache");
        let redis_endpoint = self.app_config.redis_endpoint.to_string();
        let client = redis::Client::open(redis_endpoint).unwrap();
        let mut con = client.get_connection().expect("failed to connect to Redis");
        println!("redis connection success");
        let value: Result<String, _> = con.get(api_key);
        match value {
            Ok(_val) => {
                println!("apikey found in Redis.");
                let _: () = redis::cmd("DEL")
                    .arg(&api_key)
                    .query(&mut con)
                    .expect("Couldn't remove to apikey to Redis cache");
            }
            Err(_) => {
                println!("apikey doesn't exist in Redis.");
            }
        }
    }

    pub async fn update_or_delete_api_key(&self, x_api_key: &str) {
        println!("perform dynamo get item");
        let api_key_item = self.get_item_from_dynamo_db(x_api_key).await;
        if let Some(api_key) = api_key_item {
            self.update_in_redis(&api_key);
        } else {
            //check if exist in cache
            self.remove_from_redis(x_api_key);
        }
    }
}