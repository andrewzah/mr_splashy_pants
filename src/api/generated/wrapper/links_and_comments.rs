use std::collections::HashMap;

use crate::{
    client,
    api::{
        utils,
        generated::{
            execution::links_and_comments as links_and_comments_execution,
            request::{
                links_and_comments::ApiDel,
                links_and_comments::ApiSubmit,
            },
            response::links_and_comments::ApiSubmitResponse
        },
    },
};

// API is: '/api/submit'
pub async fn wrapper_post_api_submit(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiSubmit,
) -> Result<ApiSubmitResponse, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    links_and_comments_execution::execute_post_api_submit,
  )
  .await
}

// API is: '/api/del'
pub async fn wrapper_post_api_del(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiDel,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    links_and_comments_execution::execute_post_api_del,
  )
  .await
}
