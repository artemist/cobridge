use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

/// Input to a tRPC query with a defined query name
pub trait TrpcInput {
    type Response: DeserializeOwned;
    fn query_name() -> &'static str;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A block containing markdown text, used as part of a [post](Post)
pub struct MarkdownBlock {
    /// Text, in markdown. Supports at least the features refernced on <https://cohost.org/rc/content/markdown-reference>
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// An attachment used in a [post](Post), normally an image
pub struct AttachmentBlock {
    /// Alt text, used for screen readers. Empty if there is no alt text
    pub alt_text: String,
    /// UUID of attachment
    pub attachment_id: String,
    /// Url of the file, often something like
    /// `https://staging.cohostcdn.org/attachment/<attachment_id>/<user-selected name>.png`
    #[serde(rename = "fileURL")]
    pub file_url: String,
    /// URL of an image preview of the file, often the same as file_url
    #[serde(rename = "previewURL")]
    pub preview_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
/// The basic block used for making a post
pub enum Block {
    Markdown { markdown: MarkdownBlock },
    Attachment { attachment: AttachmentBlock },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
/// Shape of the crop the client should use on the avatar
pub enum AvatarShape {
    /// A circle from the top to the bottom
    Circle,
    /// Square with a 0.5rem border radius
    RoundRect,
    /// A rounded rectangle with bulges on the sides
    Squircle,
    /// Avatar shape used on the [staff account](https://cohost.org/staff)
    Egg,
    /// Appears to be unused but referenced in the code
    #[serde(rename = "capsule-big")]
    CapsuleBig,
    /// Appears to be unused but referenced in the code
    #[serde(rename = "capsule-small")]
    CapsuleSmall,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
/// Visibility of a project's posts
pub enum Privacy {
    /// Shown to all
    Public,
    /// Shown only to approved followers
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "avatarPreviewURL")]
    pub avatar_preview_url: String,
    /// How to mask the avatar when shown
    pub avatar_shape: AvatarShape,
    #[serde(rename = "avatarURL")]
    pub avatar_url: String,
    /// The project's headline, shown above pronouns, link, and description.
    /// Internally referred to as "dek"
    #[serde(rename = "dek")]
    pub headline: String,
    pub description: String,
    pub display_name: String,
    pub flags: Value,
    pub handle: String,
    #[serde(rename = "headerPreviewURL")]
    pub header_preview_url: Option<String>,
    #[serde(rename = "headerURL")]
    pub header_url: Option<String>,
    pub privacy: Privacy,
    pub project_id: u64,
    pub pronouns: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// a post, as returned by posts.profilePosts and others
pub struct Post {
    pub blocks: Vec<Block>,
    pub can_publish: bool,
    pub can_share: bool,
    pub contributor_block_incoming_or_outgoing: bool,
    /// Content warnings of the post
    pub cws: Vec<String>,
    pub effective_adult_content: bool,
    pub filename: String,
    pub has_any_contributor_muted: bool,
    pub headline: String,
    pub is_editor: bool,
    pub is_liked: bool,
    pub num_comments: u64,
    pub num_shared_comments: u64,
    pub pinned: bool,
    pub plain_text_body: String,
    pub post_edit_url: String,
    pub post_id: u64,
    pub posting_project: Project,
    pub published_at: String,
    pub related_projects: Value,
    pub share_tree: Value,
    pub single_post_page_url: String,
    /// Probably means something but I'm not sure
    pub state: u64,
    pub tags: Vec<String>,
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
#[serde(rename_all = "kebab-case")]
pub enum AccessPermission {
    Allowed,
    NotAllowed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanAccessPermissions {
    pub can_read: AccessPermission,
    pub can_interact: AccessPermission,
    pub can_share: AccessPermission,
    pub can_edit: AccessPermission,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPageView {
    pub project: Project,
    pub page_handle: String,
    pub can_access_permissions: CanAccessPermissions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohostLoaderError {
    message: String,
    error_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectPageViewLoaderState {
    ProjectPageView(ProjectPageView),
    Error(CohostLoaderError),
}

impl From<ProjectPageViewLoaderState> for Result<ProjectPageView, CohostLoaderError> {
    fn from(state: ProjectPageViewLoaderState) -> Self {
        match state {
            ProjectPageViewLoaderState::ProjectPageView(view) => Self::Ok(view),
            ProjectPageViewLoaderState::Error(err) => Self::Err(err),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub code: String,
    pub http_status: u16,
    pub path: String,
    pub stack: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohostError {
    pub code: i64,
    pub data: ErrorData,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoggedInData {
    pub logged_in: bool,
    pub user_id: u64,
    pub email: String,
    pub project_id: u64,
    pub project_handle: String,
    pub mod_mode: bool,
    pub activated: bool,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfilePostsData {
    pub pagination: Pagination,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePostsInputOptions {
    pub hide_replies: bool,
    pub hide_shares: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePostsInput {
    pub project_handle: String,
    pub page: u64,
    pub options: ProfilePostsInputOptions,
}

impl TrpcInput for ProfilePostsInput {
    type Response = ProfilePostsData;
    fn query_name() -> &'static str {
        "posts.profilePosts"
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A basic container for a "result" field in a response
pub struct Success {
    /// Actual response as a JSON value. This may be any JSON object and is not parsed further
    /// because it would require a brute force of types. Convert this into base types with another
    /// deserialize or use the helper function
    /// [CohostApi::parse_response](crate::CohostApi::parse_response) on the containing
    /// [Response](Response) object.
    pub data: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// One of the elements returned in the response of a TRPC request.
/// Fields renamed to reduce confusion with Rust std types.
pub enum CohostResponse {
    /// Successfuly executed, internally called "result"
    #[serde(rename = "result")]
    Success(Success),
    /// Failed to execute, internally called "error"
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
