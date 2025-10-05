use crate::models::general::llm::{ ChatCompletion, Message };
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{ HeaderMap, HeaderValue };

//Call Large Language Model (GPT-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    //Extract API key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found!");
    let org_key: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found!");

    //Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    //Create headers
    let mut headers = HeaderMap::new();

    //Create API key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}",api_key)).unwrap()
    );

    //Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(org_key.as_str()).unwrap()
    );

    //Create client
    let client = Client::builder()
    .default_headers(headers)
    .build()
    .unwrap();

    //Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion { 
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1
    };

    //Troubleshooting
    let response_raw = client
    .post(url)
    .json(&chat_completion)
    .send()
    .await
    .unwrap();

    dbg!(response_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {

        let message = Message{
            role: "user".to_string(),
            content: "Hi there, this is a test so just give me short response.".to_string()
        };

        let messages: Vec<Message> = vec!(message);

        call_gpt(messages).await;
    }
}