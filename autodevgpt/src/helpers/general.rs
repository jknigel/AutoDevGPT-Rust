use crossterm::style::Print;
use serde::de::{self, DeserializeOwned};

use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "/web_template/code_template.rs";
const MAIN_RS_PATH: &str = "/web_template/main.rs";
const API_SCHEMA_PATH: &str = "/autodevgpt/schemas/api_schema.json";

// Extend ai function to encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    //Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION {}
    INSTRUCTION: You are a function printer. 
    You ONLY print the results of functions. Nothing else. No commentary. 
    Here is the input to the function: {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    return Message {
        role: "system".to_string(),
        content: msg,
    };
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //Extend the ai function to encourage specific output
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Call the LLM
    let ai_response: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Handle Success or Try Again
    let llm_response: String = match ai_response {
        Ok(response) => response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed to get response on retry"),
    };
    return llm_response;
}

// Performs call to LLM GPT -- Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T =
        serde_json::from_str(&llm_response).expect("Failed to decode LLM response");

    return decoded_response;
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    return fs::read_to_string(path).expect("Failed to read code template");
}

// Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(MAIN_RS_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// Save JSON API Endpoints
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let response: String = ai_task_request(
            "Create a webserver for making stock price api requests".to_string(),
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        dbg!(&response);
        assert!(response.len() > 0);
    }
}
