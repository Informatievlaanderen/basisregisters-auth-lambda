#[derive(Clone)]
pub struct AppConfig {
    pub region: String,
    pub account: String,
    pub stage_app_id: String,
    pub stage_name: String,
    pub anon_api_key: String,
    pub dynamo_db_table_name: String,
    pub redis_endpoint: String,
    pub plan_abuse_key: String,
    pub plan_standard_key: String,
    pub plan_unlimited_key: String,
}

impl AppConfig {

    pub fn new() -> Self {
        AppConfig {
            region: AppConfig::get_env_variable("REGION"),
            account: AppConfig::get_env_variable("ACCOUNT"),
            stage_app_id: AppConfig::get_env_variable("STAGE_APP_ID"),
            stage_name: AppConfig::get_env_variable("STAGE_NAME"),
            anon_api_key: AppConfig::get_env_variable("ANONAPIKEY"),
            dynamo_db_table_name: AppConfig::get_env_variable("DATABASENAME"),
            plan_abuse_key: AppConfig::get_env_variable("PLAN_ABUSE_KEY"),
            plan_standard_key: AppConfig::get_env_variable("PLAN_STANDARD_KEY"),
            plan_unlimited_key: AppConfig::get_env_variable("PLAN_UNLIMITED_KEY"),
            //TODO: add to env var to terraform file
            redis_endpoint: AppConfig::get_env_variable("REDIS_ENDPOINT"),
        }
    }

    fn get_env_variable(key: &str) -> String {
        std::env::var(key).expect(&format!("{} Env variable is not set.", key))
    }
}