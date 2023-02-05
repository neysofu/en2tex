#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the desired LaTeX output. You can use English words,
    /// abbreviations, or any notation resembling LaTeX commands and AsciiMath
    prompt: Vec<String>,

    /// Which OpenAI model to use
    #[clap(short = 'm', long, default_value = "text-davinci-003")]
    model: String,

    /// Copy the generated LaTeX without asking for confirmation first
    #[clap(short = 'c', long)]
    copy: bool,
}

fn main() -> Result<(), i32> {
    let cli = Cli::parse();

    let Ok(api_key) = env::var("OPENAI_API_KEY") else {
        println!("{}", "This program requires an OpenAI API key to run. Please set the OPENAI_API_KEY environment variable. https://github.com/neysofu/en2tex#usage".red());
        return Err(1);
    };

    let client = Client::new();
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating some LaTeX...".into());

    let response = client
        .post("https://api.openai.com/v1/completions")
        .json(&json!({
            "top_p": 1,
            "stop": "---",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1024,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": cli.model,
            "prompt": build_prompt(&cli.prompt.join(" ")),
        }))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .unwrap();

    let status_code = response.status();
    if status_code.is_client_error() {
        let response_body = response.json::<serde_json::Value>().unwrap();
        let error_message = response_body["error"]["message"].as_str().unwrap();
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("API error: \"{error_message}\"").red().to_string(),
        );
        return Err(1);
    } else if status_code.is_server_error() {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("OpenAI is currently experiencing problems. Status code: {status_code}")
                .red()
                .to_string(),
        );
        return Err(1);
    }

    let code = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    spinner.stop_and_persist(
        "✔".green().to_string().as_str(),
        "Got some code!".green().to_string(),
    );

    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("tex")
        .grid(true)
        .print()
        .unwrap();

    let should_copy = if cli.copy {
        true
    } else {
        Question::new(
            ">> Copy the generated code? [Y/n]"
                .bright_black()
                .to_string()
                .as_str(),
        )
        .yes_no()
        .until_acceptable()
        .default(Answer::YES)
        .ask()
        .expect("Couldn't ask question.")
            == Answer::YES
    };

    if should_copy {
        if let Err(e) = copy_to_clipboard(&code) {
            println!(
                "{} Couldn't copy to clipboard: {error}",
                "✖".red().to_string().as_str(),
                error = e
            );
        } else {
            println!("{} Copied to clipboard!", "✔".green().to_string().as_str());
        }
    }

    Ok(())
}

fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    arboard::Clipboard::new()?.set_text(content)?;

    Ok(())
}

fn build_prompt(user_input: &str) -> String {
    const PROMPT: &str = include_str!("../resources/initial_prompt.txt");
    PROMPT.replace("USER_INPUT", user_input)
}
