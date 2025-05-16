<p align="center">
	<a href="https://www.tcgdex.net">
		<img src="https://www.tcgdex.net/assets/og.png" width="90%" alt="TCGdex Main Image">
	</a>
</p>
<p align="center">
	<a href="https://crates.io/crates/tcgdex_sdk">
		<img src="https://img.shields.io/crates/v/tcgdex_sdk?style=flat-square" alt="Crates.io Version">
	</a>
	<a href="https://crates.io/crates/tcgdex_sdk">
		<img src="https://img.shields.io/crates/d/tcgdex_sdk?style=flat-square" alt="Crates.io Downloads">
	</a>
	<a href="https://github.com/tcgdex/rust-sdk/stargazers">
		<img src="https://img.shields.io/github/stars/tcgdex/rust-sdk?style=flat-square" alt="Github stars">
	</a>
	<a href="https://github.com/tcgdex/rust-sdk/actions/workflows/build.yml">
		<img src="https://img.shields.io/github/actions/workflow/status/tcgdex/rust-sdk/build.yml?style=flat-square" alt="Build Status" />
	</a>
	<a href="https://discord.gg/peACSRMZ7V">
		<img src="https://img.shields.io/discord/857231041261076491?color=%235865F2&label=Discord&style=flat-square" alt="Discord Link">
	</a>
</p>

# TCGdex Rust SDK

A fast, robust, and type-safe Rust SDK for the TCGdex API. Query Pokuémon Trading Card Game data with ease. 🦀

```rust
use tcgdex_sdk::{TCGdex, Language};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch a card in one line
    let tcgdex = TCGdex::new(Language::EN);
    let card = tcgdex.card.get(tcgdex.client(), "swsh3-136").await?;

    println!("Found: {} ({}/{})",
             card.name,
             card.local_id,
             card.set.card_count.total);
    Ok(())
}
```

## ⚡️ Quick Install

Add to your Cargo.toml:

```toml
[dependencies]
tcgdex_sdk = "0.1.0"
```

Or use cargo add:

```bash
cargo add tcgdex_sdk
```

## 🚀 Features

- **Type-Safe**: Full type safety with Rust's strong type system
- **Async/Await**: Built for modern Rust applications
- **Zero Config**: Works out of the box
- **Multi-Language**: Support for English, French, German, Japanese, Chinese, and more
- **Rich Data**: Access cards, sets, series, rarities, and more
- **Error Handling**: Comprehensive error handling with thiserror
- **Testable**: Unit and integration tests included

## 🎯 Quick Examples

### Find Cards by Various Criteria

```rust
use tcgdex_sdk::{TCGdex, Language, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcgdex = TCGdex::new(Language::EN);

    // Get the cards made by the illustrator
    let illustrator = tcgdex.illustrator.get(tcgdex.client(), "5ban Graphics").await?;

    // Get the data about the Sword & Shield serie by ID
    let series = tcgdex.serie.get(tcgdex.client(), "swsh").await?;

    // Get all cards with 110 HP
    let hp_cards = tcgdex.hp.get(tcgdex.client(), "110").await?;

    // List all available rarities
    let rarities = tcgdex.rarity.list(tcgdex.client(), None).await?;

    // List all cards with the name being "Furret"
    let mut query = Query::new();
    query.equal("name", "Furret");
    let furret_cards = tcgdex.card.list(tcgdex.client(), Some(&query)).await?;

    Ok(())
}
```

### Working with Sets and Series

```rust
use tcgdex_sdk::{TCGdex, Language, Extension};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcgdex = TCGdex::new(Language::EN);

    // Get set details
    let darkness_ablaze = tcgdex.set.get(tcgdex.client(), "swsh3").await?;
    println!("Set: {} ({} cards)",
             darkness_ablaze.name,
             darkness_ablaze.card_count.total);

    // Get series info
    let swsh = tcgdex.serie.get(tcgdex.client(), "swsh").await?;
    println!("Series: {} ({} sets)",
             swsh.name,
             swsh.sets.len());

    // Download a set logo
    if let Some(url) = darkness_ablaze.get_logo_url(Extension::PNG) {
        println!("Logo URL: {}", url);
        // You can download the image with:
        // let logo_bytes = darkness_ablaze.get_logo(tcgdex.client(), Extension::PNG).await?;
    }

    Ok(())
}
```

### Working with Card Images

```rust
use tcgdex_sdk::{TCGdex, Language, Quality, Extension};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcgdex = TCGdex::new(Language::EN);

    // Get a card
    let card = tcgdex.card.get(tcgdex.client(), "base1-4").await?; // Charizard

    // Get high resolution PNG image URL
    if let Some(image_url) = card.get_image_url(Quality::HIGH, Extension::PNG) {
        println!("Image URL: {}", image_url);

        // Download image
        if let Some(image_bytes) = card.get_image(tcgdex.client(), Quality::HIGH, Extension::PNG).await? {
            // Save to file
            let mut file = File::create("charizard.png")?;
            file.write_all(&image_bytes)?;
            println!("Image saved to charizard.png");
        }
    }

    Ok(())
}
```

## 🛠 Available Endpoints

### Card Data
```rust
tcgdex.card         // Core card data
tcgdex.rarity       // Card rarities
tcgdex.hp           // HP values
tcgdex.illustrator  // Card illustrators
```

### Game Mechanics
```rust
tcgdex.type_        // Pokémon types
tcgdex.energy_type  // Energy types
tcgdex.retreat      // Retreat costs
tcgdex.stage        // Evolution stages
```

### Card Details
```rust
tcgdex.variant         // Card variants
tcgdex.suffix          // Card suffixes
tcgdex.regulation_mark // Regulation marks
tcgdex.dex_id          // Pokédex IDs
```

### Collections
```rust
tcgdex.set           // Card sets
tcgdex.serie         // Card series
```

## 🌐 Language Support

```rust
use tcgdex_sdk::{TCGdex, Language};

// Using enum (type-safe)
let tcgdex = TCGdex::new(Language::EN); // English
let tcgdex = TCGdex::new(Language::FR); // French

// After creating the instance you can change the language
let mut tcgdex = TCGdex::default(); // Default is EN
tcgdex.set_language(Language::FR);
```

_[Full list of languages available in the Language enum](src/models/enums.rs)_

## 🔄 Query Building

The SDK provides a powerful query builder for filtering API results:

```rust
use tcgdex_sdk::{TCGdex, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcgdex = TCGdex::default();

    // Build a complex query
    let mut query = Query::new();
    query.contains("name", "Pikachu")
         .equal("types", "Electric")
         .greater_than("hp", 50)
         .sort("name", "asc");

    // Use the query
    let cards = tcgdex.card.list(tcgdex.client(), Some(&query)).await?;
    println!("Found {} matching cards", cards.len());

    Ok(())
}
```

## 🤝 Contributing

We love contributions! Here's how:

1. 🍴 Fork it
2. 🌿 Create your feature branch (`git checkout -b feature/amazing`)
3. 🔧 Make your changes
4. 🚀 Push to the branch (`git push origin feature/amazing`)
5. 🎉 Open a PR

## 📘 Documentation

- [Full API Documentation](https://www.tcgdex.dev)
- [Rust SDK Guide](https://www.tcgdex.dev/sdks/rust)
- [Rust Docs (docs.rs)](https://docs.rs/tcgdex_sdk)

## 💬 Community & Support

- [Discord Server](https://discord.gg/peACSRMZ7V) - Get help and discuss features
- [GitHub Issues](https://github.com/tcgdex/rust-sdk/issues) - Bug reports and feature requests

## 📜 License

MIT © [TCGdex](https://github.com/tcgdex)
