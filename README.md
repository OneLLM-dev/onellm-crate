# OneLLM API Client

This is a Rust client for interacting with the OneLLM API.

## Usage

Add this to your `Cargo.toml`:

```toml
onellm = "0.1.0"
```

## Example

```rust
use onellm::input::{APIInput, Model, Message};
use onellm::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = APIInput::new(
        "http://localhost:8000".to_string(),
        Model::Gemini15Pro,
        vec![
            Message {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
            },
        ],
        100,
    );

    let response = input.send().await?;

    println!("Response: {:?}", response);

    Ok(())
}
```
