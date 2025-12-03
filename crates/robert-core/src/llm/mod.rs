use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, Role},
    Client,
};
use anyhow::Result;
use crate::claude::ClaudeClient;

pub enum LlmProvider {
    OpenAI(Client<async_openai::config::OpenAIConfig>, String),
    Claude(ClaudeClient),
}

pub struct LlmClient {
    provider: LlmProvider,
}

impl LlmClient {
    pub fn new_openai(api_key: String, model: Option<String>) -> Self {
        let config = async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key);
        let client = Client::with_config(config);
        
        Self {
            provider: LlmProvider::OpenAI(client, model.unwrap_or_else(|| "gpt-4o".to_string())),
        }
    }

    pub fn new_claude(binary_path: Option<String>) -> Self {
        let client = if let Some(path) = binary_path {
            ClaudeClient::with_path(path)
        } else {
            ClaudeClient::new()
        };
        
        Self {
            provider: LlmProvider::Claude(client),
        }
    }

    pub fn from_env() -> Self {
        // Default to OpenAI for now unless CLAUDE_ENABLED is set
        if std::env::var("ROBERT_USE_CLAUDE").is_ok() {
            Self::new_claude(None)
        } else {
            let client = Client::new();
            Self {
                provider: LlmProvider::OpenAI(client, "gpt-4o".to_string()),
            }
        }
    }

    pub async fn complete(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String> {
        match &self.provider {
            LlmProvider::OpenAI(client, model) => {
                let mut messages = Vec::new();

                if let Some(sys) = system_prompt {
                    messages.push(ChatCompletionRequestMessage {
                        role: Role::System,
                        content: Some(sys.to_string()),
                        name: None,
                        function_call: None,
                    });
                }

                messages.push(ChatCompletionRequestMessage {
                    role: Role::User,
                    content: Some(prompt.to_string()),
                    name: None,
                    function_call: None,
                });

                let request = CreateChatCompletionRequestArgs::default()
                    .model(model)
                    .messages(messages)
                    .build()?;

                let response = client.chat().create(request).await?;

                let content = response.choices.first()
                    .and_then(|choice| choice.message.content.clone())
                    .unwrap_or_default();

                Ok(content)
            }
            LlmProvider::Claude(client) => {
                client.complete(prompt, system_prompt).await
            }
        }
    }

    pub async fn check_claude_availability() -> bool {
        ClaudeClient::is_available().await
    }
}
