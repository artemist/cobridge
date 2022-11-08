use cohost::CohostApi;
use std::sync::Arc;
use std::error::Error;
use serde_json::json;

mod cohost;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let api = CohostApi::new();

    println!("{:#}", api.make_trpc_request(Arc::new(["posts.profilePosts".to_string(), "posts.profilePosts".to_string()]), false, &json!({
        "0": {
            "projectHandle": "artemist",
            "page": 0,
            "options": {
                "hideReplies": false,
                "hideShares": false,
            },
        },
        "1": {
            "projectHandle": "nonexistant_account",
            "page": 0,
            "options": {
                "hideReplies": false,
                "hideShares": false,
            },
        },
    })
    ).await?);

    Ok(())
}
