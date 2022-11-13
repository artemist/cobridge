use cohost::CohostApi;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

mod cohost;
mod activitypub;
mod webfinger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let api = CohostApi::new();

    let mut responses = api
        .make_trpc_request(
            Arc::new([
                "posts.profilePosts".to_string(),
                "posts.profilePosts".to_string(),
            ]),
            false,
            &json!({
                "0": {
                    "projectHandle": "QuestForTori",
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
            }),
        )
        .await?;

    for response in responses.as_array_mut().unwrap() {
        match CohostApi::parse_response::<cohost::types::ProfilePostsData>(response.take()) {
            Ok(value) => println!("{:#?}", value),
            Err(error) => println!("{}", error.root_cause()),
        }
    }

    Ok(())
}
