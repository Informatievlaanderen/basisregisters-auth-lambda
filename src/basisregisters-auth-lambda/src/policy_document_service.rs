use base64::{engine::general_purpose, Engine as _};
use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerPolicy,
    ApiGatewayCustomAuthorizerResponse,
    IamPolicyStatement,
};
use serde_json::json;
use crate::app_config::AppConfig;
use crate::structs::{ApiKeyItem, XApiToken, XApiTokenMetaData};

pub struct PolicyDocumentService {
    app_config: AppConfig,
}

impl PolicyDocumentService {
    pub fn new(app_config: &AppConfig) -> Self {
        PolicyDocumentService {
            app_config: app_config.clone()
        }
    }

    fn get_base64_header(&self, api_key_item: &ApiKeyItem) -> (String, String) {
        let item = api_key_item.clone();
        let auth_response_context = XApiToken {
            api_key: item.api_key,
            client_name: item.client_name.unwrap_or("".to_string()),
            metadata: XApiTokenMetaData {
                sync_access: item.sync_access.unwrap_or(false),
                wr_access: item.wr_access.unwrap_or(false),
                tickets: item.tickets.unwrap_or(false),
            },
        };

        let json_str = serde_json::to_string(&auth_response_context).unwrap();
        let json_str_encoded = general_purpose::STANDARD.encode(&json_str);

        return (json_str, json_str_encoded);
    }

    fn get_usage_id_key(&self, plan: &str) -> String {
        if plan == "standard" {
            return self.app_config.plan_standard_key.to_string();
        }

        if plan == "abuse" {
            return self.app_config.plan_abuse_key.to_string();
        }

        if plan == "unlimited" {
            return self.app_config.plan_unlimited_key.to_string();
        }

        return self.app_config.anon_api_key.to_string();
    }

    fn get_policy_document(&self, effect: &str) -> ApiGatewayCustomAuthorizerPolicy {
        let region = self.app_config.region.to_string();
        let account = self.app_config.account.to_string();
        let stage_app_id = self.app_config.stage_app_id.to_string();
        let stage_name = self.app_config.stage_name.to_string();
        let resource = format!(
            "arn:aws:execute-api:{}:{}:{}/{}/*",
            region,
            account,
            stage_app_id,
            stage_name
        );

        let statement = IamPolicyStatement {
            action: vec!["execute-api:Invoke".to_string()],
            resource: vec![resource],
            effect: Some(effect.to_string()),
        };

        ApiGatewayCustomAuthorizerPolicy {
            version: Some("2012-10-17".to_string()),
            statement: vec![statement],
        }
    }

    fn get_context(&self,
                   x_api_key: Option<&str>,
                   x_output: Option<&str>,
                   x_api_token: Option<&str>,
                   reason: Option<&str>,
                   status: Option<i32>) -> serde_json::Value
    {
        if x_api_key.is_none() ||
            x_output.is_none() ||
            x_api_token.is_none() {
            return json!({
                "xReason": reason.map(|s| s.to_string()),
                "xStatus": status,
            });
        }
        let _api_key = x_api_key.unwrap().to_string();
        let _output = x_output.unwrap().to_string();
        let _api_token = x_api_token.unwrap().to_string();

        return json!({
                "xApiKey": _api_key,
                "xOutput": _output,
                "xApiToken": _api_token,
                "xReason": reason.map(|s| s.to_string()),
                "xStatus": status,
            });
    }

    fn generate(&self,
                data: Option<&ApiKeyItem>,
                principal_id: &str,
                effect: &str,
                reason: Option<&str>,
                status: Option<i32>,
    ) -> ApiGatewayCustomAuthorizerResponse
    {
        let policy_document = self.get_policy_document(effect);

        if effect != "allow" {
            let context = self.get_context(
                None,
                None,
                None,
                reason,
                status);

            return ApiGatewayCustomAuthorizerResponse {
                principal_id: None,
                usage_identifier_key: None,
                context,
                policy_document,
            };
        }

        let api_key_item = data.unwrap();
        let plan = api_key_item.plan.to_string();
        let usage_identifier_key = Some(self.get_usage_id_key(&plan));
        let (x_output, x_api_token) = self.get_base64_header(api_key_item);
        let api_key = api_key_item.api_key.to_string();

        let context = self.get_context(
            Some(&api_key),
            Some(&x_output),
            Some(&x_api_token),
            reason,
            status);

        return ApiGatewayCustomAuthorizerResponse {
            principal_id: Some(principal_id.to_string()),
            usage_identifier_key,
            context,
            policy_document,
        };
    }

    pub fn generate_policy(&self, api_key_option: &Option<ApiKeyItem>)
                           -> ApiGatewayCustomAuthorizerResponse {
        let api_key_item = api_key_option.clone();

        if api_key_item.is_none() {
            return self.generate(
                None,
                "user",
                "deny",
                Some("U beschikt niet over de correcte rechten."),
                Some(403));
        }

        let key = api_key_item.unwrap();
        if key.revoked == Some(true) {
            return self.generate(
                None,
                "user",
                "deny",
                Some("Your api-key has been revoked. Please contact support!"),
                Some(403));
        }

        return self.generate(
            Some(&key),
            "user",
            "allow",
            Some("OK"),
            Some(200));
    }
}