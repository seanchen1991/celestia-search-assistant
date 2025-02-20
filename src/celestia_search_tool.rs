use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const API_ENDPOINT: &str = "https://api-mainnet.celenium.io/v1/block";

/// The query parameters that the agent will inject into the search.
#[derive(Deserialize)]
pub struct CelestiaSearchArgs {
    /// The block height at which to query.
    height: u64,
}

/// The fields that are received in the search response.
#[derive(Serialize)]
pub struct CelestiaOption {
    blobs_count: u64,
    blobs_size: u64,
    block_time: u64,
    bytes_in_block: u64,
    commissions: String,
    events_count: u64,
    fee: String,
    fill_rate: String,
    gas_limit: u64,
    gas_used: u64,
    inflation_rate: String,
    rewards: String,
    square_size: u64,
    supply_change: String,
    tx_count: u64,
}

/// Captures the possible types of errors that may occur while searching.
#[derive(Debug, thiserror::Error)]
pub enum CelestiaSearchError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

pub struct CelestiaSearchTool;

impl Tool for CelestiaSearchTool {
    const NAME: &'static str = "search_blocks";

    type Args = CelestiaSearchArgs;
    type Output = String;
    type Error = CelestiaSearchError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search for info on Celestia blocks".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "height": { "type": "integer", "description": "Height of the block to search for (e.g., '10000')" },
                },
                "required": ["height"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Format the search URL
        let url = format!("{}/{}/stats", API_ENDPOINT, args.height);

        // Make the API request
        let response = reqwest::get(url)
            .await
            .map_err(|e| CelestiaSearchError::HttpRequestFailed(e.to_string()))?;

        // Get the status code before consuming the response
        let status = response.status();

        // Consume the response and read the response text
        let text = response
            .text()
            .await
            .map_err(|e| CelestiaSearchError::HttpRequestFailed(e.to_string()))?;

        // Check if the response is an error
        if !status.is_success() {
            return Err(CelestiaSearchError::ApiError(format!(
                "Status: {}, Response: {}",
                status, text
            )));
        }

        // Parse the response JSON
        let data: Value = serde_json::from_str(&text)
            .map_err(|e| CelestiaSearchError::HttpRequestFailed(e.to_string()))?;

        // Check for API errors in the JSON response
        if let Some(error) = data.get("error") {
            let error_message = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            return Err(CelestiaSearchError::ApiError(error_message.to_string()));
        }

        // Populate the CelestiaOption type with fields from the response
        let tx_count = data
            .get("tx_count")
            .and_then(|tc| tc.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let block_time = data
            .get("block_time")
            .and_then(|bt| bt.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let gas_limit = data
            .get("gas_limit")
            .and_then(|gl| gl.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let gas_used = data
            .get("gas_used")
            .and_then(|gu| gu.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let square_size = data
            .get("square_size")
            .and_then(|ss| ss.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let bytes_in_block = data
            .get("bytes_in_block")
            .and_then(|bib| bib.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let events_count = data
            .get("events_count")
            .and_then(|ec| ec.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let blobs_count = data
            .get("blobs_count")
            .and_then(|bc| bc.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let blobs_size = data
            .get("blobs_size")
            .and_then(|bs| bs.as_str())
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let fee = data.get("fee").and_then(|f| f.as_str()).unwrap_or("0");
        let supply_change = data
            .get("supply_change")
            .and_then(|sc| sc.as_str())
            .unwrap_or("0");
        let inflation_rate = data
            .get("inflation_rate")
            .and_then(|ir| ir.as_str())
            .unwrap_or("0");
        let fill_rate = data
            .get("fill_rate")
            .and_then(|fr| fr.as_str())
            .unwrap_or("0");
        let rewards = data.get("rewards").and_then(|r| r.as_str()).unwrap_or("0");
        let commissions = data
            .get("commissions")
            .and_then(|c| c.as_str())
            .unwrap_or("0");

        let celestia_option = CelestiaOption {
            blobs_count,
            blobs_size,
            block_time,
            bytes_in_block,
            commissions: commissions.to_string(),
            events_count,
            fee: fee.to_string(),
            fill_rate: fill_rate.to_string(),
            gas_limit,
            gas_used,
            inflation_rate: inflation_rate.to_string(),
            rewards: rewards.to_string(),
            square_size,
            supply_change: supply_change.to_string(),
            tx_count,
        };

        let mut output = String::new();
        output.push_str(&format!("    The gas fee is: {}", celestia_option.fee));

        Ok(output)
    }
}
