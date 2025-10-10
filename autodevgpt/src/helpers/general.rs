use crossterm::style::Print;

use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;
use crate::helpers::command_line::PrintCommand;

// Extend ai function to encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    //Extend the string to encourage only printing the output
    let msg: String = format!("FUNCTION {}
    INSTRUCTION: You are a function printer. 
    You ONLY print the results of functions. Nothing else. No commentary. 
    Here is the input to the function: {}.
    Print out what the function will return.",
    ai_function_str, func_input);

    return Message {
        role: "system".to_string(),
        content: msg
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for <'a> fn(&'a str) -> &'static str
) -> String {
    //Extend the ai function to encourage specific output
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(
        agent_position,
        agent_operation);

    // Call the LLM
    let ai_response: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(vec![extended_msg.clone()]).await;

    // Handle Success or Try Again
    let llm_response: String = match ai_response {
        Ok(response) => response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
        .await
        .expect("Failed to get response on retry")
        };
    return llm_response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {

        let extended_msg: Message = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());

    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let response: String = ai_task_request(
            "Create a webserver for making stock price api requests".to_string(),
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal
        ).await;

        dbg!(&response);
        assert!(response.len() > 0);
    }
}