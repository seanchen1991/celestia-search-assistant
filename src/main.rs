mod celestia_search_tool;

use crate::celestia_search_tool::CelestiaSearchTool;

use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = openai::Client::from_env();

    let agent = openai_client
        .agent("gpt-4o-mini")
        .preamble("You are a helpful assistant.")
        .tool(CelestiaSearchTool)
        .build();

    let response = agent
        .prompt("What is the gas fee of the Celestia block at height 9999?")
        .await?;

    let formatted_response: String = serde_json::from_str(&response)?;

    println!("Agent response:\n{}", formatted_response);

    Ok(())
}
