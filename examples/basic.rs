//! A simple example using the TCGdex Rust SDK

use tcgdex_sdk::{Language, TCGdex, Query, Extension, Quality};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TCGdex SDK Example");
    
    // Create a client
    let tcgdex = TCGdex::new(Language::EN);
    
    // Get a specific card (Charizard from Base Set)
    println!("Fetching card...");
    let card = tcgdex.card.get(tcgdex.client(), "base1-4").await?;
    println!("Found card: {} (HP: {})", card.name, card.hp.unwrap_or(0));
    
    // Build a query to find fire-type Pokémon
    println!("\nSearching for fire-type Pokémon...");
    let mut query = Query::new();
    query.equal("types", "Fire").limit(5);
    let cards = tcgdex.card.list(tcgdex.client(), Some(&query)).await?;
    println!("Found {} fire-type Pokémon", cards.len());
    
    // Display the first few results
    for (i, card_resume) in cards.iter().take(3).enumerate() {
        println!("{}. {}", i+1, card_resume.name);
    }
    
    // Get a specific set
    println!("\nFetching set...");
    let set = tcgdex.set.get(tcgdex.client(), "base1").await?;
    println!("Set: {} ({} cards)", set.name, set.card_count.total);
    
    // Get card image URL
    if let Some(image_url) = card.get_image_url(Quality::HIGH, Extension::PNG) {
        println!("\nCard image URL: {}", image_url);
    }
    
    Ok(())
}