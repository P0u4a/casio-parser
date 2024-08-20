# Casio Model Code Parser ⌚

Ever wondered what that cryptic code on the back of your Casio watch means?

## Example

```
cargo run -- GW-B5600BC-1BER
```

**Output**

```
Found the following GShock data:
    Category: G-Shock Waveceptor
    Feature: Bluetooth
    Collection: Baby-G
    Main Colour: Black
    Accent Colour: None
    Order: 2nd Wave
    Manufacturer Origin: Europe
```

## Running

1. Build the project with `cargo build`
2. Test with `cargo test`
3. Run with `cargo run -- <watch-code>`
