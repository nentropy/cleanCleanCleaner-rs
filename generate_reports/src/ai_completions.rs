use anyhow::{anyhow, Result};
use dotenv::dotenv;
use reqwest::Client;
use std::collections::HashMap;
use std::io::{self, Write};
use syn_crabs::{setup_logging};
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use uuid::uuidv4;
use openai::{Client as openai_client, models};
use openai::models::*;
use anthropic::client::ClientBuilder;
use anthropic::config::AnthropicConfig;
use anthropic::types::CompleteRequestBuilder;
use anthropic::{SYSTEM_PROMPT, AI_PROMPT, HUMAN_PROMPT};

static OPENAI_API_KEY: Option<String> = None;
static CLAUDE_API_KEY: Option<String> = None;
static SYS_PROMPT: Str = r#""You are a responsive report writer that receives and returns JSON following the example ```schema```. Be concise
                                and to the point. Always include a "Next Steps" with creative ways to move forward. ```Schema:
                                {Uuid: uuidv4(), data: [], sys_prompt: String, request: json, response: json, metadata: []}"```
                                """#;

#[tokio::main]
pub async fn get_api_keys() -> Result<()> {
    dotenv().ok();
    OPENAI_API_KEY = env::var("OPENAI_API_KEY").ok();
    CLAUDE_API_KEY = env::var("CLAUDE_API_KEY").ok();
    if OPENAI_API_KEY.is_none() && CLAUDE_API_KEY.is_none() {
        return Err(anyhow!("No API key found"));
    }
    Ok(())
}

trait AsModels {
    fn as_models(&self) -> Vec<models::Model>;
}

impl AsModels for Value {
    fn as_models(&self) -> Vec<models::Model> {
        match self {
            Value::Array(models) => models
                .iter()
                .filter_map(|model| match model {
                    Value::Object(model) => Some(models::Model::from(model)),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
    }
}


enum Providers {
    Anthropic,
    OpenAI,
    OpenSourceLlama,
    OpenSourceMistral,
    OLlama
}

impl Providers {
    fn provider_string(&self) -> &str {
        match self {
            Providers::Anthropic => "anthropic",
            Providers::OpenAI => "openai",
            Providers::OpenSourceLlama => "llama3_1",
            Providers::OpenSourceMistral => "Mixtral8b",
            Providers::OLlama => "OLlama",
        }
    }
    
    #[tokio::main]
    async fn list_models(&self) -> Result<Vec<String>> {
        match self {
            Providers::OpenAI => self.list_openai_models().await,
            Providers::Anthropic => self.list_anthropic_models().await,
            // Implement for other providers
            _ => Ok(vec![self.provider_string().to_string()]),
        }
    }
    
    #[derive(Serialize, Deserialize)]
    #[tokio::main]
    async fn list_openai_models(&self) -> Result<Vec<String>> {
        
        let oai_client = Client::new();
        let response = oai_client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", OPENAI_API_KEY.unwrap()))
            .send()
            .await?;
        let models = response.json::<Value>().await?;
        let models = models["data"].as_models();
    }
    
    #[derive(Serialize, Deserialize)]
    #[tokio::main]
    async fn list_claude_models(&self) -> Result<Vec<String>> {
        const DEFAULT_CLAUDE: &str ="claude-3-sonnet-20240229"
        let claude_client = ClientBuilder::default().api_key("CLAUDE_API_KEY".to_string()).build()?;;
        let complete_request = CompleteRequestBuilder::default()
            .prompt(format!("{HUMAN_PROMPT}How many toes do dogs have?\n{AI_PROMPT}"))
            .model(DEFAULT_CLAUDE.to_string())
            .stream(false)
            .stop_sequences(vec![HUMAN_PROMPT.to_string()])
            .build()?;
        let models = response.json::<Value>().await?;
        let models = models["data"].as_models();

        Ok((models))

}
#[tokio::main]
async fn select_model(provider: &provider_string, default_provider: Providers::OpenAI) -> Result<String> {
    if provider {
        let provider = provider.provider_string();
    else if provider == None | "" {
        provider = default_provider.provider_string();
    }
    
    println!("Available models:");
    
    
    for (i, model) in models.as_models().iter().enumerate() {
        let index = i + 1;
        println!("{}. {}", index, model.id);
        model_map.insert(index, model.id.clone());
    }

    loop {
        print!("Select a model (enter the number): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Ok(choice) = input.trim().parse::<usize>() {
            if let Some(model) = model_map.get(&choice) {
                return Ok(model.clone());
            }
        }

        println!("Invalid selection. Please try again.");
        Ok(choice)
    }
}
data_sample: = vec!["A happy testing adventure",
                    "Huzzah this is fun",
                    "What is the meaning of life?",
                    "42"]

let payload: AIPayload = AIPayload::new()
let payload_pack = {Uuid::new::Uuidv4(),
                    data=dataSample,
                    }
let response = Response::new();

struct AIPayload {
    Uuid: uuidv4(),
    Data: Vec<String>,
    SysPrompt: String,
    Request: json,
    Response: json,
    Metadata: Vec<String>
}

impl AIPayload {

    fn new(uuid: Uuidv4, data: Data, request: Request, 
        response: Response, metadata: Metadata) -> Self {
        Self {
            uuid,
            data,
            sys_prompt,
            request,
            response,
            metadata,
        }
    }
}

    #[tokio::main]
    async fn main(data: AIPayload, model:&str, sys_prompt: &str) -> Result<()> {
        //const CLAUDE_DEFAULT: &str="claude-3-sonnet-20240229";
        dotenv().ok();
        init_logging().expect("Failed to initialize logger");
        if provider_string = "OpenAI" {
            //@TODO
        } else if provider_string = "Anthropic" {
        // Anthropic
        pub const DEFAULT_MODEL: &str = CLAUDE_DEFAULT;
        let cfg = AnthropicConfig::new()?;
        let client = Client::try_from(cfg)?;
        log::info!("Client initialized: ", {})?;

        let complete_request = CompleteRequestBuilder::default()
            .prompt(format!("{&SYS_PROMPT}{&data}{AI_PROMPT}"))
            .model(model.to_string())
            .stream_response(true)
            .stop_sequences(vec![HUMAN_PROMPT.to_string()])
            .build()?;

        // Send a completion request.
        let complete_response = client.complete(complete_request).await?;

        println!("completion response: {complete_response:?}");

        Ok(())
    }

 


    let payload = Payload::new(uuid, data, sys_prompt, request, response, metadata);

    /// Build a Vector of data that the  OpenAI API can return in a structured manner.
   
    #[serde(Serialize, Deserialize, Debug)]
    async fn init_payload(&data: Vec<String>) -> Payload {
        let payload = Payload::new(uuid, data, sys_prompt, request, response, metadata);
        let uuid = Uuid::new_v4();
        let data_ser;
        for ix in data.len() {
            data_ser.push(data[ix])
            log::info!("Added data {} to array", data[ix])
        }
        let json_data: json = &data_ser.Serialize().to_json();

        Ok((json_data));
        
    
        
        
        // Aftrr being serialized and converted to JSON, we can now use the `init_payload` function to initialize the payload
        
        
        let req: Request = reqwest::post(sys_pr"https://api.openai.com/v1/chat/completions").Deserialize().await;
        let res: Response = reqwest::get("https://api.openai.com/v1/chat/completions").Deserialize().await;
        let 
        
    }

    let mut report_section = Vec::new();
    let mut_to_summarise = String::new();
            .mut_to_summarise.push_str(topic);
    loop {
        println!("topic: {}", topic);
        io::stdout().flush()?;
        let hash_data = HashMap::new();
        for topic in &data.len() {
            hash.data.insert(topic, ai_res)
            

        }
    }

}