# systemone

rust client for the system one api (`POST /systemone`). send state + typed
questions, get typed answers. base url, model and api key are all swappable.

```rust
let client = SystemOneClient::builder()
    .base_url("https://openrouter.ai/api/v1")
    .api_key("sk-or-...")
    .model("typesafe/jev-1.13")
    .build()?;

let resp = client
    .submit("my checkout page is blank after clicking pay", [
        ("is_bug", Question::noul("is this a software defect?")),
        ("team", Question::choice("which team owns this?", [("payments", "billing stuff"), ("frontend", "ui stuff")])),
        ("urgency", Question::score("how urgent?", ["whenever", "this week", "on fire"])),
    ])
    .await?;
```

question types: `noul` (0..1), `choice`, `score`. answers come
back as the matching `Answer` variant. errors: `SystemOneError::{Http, Api, Decode}`.