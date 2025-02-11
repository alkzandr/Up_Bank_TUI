use std::env;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DataStruct {
    data: Vec<Transactions>,
    links: Links,
}

#[derive(Debug, Deserialize, Serialize)]
struct Links {
    next: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transactions {
    attributes: TransactionAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
struct TransactionAttributes {
    description: Option<String>,
    message: Option<String>,
    roundUp: Option<Amount>,
    amount: Option<Amount>,
    performingCustomer: Option<PerformingCustomer>,
    settledAt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PerformingCustomer {
    displayName: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Amount {
    currencyCode: Option<String>,
    value: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = env::var("UP_API_TOKEN")
        .expect("UP_API_TOKEN must be correctly set in the environment");

    let mut url = "https://api.up.com.au/api/v1/transactions".to_string();
    let mut all_transactions: Vec<Transactions> = Vec::new();

    while !url.is_empty() {
        let datastruct: DataStruct = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await?
            .json()
            .await?;

        // Append transactions to the vector
        all_transactions.extend(datastruct.data);

        url = datastruct.links.next.unwrap_or_default();
    }

    // Save to JSON file
    let json_data = serde_json::to_string_pretty(&all_transactions)
        .expect("Failed to serialize transactions");

    let mut file = File::create("transactions.json")
        .expect("Failed to create file");

    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    println!("Transactions saved to transactions.json");

    Ok(())
}
