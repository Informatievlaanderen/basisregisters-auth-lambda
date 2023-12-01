extern crate redis;
extern crate serde;
extern crate serde_dynamo;

use crate::app_config::AppConfig;
use crate::structs::ApiKeyItem;
use redis::Commands;

use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_dynamodb::types::AttributeValue;

pub struct ApiKeyRepository {
    app_config: AppConfig,
}

impl ApiKeyRepository {
    pub fn new(app_config: &AppConfig) -> Self {
        ApiKeyRepository {
            app_config: app_config.clone()
        }
    }

    pub async fn find_api_key_item(&self, api_key: &str) -> Option<ApiKeyItem> {
        // Open redis connection
        let redis_endpoint = self.app_config.redis_endpoint.to_string();
        let client = redis::Client::open(redis_endpoint).unwrap();
        let mut con = client.get_connection().expect("failed to connect to Redis");

        //redis lookup
        let mut record: Option<ApiKeyItem> = None;
        let mut found_in_redis = false;

        let value: Result<String, _> = con.get(&api_key);

        match value {
            Ok(val) => {
                println!("apikey found in Redis.");
                let api_key_item: ApiKeyItem = serde_json::from_str(&val).unwrap();
                record = Some(api_key_item);
                found_in_redis = true;
            }
            Err(_) => {
                println!("apikey doesn't exist in Redis.");
            }
        }

        if !found_in_redis {
            //Lookup in Dynamo
            record = self.get_item_from_dynamo_db(&api_key).await;

            //Save results to redis cache
            if let Some(api_key_item) = &record {
                let json_str = serde_json::to_string(api_key_item).unwrap();
                let _: () = redis::cmd("SET")
                    .arg(&api_key)
                    .arg(&json_str)
                    .query(&mut con)
                    .expect("Couldn't save to apikey to Redis cache");
            }
        }

        return record;
    }

    async fn get_item_from_dynamo_db(&self, api_key: &str) -> Option<ApiKeyItem>
    {
        let config = aws_config::load_from_env().await;
        let database_name = self.app_config.dynamo_db_table_name.to_string();
        let client = DynamoDbClient::new(&config);

        let res = client
            .get_item()
            .table_name(database_name)
            .key("ApiKey", AttributeValue::S(api_key.to_string()))
            .send()
            .await
            .map(|data| {
                if data.item.is_none() {
                    return None;
                }

                let result= serde_dynamo::from_item(data.item.unwrap());
                if result.is_err() {
                    return None;
                }

                let api_key_item: ApiKeyItem = result.unwrap();
                Some(api_key_item)
            })
            .map_err(|err| eprintln!("Error: {:?}", err))
            .ok();

        if res.is_none() {
            return None;
        }
        return res.unwrap();
    }
}