use serde::{Deserialize, Serialize};

/// Representation of a WebFinger response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebFinger {
    /// `acct:` url of the subject
    pub subject: String,

    /// List of aliases, may be HTTP(S) links
    #[serde(default)]
    pub aliases: Vec<String>,

    /// Links for specific purposes
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    /// What this link refers to (e.g. "self")
    pub rel: String,
    /// MIME type of the resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Actual link in question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// URL template, may appear instead of [href](self::href)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl WebFinger {
    pub fn with_cohost_handle(cohost_handle: &str, local_domain: &str) -> Self {
        Self {
            subject: format!("acct:{}@{}", cohost_handle, local_domain),
            aliases: vec![
                format!("https://{}/users/{}", local_domain, cohost_handle),
                format!("https://cohost.org/{}", cohost_handle),
            ]
            ,
            links: vec![
                Link {
                    rel: String::from("http://webfinger.net/rel/profile-page"),
                    mime_type: Some(String::from("text/html")),
                    href: Some(format!("https://cohost.org/{}", cohost_handle)),
                    template: None,
                },
                Link {
                    rel: String::from("self"),
                    mime_type: Some(String::from("application/activity+json")),
                    href: Some(
                        format!("https://{}/users/{}", local_domain, cohost_handle)
                            ,
                    ),
                    template: None,
                },
            ]
            ,
        }
    }
}
