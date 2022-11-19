use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cohost::types::{Privacy, Project};

/// ActivityStreams types, as structures so that you can use serde to
/// serialize and deserialize the types
/// For simplicity these should be called in a compacted form with a
/// context of:
/// { "@context": ["https://w3.org/ns/activitystreams", "https://w3id.org/security/v1"]}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorType {
    Application,
    Group,
    Organization,
    Person,
    Service,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_authorization_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provide_client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_client_key: Option<String>,
    /// A single inbox shared by many users to reduce the number of
    /// POST requests when sending to followers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_inbox: Option<String>,
}

impl Endpoints {
    pub fn is_empty(&self) -> bool {
        self.proxy_url.is_none()
            && self.oauth_authorization_endpoint.is_none()
            && self.provide_client_key.is_none()
            && self.sign_client_key.is_none()
            && self.shared_inbox.is_none()
    }
}

fn _default_false() -> bool {
    false
}
fn _default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActorPage {
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: ActorType,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_tags: Option<String>,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "as:manuallyApprovesFollowers", default = "_default_true")]
    pub manually_approves_followers: bool,
    #[serde(default = "_default_true")]
    pub discoverable: bool,
    pub published: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<String>,
    #[serde(skip_serializing_if = "Endpoints::is_empty")]
    pub endpoints: Endpoints,
}

impl ActorPage {
    pub fn with_project(domain: &str, project: &Project) -> Self {
        Self {
            id: format!("https://{}/users/{}", domain, &project.handle),
            actor_type: ActorType::Person,
            following: format!("https://{}/users/{}/following", domain, &project.handle),
            followers: format!("https://{}/users/{}/followers", domain, &project.handle),
            inbox: format!("https://{}/users/{}/inbox", domain, &project.handle),
            outbox: format!("https://{}/users/{}/outbox", domain, &project.handle),
            liked: None,
            featured: None,
            featured_tags: None,
            preferred_username: project.display_name.to_string(),
            name: project.display_name.to_string(),
            summary: project.headline.to_string(),
            url: None,
            manually_approves_followers: project.privacy == Privacy::Private,
            discoverable: true,
            published: None,
            devices: None,
            tag: vec![],
            attachment: vec![],
            endpoints: Endpoints::default(),
        }
    }
}

pub fn serialize_with_context<T: Serialize>(obj: T) -> anyhow::Result<Value> {
    let mut value = serde_json::to_value(obj)?;
    if let Some(dict) = value.as_object_mut() {
        dict.insert(
            "@context".to_string(),
            json!([
                "https://w3.org/ns/activitystreams",
                "https://w3id.org/security/v1"
            ]),
        );
    } else {
        anyhow::bail!("Not an object");
    }
    Ok(value)
}
