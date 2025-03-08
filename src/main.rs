use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use clipboard::{ClipboardContext, ClipboardProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let staged_diff_text = run_git_diff("--cached")?;
    let unstaged_diff_text = run_git_diff("")?;

    let diff_text = if staged_diff_text.is_empty() {
        if unstaged_diff_text.is_empty() {
            println!("No staged or unstaged changes detected. Exiting.");
            return Ok(());
        }
        unstaged_diff_text
    } else if !unstaged_diff_text.is_empty() {
        println!("Both staged and unstaged changes detected.");
        if confirm_user_input("Include both staged and unstaged changes in commit message? (y/n): ")? {
            format!("{}\n{}", staged_diff_text, unstaged_diff_text)
        } else {
            staged_diff_text
        }
    } else {
        staged_diff_text
    };

    let original_diff_text = diff_text.clone();

    let mut prompt = format!(
        "Generate a conventional commit message referencing changed files:\n\n<GIT_DIFF>",
    );

    let model = "gpt-4o-mini";
    let client = Client::default();

    loop {
        let actual_prompt = prompt.replace("<GIT_DIFF>", &original_diff_text);

        let chat_req = ChatRequest::new(vec![
            ChatMessage::system("Provide a concise conventional commit message without markdown formatting."),
            ChatMessage::user(&actual_prompt),
        ]);

        println!("\nQuerying ChatGPT for commit message...");
        let chat_res = client.exec_chat(model, chat_req, None).await?;
        let commit_message = chat_res.content_text_as_str().unwrap_or("No response.").replace('`', "");

        log_output("output_cc_suggestions.txt", &commit_message)?;

        println!("\nSuggested commit message:\n{}", commit_message);

        if confirm_user_input("\nAccept this commit message? (y/n): ")? {
            if confirm_user_input("\nCopy commit message to clipboard? (y/n): ")? {
                let mut clipboard: ClipboardContext = ClipboardProvider::new()?;
                clipboard.set_contents(commit_message.to_owned())?;
                println!("Commit message copied to clipboard.");
            } else {
                println!("Commit message not copied.");
            }
            break;
        } else {
            println!("Refining prompt for a better commit message...");
            let refinement_req = ChatRequest::new(vec![
                ChatMessage::system("Suggest an improved prompt to obtain a better conventional commit message following conventional commit specifications."),
                ChatMessage::user(&actual_prompt),
            ]);

            let refinement_res = client.exec_chat(model, refinement_req, None).await?;
            let refined_prompt_template = refinement_res.content_text_as_str().unwrap_or(&prompt).replace("<GIT_DIFF>", &original_diff_text);

            log_output("output_cc_prompts.txt", &refined_prompt_template)?;

            prompt = refinement_res.content_text_as_str().unwrap_or(&prompt).to_string();

            println!("\nRefined prompt used:\n{}", prompt);
        }
    }

    Ok(())
}

fn run_git_diff(args: &str) -> Result<String, Box<dyn std::error::Error>> {
    let args_vec: Vec<&str> = if args.is_empty() {
        vec!["diff"]
    } else {
        vec!["diff", args]
    };
    let output = Command::new("git").args(&args_vec).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn confirm_user_input(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn log_output(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;
    writeln!(file, "\n----------------------\n{}\n", content)?;
    Ok(())
}

