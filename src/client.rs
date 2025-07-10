use reqwest::{Client, Error};
use crate::input::APIInput;
use crate::output::LlmUnifiedResponse;

pub struct OneLLMClient {
    client: Client,
    base_url: String,
}

impl OneLLMClient {
    pub fn new(base_url: String) -> Self {
        OneLLMClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn call_llm(&self, request: &APIInput) -> Result<LlmUnifiedResponse, Error> {
        let url = format!("{}/api/", self.base_url); // Assuming /api/ is the endpoint
        let response = self.client.post(&url).json(request).send().await?;
        response.json::<LlmUnifiedResponse>().await
    }
}