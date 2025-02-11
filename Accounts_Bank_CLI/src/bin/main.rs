use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct Balance {
    #[serde(rename = "currencyCode")]
    currency_code: String,
    value: String,
    #[serde(rename = "valueInBaseUnits")]
    value_in_base_units: i32,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Account {
    #[serde(rename = "id")]
    account_id: String,
    #[serde(rename = "attributes")]
    attributes: AccountAttributes,
}

#[derive(Debug, Deserialize)]
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

    // Print account info
    for account in response.data {
        println!("Account ID: {}", account.account_id);
        println!("Display Name: {}", account.attributes.display_name);
        println!("Account Type: {}", account.attributes.account_type);
        println!("Ownership Type: {}", account.attributes.ownership_type);
        println!("Balance: {} {}", account.attributes.balance.value, account.attributes.balance.currency_code);
        println!("Created At: {}", account.attributes.created_at);
        println!();
    }
}
