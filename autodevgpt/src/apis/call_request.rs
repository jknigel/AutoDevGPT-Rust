use crate::models::general::llm::{ Message };
use dotenv::dotenv;
use reqwest::Client;
use std::env;

//Call Large Language Model (GPT-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    //Extract API key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found!");
    let org_key: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found!");

    //Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";
}