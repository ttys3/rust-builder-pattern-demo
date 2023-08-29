/// Configuration for OpenAI API
#[derive(Debug, Default)]
pub struct OpenAIConfig {
    api_base: String,
    api_key: String,
    org_id: String,
}

impl Clone for OpenAIConfig {
    fn clone(&self) -> Self {
        println!("Cloning OpenAIConfig: {}", file!());
        Self {
            api_base: self.api_base.clone(),
            api_key: self.api_key.clone(),
            org_id: self.org_id.clone(),
        }
    }
}

impl OpenAIConfig {
    /// Create client with default [OPENAI_API_BASE] url and default API key from OPENAI_API_KEY env var
    pub fn new() -> Self {
        Self {
            api_base: "https://example.com/v1".to_string(),
            .. Default::default()
        }
    }

    /// To use a different organization id other than default
    pub fn with_org_id<S: Into<String>>(mut self, org_id: S) -> Self {
        self.org_id = org_id.into();
        self
    }

    /// To use a different API key different from default OPENAI_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = api_key.into();
        self
    }

    /// To use a API base url different from default [OPENAI_API_BASE]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    pub fn org_id(&self) -> &str {
        &self.org_id
    }
}