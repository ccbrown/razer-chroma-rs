# razer-chroma-rs ![docs](https://docs.rs/razer_chroma/badge.svg)

The `razer_chroma` crate provides Razer Chroma SDK functionality in a safe and simple API.

You can initialize the SDK like so:

```rust
let sdk = match razer_chroma::SDK::new() {
    Some(sdk) => sdk,
    None => { warn!("razer drivers not installed or supported"); return },
}
```

Once you have the SDK instance, you can create and activate effects like so:

```rust
let effect = sdk.create_keyboard_effect(razer_chroma::KeyboardEffect::Static{
    color: razer_chroma::Color::from_rgb(0, 0, 255),
})?;
sdk.set_effect(&effect)?;
```
