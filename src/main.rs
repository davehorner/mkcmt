#![doc = include_str!("../README.md")]

// Prevent both Tokio implementations from being enabled at the same time.
#[cfg(all(feature = "uses_tokio_rt", feature = "uses_tokio_plain"))]
compile_error!(
    "Features 'uses_tokio_rt' and 'uses_tokio_plain' cannot be enabled simultaneously. Please choose one."
);

// --- Conditional Imports ---

// When using the runtime-enabled Tokio, alias the dependency as `tokio`.
#[cfg(feature = "uses_tokio_rt")]
use tokio_rt as tokio;

// Clipboard support via arboard.
#[cfg(feature = "uses_arboard")]
use arboard::Clipboard;

// Async chat functionality via GenAI is only available with tokio_rt.
#[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
use genai::Client;
#[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
use genai::chat::{ChatMessage, ChatRequest};

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

// --- CLI Argument Parsing ---
use clap::Parser;

/// mkcmt: A tool for conventional commits and commit message recovery.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Activate recovery mode to restore a previous commit message (from HEAD@{1}).
    #[arg(short = 'r', long)]
    recovery: bool,

    /// Perform a soft reset on the current branch.
    #[arg(short = 's', long = "soft-reset")]
    soft: bool,

    /// Display the current (last) commit message.
    #[arg(short = 'c', long = "current")]
    current: bool,
}

// --- Common Helper Functions ---

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
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "\n----------------------\n{}\n", content)?;
    Ok(())
}

/// Gather the Git diff text.
fn get_diff_text() -> Result<String, Box<dyn std::error::Error>> {
    let staged_diff_text = run_git_diff("--cached")?;
    let unstaged_diff_text = run_git_diff("")?;

    let diff_text = if staged_diff_text.is_empty() {
        if unstaged_diff_text.is_empty() {
            println!("No staged or unstaged changes detected. Exiting.");
            return Ok(String::new());
        } else {
            unstaged_diff_text
        }
    } else if !unstaged_diff_text.is_empty() {
        println!("Both staged and unstaged changes detected.");
        if confirm_user_input(
            "Include both staged and unstaged changes in commit message? (y/n): ",
        )? {
            format!("{}\n{}", staged_diff_text, unstaged_diff_text)
        } else {
            staged_diff_text
        }
    } else {
        staged_diff_text
    };

    Ok(diff_text)
}

// --- Chat Loop Implementations ---

// Asynchronous chat loop when using tokio_rt and GenAI.
#[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
async fn chat_loop(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
    let prompt_template =
        "Generate a conventional commit message referencing changed files:\n\n<GIT_DIFF>";
    let model = "gpt-4o-mini";
    let client = Client::default();
    let original_diff_text = diff_text.clone();

    loop {
        let actual_prompt = prompt_template.replace("<GIT_DIFF>", &original_diff_text);

        let chat_req = ChatRequest::new(vec![
            ChatMessage::system(
                "Provide a concise conventional commit message without markdown formatting.",
            ),
            ChatMessage::user(&actual_prompt),
        ]);

        println!("\nQuerying ChatGPT for commit message...");
        let chat_res = client.exec_chat(model, chat_req, None).await?;
        let commit_message = chat_res
            .content_text_as_str()
            .unwrap_or("No response.")
            .replace('`', "");

        log_output("output_cc_suggestions.txt", &commit_message)?;
        println!("\nSuggested commit message:\n{}", commit_message);

        if confirm_user_input("\nAccept this commit message? (y/n): ")? {
            if confirm_user_input("\nCopy commit message to clipboard? (y/n): ")? {
                #[cfg(feature = "uses_arboard")]
                {
                    let mut clipboard = Clipboard::new()?;
                    clipboard.set_text(commit_message.to_owned())?;
                    println!("Commit message copied to clipboard.");
                }
                #[cfg(not(feature = "uses_arboard"))]
                {
                    println!(
                        "Clipboard functionality not enabled (feature 'uses_arboard' is off)."
                    );
                }
            } else {
                println!("Commit message not copied.");
            }
            break;
        } else {
            println!("Refining prompt for a better commit message...");
            let refinement_req = ChatRequest::new(vec![
                ChatMessage::system(
                    "Suggest an improved prompt to obtain a better conventional commit message following conventional commit specifications.",
                ),
                ChatMessage::user(&actual_prompt),
            ]);

            let refinement_res = client.exec_chat(model, refinement_req, None).await?;
            let refined_prompt_template = refinement_res
                .content_text_as_str()
                .unwrap_or(prompt_template)
                .replace("<GIT_DIFF>", &original_diff_text);

            log_output("output_cc_prompts.txt", &refined_prompt_template)?;
            println!("\nRefined prompt used:\n{}", refined_prompt_template);
        }
    }

    Ok(())
}

// Asynchronous stub when using tokio_rt but GenAI is disabled.
#[cfg(all(feature = "uses_tokio_rt", not(feature = "uses_genai")))]
async fn chat_loop(_diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Async chat functionality is disabled because the 'uses_genai' feature is off.");
    Ok(())
}

// Synchronous chat loop when using the plain Tokio dependency.
#[cfg(feature = "uses_tokio_plain")]
fn chat_loop(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Diff text:\n{}", diff_text);
    println!("Async chat functionality is disabled (using tokio_plain).");
    Ok(())
}

// --- Recovery Mode ---

/// Recovers the commit message from HEAD@{1}, prints it, and prompts the user to copy it to the clipboard.
///
/// This function executes `git show -s --format=%B HEAD@{1}` to obtain the commit
/// message, prints the message to the terminal, and then asks the user if they wish to copy it.
fn recover_commit_message() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["show", "-s", "--format=%B", "HEAD@{1}"])
        .output()?;

    if !output.status.success() {
        eprintln!("Error retrieving commit message");
        std::process::exit(1);
    }

    let commit_message = String::from_utf8_lossy(&output.stdout);
    println!("Recovered commit message:\n\n{}", commit_message);

    if confirm_user_input("\nCopy commit message to clipboard? (y/n): ")? {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(commit_message.to_owned())?;
        println!("Commit message copied to clipboard.");
    } else {
        println!("Commit message not copied.");
    }
    Ok(())
}

// --- New: Soft Reset Mode ---

/// Performs a soft reset on the current branch.
fn perform_soft_reset() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["reset", "--soft", "HEAD~1"])
        .output()?;

    if !output.status.success() {
        eprintln!("Error performing soft reset");
        std::process::exit(1);
    }

    println!("Soft reset performed on the current branch.");
    Ok(())
}

/// Displays the current (last) commit message.
fn display_last_commit_message() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["show", "-s", "--format=%B", "HEAD"])
        .output()?;

    if !output.status.success() {
        eprintln!("Error retrieving last commit message");
        std::process::exit(1);
    }

    let commit_message = String::from_utf8_lossy(&output.stdout);
    println!("Last commit message:\n\n{}", commit_message);
    Ok(())
}

// --- Main Functions ---

// When using tokio_rt, parse CLI arguments, check for additional flags, then run the async main.
#[cfg(feature = "uses_tokio_rt")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.recovery {
        recover_commit_message()?;
        return Ok(());
    } else if args.soft {
        perform_soft_reset()?;
        return Ok(());
    } else if args.current {
        display_last_commit_message()?;
        return Ok(());
    }

    let diff_text = get_diff_text()?;
    if diff_text.is_empty() {
        return Ok(());
    }
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main(diff_text))
}

#[cfg(feature = "uses_tokio_rt")]
async fn async_main(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
    chat_loop(diff_text).await
}

// When using tokio_plain, parse CLI arguments and check for additional flags, then run synchronously.
#[cfg(feature = "uses_tokio_plain")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.recovery {
        recover_commit_message()?;
        return Ok(());
    } else if args.soft {
        perform_soft_reset()?;
        return Ok(());
    } else if args.current {
        display_last_commit_message()?;
        return Ok(());
    }

    let diff_text = get_diff_text()?;
    if diff_text.is_empty() {
        return Ok(());
    }
    chat_loop(diff_text)
}

// #![doc = include_str!("../README.md")]

// // Prevent both Tokio implementations from being enabled at the same time.
// #[cfg(all(feature = "uses_tokio_rt", feature = "uses_tokio_plain"))]
// compile_error!(
//     "Features 'uses_tokio_rt' and 'uses_tokio_plain' cannot be enabled simultaneously. Please choose one."
// );

// // --- Conditional Imports ---

// // When using the runtime-enabled Tokio, alias the dependency as `tokio`.
// #[cfg(feature = "uses_tokio_rt")]
// use tokio_rt as tokio;

// // Clipboard support via arboard.
// #[cfg(feature = "uses_arboard")]
// use arboard::Clipboard;

// // Async chat functionality via GenAI is only available with tokio_rt.
// #[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
// use genai::Client;
// #[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
// use genai::chat::{ChatMessage, ChatRequest};

// use std::fs::OpenOptions;
// use std::io::{self, Write};
// use std::process::Command;

// // --- CLI Argument Parsing ---
// use clap::Parser;

// /// mkcmt: A tool for conventional commits and commit message recovery.
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Activate recovery mode to restore a previous commit message.
//     #[arg(short, long)]
//     recovery: bool,
// }

// // --- Common Helper Functions ---

// fn run_git_diff(args: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let args_vec: Vec<&str> = if args.is_empty() {
//         vec!["diff"]
//     } else {
//         vec!["diff", args]
//     };
//     let output = Command::new("git").args(&args_vec).output()?;
//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }

// fn confirm_user_input(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
//     print!("{}", prompt);
//     io::stdout().flush()?;
//     let mut input = String::new();
//     io::stdin().read_line(&mut input)?;
//     Ok(input.trim().eq_ignore_ascii_case("y"))
// }

// fn log_output(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(filename)?;
//     writeln!(file, "\n----------------------\n{}\n", content)?;
//     Ok(())
// }

// /// Gather the Git diff text.
// fn get_diff_text() -> Result<String, Box<dyn std::error::Error>> {
//     let staged_diff_text = run_git_diff("--cached")?;
//     let unstaged_diff_text = run_git_diff("")?;

//     let diff_text = if staged_diff_text.is_empty() {
//         if unstaged_diff_text.is_empty() {
//             println!("No staged or unstaged changes detected. Exiting.");
//             return Ok(String::new());
//         } else {
//             unstaged_diff_text
//         }
//     } else if !unstaged_diff_text.is_empty() {
//         println!("Both staged and unstaged changes detected.");
//         if confirm_user_input(
//             "Include both staged and unstaged changes in commit message? (y/n): ",
//         )? {
//             format!("{}\n{}", staged_diff_text, unstaged_diff_text)
//         } else {
//             staged_diff_text
//         }
//     } else {
//         staged_diff_text
//     };

//     Ok(diff_text)
// }

// // --- Chat Loop Implementations ---

// // Asynchronous chat loop when using tokio_rt and GenAI.
// #[cfg(all(feature = "uses_tokio_rt", feature = "uses_genai"))]
// async fn chat_loop(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
//     let prompt_template =
//         "Generate a conventional commit message referencing changed files:\n\n<GIT_DIFF>";
//     let model = "gpt-4o-mini";
//     let client = Client::default();
//     let original_diff_text = diff_text.clone();

//     loop {
//         let actual_prompt = prompt_template.replace("<GIT_DIFF>", &original_diff_text);

//         let chat_req = ChatRequest::new(vec![
//             ChatMessage::system(
//                 "Provide a concise conventional commit message without markdown formatting.",
//             ),
//             ChatMessage::user(&actual_prompt),
//         ]);

//         println!("\nQuerying ChatGPT for commit message...");
//         let chat_res = client.exec_chat(model, chat_req, None).await?;
//         let commit_message = chat_res
//             .content_text_as_str()
//             .unwrap_or("No response.")
//             .replace('`', "");

//         log_output("output_cc_suggestions.txt", &commit_message)?;
//         println!("\nSuggested commit message:\n{}", commit_message);

//         if confirm_user_input("\nAccept this commit message? (y/n): ")? {
//             if confirm_user_input("\nCopy commit message to clipboard? (y/n): ")? {
//                 #[cfg(feature = "uses_arboard")]
//                 {
//                     let mut clipboard = Clipboard::new()?;
//                     clipboard.set_text(commit_message.to_owned())?;
//                     println!("Commit message copied to clipboard.");
//                 }
//                 #[cfg(not(feature = "uses_arboard"))]
//                 {
//                     println!(
//                         "Clipboard functionality not enabled (feature 'uses_arboard' is off)."
//                     );
//                 }
//             } else {
//                 println!("Commit message not copied.");
//             }
//             break;
//         } else {
//             println!("Refining prompt for a better commit message...");
//             let refinement_req = ChatRequest::new(vec![
//                 ChatMessage::system(
//                     "Suggest an improved prompt to obtain a better conventional commit message following conventional commit specifications.",
//                 ),
//                 ChatMessage::user(&actual_prompt),
//             ]);

//             let refinement_res = client.exec_chat(model, refinement_req, None).await?;
//             let refined_prompt_template = refinement_res
//                 .content_text_as_str()
//                 .unwrap_or(prompt_template)
//                 .replace("<GIT_DIFF>", &original_diff_text);

//             log_output("output_cc_prompts.txt", &refined_prompt_template)?;
//             println!("\nRefined prompt used:\n{}", refined_prompt_template);
//         }
//     }

//     Ok(())
// }

// // Asynchronous stub when using tokio_rt but GenAI is disabled.
// #[cfg(all(feature = "uses_tokio_rt", not(feature = "uses_genai")))]
// async fn chat_loop(_diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
//     println!("Async chat functionality is disabled because the 'uses_genai' feature is off.");
//     Ok(())
// }

// // Synchronous chat loop when using the plain Tokio dependency.
// #[cfg(feature = "uses_tokio_plain")]
// fn chat_loop(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
//     println!("Diff text:\n{}", diff_text);
//     println!("Async chat functionality is disabled (using tokio_plain).");
//     Ok(())
// }

// // --- Recovery Mode ---

// /// Recovers the commit message from HEAD@{1}, prints it, and prompts the user to copy it to the clipboard.
// ///
// /// This function executes `git show -s --format=%B HEAD@{1}` to obtain the commit
// /// message, prints the message to the terminal, and then asks the user if they wish to copy it.
// fn recover_commit_message() -> Result<(), Box<dyn std::error::Error>> {
//     let output = Command::new("git")
//         .args(&["show", "-s", "--format=%B", "HEAD@{1}"])
//         .output()?;

//     if !output.status.success() {
//         eprintln!("Error retrieving commit message");
//         std::process::exit(1);
//     }

//     let commit_message = String::from_utf8_lossy(&output.stdout);
//     println!("Recovered commit message:\n\n{}", commit_message);

//     if confirm_user_input("\nCopy commit message to clipboard? (y/n): ")? {
//         let mut clipboard = Clipboard::new()?;
//         clipboard.set_text(commit_message.to_owned())?;
//         println!("Commit message copied to clipboard.");
//     } else {
//         println!("Commit message not copied.");
//     }
//     Ok(())
// }

// // --- Main Functions ---

// // When using tokio_rt, parse CLI arguments, check for recovery mode, then run the async main.
// #[cfg(feature = "uses_tokio_rt")]
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Args::parse();
//     if args.recovery {
//         recover_commit_message()?;
//         return Ok(());
//     }
//     let diff_text = get_diff_text()?;
//     if diff_text.is_empty() {
//         return Ok(());
//     }
//     let rt = tokio::runtime::Runtime::new()?;
//     rt.block_on(async_main(diff_text))
// }

// #[cfg(feature = "uses_tokio_rt")]
// async fn async_main(diff_text: String) -> Result<(), Box<dyn std::error::Error>> {
//     chat_loop(diff_text).await
// }

// // When using tokio_plain, parse CLI arguments and check for recovery mode, then run synchronously.
// #[cfg(feature = "uses_tokio_plain")]
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Args::parse();
//     if args.recovery {
//         recover_commit_message()?;
//         return Ok(());
//     }
//     let diff_text = get_diff_text()?;
//     if diff_text.is_empty() {
//         return Ok(());
//     }
//     chat_loop(diff_text)
// }
