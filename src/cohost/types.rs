use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownBlock {
    pub content: Box<str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentBlock {
    alt_text: Box<str>,
    attachment_id: Box<str>,
    #[serde(rename = "fileURL")]
    file_url: Box<str>,
    #[serde(rename = "previewURL")]
    preview_url: Box<str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Block {
    Markdown { markdown: MarkdownBlock },
    Attachment { attachment: AttachmentBlock },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AvatarShape {
    Circle,
    RoundRect,
    Squircle,
    /// Secret 4th avatar shape
    Egg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Privacy {
    Public,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "avatarPreviewURL")]
    pub avatar_preview_url: Box<str>,
    pub avatar_shape: AvatarShape,
    #[serde(rename = "avatarURL")]
    pub avatar_url: Box<str>,
    /// The project's headline, shown above pronouns, link, and description.
    /// Internally referred to as "dek"
    #[serde(rename = "dek")]
    pub headline: Box<str>,
    pub description: Box<str>,
    pub display_name: Box<str>,
    pub flags: Value,
    pub handle: Box<str>,
    #[serde(rename = "headerPreviewURL")]
    pub header_preview_url: Option<Box<str>>,
    #[serde(rename = "headerURL")]
    pub header_url: Option<Box<str>>,
    pub privacy: Privacy,
    pub project_id: u64,
    pub pronouns: Option<Box<str>>,
    pub url: Box<str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// a post, as returned by posts.profilePosts and others
pub struct Post {
    pub blocks: Box<[Block]>,
    pub can_publish: bool,
    pub can_share: bool,
    pub contributor_block_incoming_or_outgoing: bool,
    /// Content warnings of the post
    pub cws: Box<[Box<str>]>,
    pub effective_adult_content: bool,
    pub filename: Box<str>,
    pub has_any_contributor_muted: bool,
    pub headline: Box<str>,
    pub is_editor: bool,
    pub is_liked: bool,
    pub num_comments: u64,
    pub num_shared_comments: u64,
    pub pinned: bool,
    pub plain_text_body: Box<str>,
    pub post_edit_url: Box<str>,
    pub post_id: u64,
    pub posting_project: Project,
    pub published_at: Box<str>,
    pub related_projects: Value,
    pub share_tree: Value,
    pub single_post_page_url: Box<str>,
    /// Probably means something but I'm not sure
    pub state: u64,
    pub tags: Box<[Box<str>]>,
    pub transparent_share_of_post_id: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub current_page: u64,
    pub more_pages_forward: bool,
    pub next_page: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub code: Box<str>,
    pub http_status: u16,
    pub path: Box<str>,
    pub stack: Box<str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohostError {
    pub code: i64,
    pub data: ErrorData,
    pub message: Box<str>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoggedInData {
    pub logged_in: bool,
    pub user_id: u64,
    pub email: Box<str>,
    pub project_id: u64,
    pub project_handle: Box<str>,
    pub mod_mode: bool,
    pub activated: bool,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfilePostsData {
    pub pagination: Pagination,
    pub posts: Box<[Post]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Success {
    pub data: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    #[serde(rename = "result")]
    Success(Success),
    #[serde(rename = "error")]
    Failure(CohostError),
}

impl std::error::Error for CohostError {}

impl Display for CohostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cohost HTTP error {}, error code {} ({}), request type {}, message \"{}\"",
            self.data.http_status, self.data.code, self.code, self.data.path, self.message
        )
    }
}
