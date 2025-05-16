//! A simple diagnostic example for the TCGdex Rust SDK

use tcgdex_sdk::{Language, TCGdex};

#[tokio::main]
async fn main() {
    println!("TCGdex SDK Diagnostic Example");
    
    // Create a client
    let tcgdex = TCGdex::new(Language::EN);
    
    // Test getting a card
    println!("\nTesting card endpoint...");
    match tcgdex.card.get(tcgdex.client(), "base1-4").await {
        Ok(card) => println!("✅ Successfully fetched card: {}", card.name),
        Err(err) => println!("❌ Error fetching card: {:?}", err),
    }
    
    // Test getting a set
    println!("\nTesting set endpoint...");
    match tcgdex.set.get(tcgdex.client(), "base1").await {
        Ok(set) => println!("✅ Successfully fetched set: {}", set.name),
        Err(err) => println!("❌ Error fetching set: {:?}", err),
    }
    
    // Test listing types
    println!("\nTesting type listing...");
    match tcgdex.type_.list(tcgdex.client(), None).await {
        Ok(types) => println!("✅ Successfully listed {} types", types.len()),
        Err(err) => println!("❌ Error listing types: {:?}", err),
    }
    
    println!("\nDiagnostic complete");
}