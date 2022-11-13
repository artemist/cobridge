use serde::{Deserialize, Serialize};

/// Representation of a WebFinger response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebFinger {
    /// `acct:` url of the subject
    pub subject: Box<str>,

    /// List of aliases, may be HTTP(S) links
    #[serde(default)]
    pub aliases: Box<[Box<str>]>,

    /// Links for specific purposes
    pub links: Box<[Link]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    /// What this link refers to (e.g. "self")
    pub rel: Box<str>,
    /// MIME type of the resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Box<str>>,
    /// Actual link in question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Box<str>>,
    /// URL template, may appear instead of [href](self::href)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<str>>,
}

impl WebFinger {
    pub fn with_cohost_handle(cohost_handle: &str, local_domain: &str) -> Self {
        Self {
            subject: format!("acct:{}@{}", cohost_handle, local_domain).into_boxed_str(),
            aliases: vec![
                format!("https://{}/users/{}", local_domain, cohost_handle).into_boxed_str(),
                format!("https://cohost.org/{}", cohost_handle).into_boxed_str(),
            ]
            .into_boxed_slice(),
            links: vec![
                Link {
                    rel: String::from("http://webfinger.net/rel/profile-page").into_boxed_str(),
                    mime_type: Some(String::from("text/html").into_boxed_str()),
                    href: Some(format!("https://cohost.org/{}", cohost_handle).into_boxed_str()),
                    template: None,
                },
                Link {
                    rel: String::from("self").into_boxed_str(),
                    mime_type: Some(String::from("application/activity+json").into_boxed_str()),
                    href: Some(
                        format!("https://{}/users/{}", local_domain, cohost_handle)
                            .into_boxed_str(),
                    ),
                    template: None,
                },
            ]
            .into_boxed_slice(),
        }
    }
}
