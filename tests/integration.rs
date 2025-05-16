//! Integration tests for the TCGdex SDK
//!
//! These tests make real API requests and are marked with #[ignore] by default
//! To run these tests: cargo test --test integration -- --ignored

use tcgdex_sdk::{Language, Query, TCGdex};

#[tokio::test]
async fn test_get_card() {
    let tcgdex = TCGdex::new(Language::EN);

    // Test getting a specific card (Charizard from Base Set)
    let card = tcgdex.card.get(tcgdex.client(), "base1-4").await.unwrap();
    assert_eq!(card.name, "Charizard");
    assert_eq!(card.hp, Some(120));
    assert!(card.image.is_some());
}

#[tokio::test]
async fn test_card_list() {
    let tcgdex = TCGdex::new(Language::EN);

    // Test listing cards with a query
    let mut query = Query::new();
    query.contains("name", "Pikachu");
    let cards = tcgdex
        .card
        .list(tcgdex.client(), Some(&query))
        .await
        .unwrap();

    // Should find some Pikachu cards
    assert!(!cards.is_empty());
    for card in cards {
        assert!(card.name.contains("Pikachu"));
    }
}

#[tokio::test]
async fn test_set() {
    let tcgdex = TCGdex::new(Language::EN);

    // Test getting a specific set (Base Set)
    let set = tcgdex.set.get(tcgdex.client(), "base1").await.unwrap();
    assert_eq!(set.name, "Base Set");
    assert!(!set.cards.is_empty());
}

#[tokio::test]
async fn test_serie() {
    let tcgdex = TCGdex::new(Language::EN);

    // Test getting a specific serie (Base)
    let serie = tcgdex.serie.get(tcgdex.client(), "base").await.unwrap();
    assert_eq!(serie.name, "Base");
    assert!(!serie.sets.is_empty());
}

#[tokio::test]
async fn test_different_languages() {
    // Test English (default)
    let tcgdex_en = TCGdex::new(Language::EN);
    let card_en = tcgdex_en
        .card
        .get(tcgdex_en.client(), "base1-4")
        .await
        .unwrap();
    assert_eq!(card_en.name, "Charizard");

    // Test French
    let tcgdex_fr = TCGdex::new(Language::FR);
    let card_fr = tcgdex_fr
        .card
        .get(tcgdex_fr.client(), "base1-4")
        .await
        .unwrap();
    assert_eq!(card_fr.name, "Dracaufeu");

    // Test German
    let tcgdex_de = TCGdex::new(Language::DE);
    let card_de = tcgdex_de
        .card
        .get(tcgdex_de.client(), "base1-4")
        .await
        .unwrap();
    assert_eq!(card_de.name, "Glurak");
}

#[tokio::test]
async fn test_string_endpoints() {
    let tcgdex = TCGdex::new(Language::EN);

    // Test type endpoint
    let types_list = tcgdex.type_.list(tcgdex.client(), None).await.unwrap();
    assert!(!types_list.is_empty());
    
    // We expect each StringList to contain various types including "Fire"
    let has_fire = types_list.iter().any(|string_list| {
        string_list.contains("Fire")
    });
    assert!(has_fire, "Should find 'Fire' type in the results");

    // Test getting cards of a specific type
    let fire_type = tcgdex.type_.get(tcgdex.client(), "Fire").await.unwrap();
    assert_eq!(fire_type.name, "Fire");
    assert!(!fire_type.cards.is_empty());
}

#[tokio::test]
async fn test_complex_query() {
    let tcgdex = TCGdex::new(Language::EN);

    // Build a complex query
    let mut query = Query::new();
    query
        .equal("types", "Fire")
        .greater_or_equal_than("hp", 100)
        .sort("name", "asc");

    // Get high HP Fire-type Poku00e9mon
    let card_resumes = tcgdex
        .card
        .list(tcgdex.client(), Some(&query))
        .await
        .unwrap();

    // Verify at least one result was returned
    assert!(!card_resumes.is_empty());

    // Get the full details of the first card and verify its HP
    let card = card_resumes[0].get_full_card(&tcgdex).await.unwrap();
    assert!(card.hp.unwrap_or(0) >= 100, "Card HP should be >= 100");
}

#[tokio::test]
async fn test_card_image_url() {
    let tcgdex = TCGdex::new(Language::EN);

    // Get a card with an image
    let card = tcgdex.card.get(tcgdex.client(), "base1-4").await.unwrap();

    // Check that we can generate an image URL
    let url = card.get_image_url(tcgdex_sdk::Quality::HIGH, tcgdex_sdk::Extension::PNG);
    assert!(url.is_some());
    let url = url.unwrap();
    assert!(url.ends_with("/high.png"));

    // We won't actually download the image to avoid unnecessary load on the API
    // but we could use card.get_image(tcgdex.client(), Quality::HIGH, Extension::PNG).await
}
