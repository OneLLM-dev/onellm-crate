use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Model {
    Gpt4_1,
    Gpt4_1Mini,
    Gpt4_1Nano,
    GptO3,
    GptO4Mini,
    ClaudeOpus4,
    ClaudeSonnet4,
    ClaudeHaiku3_5,
    ClaudeOpus3,
    ClaudeSonnet3_7,
    ClaudeHaiku3,
    DeepSeekR1,
    DeepSeekV3,
    Gemini25FlashPreview,
    Gemini25ProPreview,
    Gemini20Flash,
    Gemini20FlashLite,
    Gemini15Flash,
    Gemini15Flash8B,
    Gemini15Pro,
}

impl Model {
    pub fn name(&self) -> &str {
        match self {
            Model::Gpt4_1 => "GPT-4.1",
            Model::Gpt4_1Mini => "GPT-4.1-Mini",
            Model::Gpt4_1Nano => "GPT-4.1-Nano",
            Model::GptO3 => "GPT-o3",
            Model::GptO4Mini => "GPT-o4-mini",
            Model::ClaudeOpus4 => "Opus-4",
            Model::ClaudeSonnet4 => "Sonnet-4",
            Model::ClaudeHaiku3_5 => "Haiku-3.5",
            Model::ClaudeOpus3 => "Opus-3",
            Model::ClaudeSonnet3_7 => "Sonnet-3.7",
            Model::ClaudeHaiku3 => "Haiku-3",
            Model::DeepSeekR1 => "DeepSeek-Reasoner",
            Model::DeepSeekV3 => "DeepSeek-Chat",
            Model::Gemini25FlashPreview => "Gemini25FlashPreview",
            Model::Gemini25ProPreview => "2.5-Pro-preview",
            Model::Gemini20Flash => "2.0-Flash",
            Model::Gemini20FlashLite => "2.0-Flash-lite",
            Model::Gemini15Flash => "1.5-Flash",
            Model::Gemini15Flash8B => "1.5-Flash-8B",
            Model::Gemini15Pro => "1.5-Pro",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub r#type: String,
    pub function: Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfig {
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: u32,
    pub candidate_count: u32,
    pub max_output_tokens: u32,
    pub stop_sequences: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseFormat {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIInput {
    pub endpoint: String,
    // Common fields
    pub model: Model,
    pub temperature: Option<f64>,
    pub stream: Option<bool>,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub top_p: f64,
    pub stop_sequences: Option<Vec<String>>,
    pub tools: Option<Vec<Tool>>,

    // Gemini
    #[serde(rename = "contents")]
    pub contents: Option<Vec<Content>>,
    #[serde(rename = "safety_settings")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(rename = "generation_config")]
    pub generation_config: Option<GenerationConfig>,

    // OpenAI, DeepSeek
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f64>,

    // OpenAI
    pub n: Option<u32>,
    #[serde(rename = "response_format")]
    pub response_format: Option<ResponseFormat>,
    pub seed: Option<u32>,
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    pub user: Option<String>,

    // DeepSeek
    pub logprobs: Option<bool>,
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<u32>,

    // Claude
    pub system: Option<String>,
    #[serde(rename = "top_k")]
    pub top_k: Option<u32>,
}

impl APIInput {
    pub fn new(
        endpoint: String,
        model: Model,
        messages: Vec<Message>,
        max_tokens: u32,
    ) -> Self {
        Self {
            endpoint,
            model,
            messages,
            max_tokens,
            temperature: None,
            stream: None,
            top_p: 1.0, // Default value
            stop_sequences: None,
            tools: None,
            contents: None,
            safety_settings: None,
            generation_config: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            response_format: None,
            seed: None,
            tool_choice: None,
            user: None,
            logprobs: None,
            top_logprobs: None,
            system: None,
            top_k: None,
        }
    }

        pub async fn send(self) -> anyhow::Result<crate::output::LlmUnifiedResponse> {
        let client = crate::client::OneLLMClient::new(self.endpoint.clone());
        let response = client.call_llm(&self).await?;
        Ok(response)
    }
}
