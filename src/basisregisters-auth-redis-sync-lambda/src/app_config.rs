#[derive(Clone)]
pub struct AppConfig {
    pub dynamo_db_table_name: String,
    pub redis_endpoint: String,
}

impl AppConfig {

    pub fn new() -> Self {
        AppConfig {
            dynamo_db_table_name: AppConfig::get_env_variable("DATABASENAME"),
            redis_endpoint: AppConfig::get_env_variable("REDIS_ENDPOINT"),
        }
    }

    fn get_env_variable(key: &str) -> String {
        std::env::var(key).expect(&format!("{} Env variable is not set.", key))
    }
}