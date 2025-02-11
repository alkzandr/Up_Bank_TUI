use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct Balance {
    #[serde(rename = "currencyCode")]
    currency_code: String,
    value: String,
    #[serde(rename = "valueInBaseUnits")]
    value_in_base_units: i32,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct AccountAttributes {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "accountType")]
    account_type: String,
    #[serde(rename = "ownershipType")]
    ownership_type: String,
    balance: Balance,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct Account {
    #[serde(rename = "id")]
    account_id: String,
    #[serde(rename = "attributes")]
    attributes: AccountAttributes,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct ApiResponse {
    data: Vec<Account>,
}

fn main() {
    let token = env::var("UP_API_TOKEN").expect("UP_API_TOKEN not set in environment variables");
    let url = "https://api.up.com.au/api/v1/accounts";

    let client = Client::new();
    let response_text = client
        .get(url)
        .bearer_auth(token)
        .send()
        .expect("Request failed")
        .text()
        .expect("Failed to read response");

    // Deserialize the raw JSON into structured data
    let response: ApiResponse = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    // Save data to JSON file
    let file_path = "accounts_balance.json";
    let file = File::create(file_path).expect("Failed to create file");
    serde_json::to_writer_pretty(&file, &response).expect("Failed to write JSON to file");

    println!("Data saved to {}", file_path);
}
