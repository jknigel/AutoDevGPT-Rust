use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) -> String {
        let mut stdout:std::io::Stdout = stdout();

        // Decide on the print colour
        let statement_colour: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        
        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::DarkYellow)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_colour)).unwrap();
        println!("{}", agent_statement);

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut user_response: String = String::new();
        stdin()
            .read_line(&mut user_response)
            .expect("Failed to read response");

        return user_response.trim().to_string();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout:std::io::Stdout = stdout();

    //Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}",question);

    //Reset Color
    stdout.execute(ResetColor).unwrap();

    //Read user input
    let mut user_response:String = String::new();
    stdin()
    .read_line(&mut user_response)
    .expect("Failed to read response");

    //Trim whitespace and return
    return user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_print_agent_message() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "This is a test message for AICall from the Managing Agent.");
    }
}