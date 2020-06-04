use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub enum Selector {
    New,
}

impl Selector {
    pub fn reddit_name(&self) -> &'static str {
        match self {
            Selector::New => "new",
        }
    }
}
#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub kind: String,
    pub data: Data,
}

impl Root {
    pub fn to_set(self) -> HashSet<Child> {
        use std::iter::FromIterator;
        HashSet::from_iter(self.data.children.into_iter().map(|a| a.data))
    }
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub modhash: String,
    pub dist: Option<i64>,
    pub children: Vec<Children>,
    pub after: String,
    pub before: Option<String>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub kind: String,
    pub data: Child,
}

#[derive(Default, Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub subreddit: String,
    pub selftext: String,
    #[serde(rename = "author_fullname")]
    pub author_fullname: String,
    pub saved: bool,
    pub gilded: Option<i64>,
    pub clicked: bool,
    pub title: String,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: Option<i64>,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<String>,
    pub downs: Option<i64>,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    #[serde(rename = "hide_score")]
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: String,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<String>,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: String,
    pub ups: Option<i64>,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,
    #[serde(rename = "media_embed")]
    pub media_embed: MediaEmbed,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<String>,
    #[serde(rename = "is_original_content")]
    pub is_original_content: bool,
    #[serde(rename = "secure_media")]
    pub secure_media: Option<SecureMedia>,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: bool,
    #[serde(rename = "is_meta")]
    pub is_meta: bool,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: SecureMediaEmbed,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<String>,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: bool,
    pub score: Option<i64>,
    #[serde(rename = "author_premium")]
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: bool,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<String>,
    pub gildings: Gildings,
    #[serde(rename = "post_hint")]
    pub post_hint: Option<String>,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "mod_note")]
    pub mod_note: Option<String>,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: String,
    pub wls: Option<i64>,
    #[serde(rename = "removed_by_category")]
    pub author_flair_type: Option<String>,
    pub domain: String,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: bool,
    pub archived: bool,
    #[serde(rename = "no_follow")]
    pub no_follow: bool,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: bool,
    pub pinned: bool,
    #[serde(rename = "over_18")]
    pub over18: bool,
    pub preview: Option<Preview>,
    #[serde(rename = "media_only")]
    pub media_only: bool,
    #[serde(rename = "link_flair_template_id")]
    pub link_flair_template_id: Option<String>,
    #[serde(rename = "can_gild")]
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<String>,
    pub visited: bool,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    pub id: String,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: bool,
    pub author: String,
    #[serde(rename = "num_comments")]
    pub num_comments: Option<i64>,
    #[serde(rename = "send_replies")]
    pub send_replies: bool,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: Option<String>,
    #[serde(rename = "contest_mode")]
    pub contest_mode: bool,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: bool,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<String>,
    pub permalink: String,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: Option<String>,
    pub stickied: bool,
    pub url: String,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: Option<i64>,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: Option<i64>,
    pub media: Option<Media>,
    #[serde(rename = "is_video")]
    pub is_video: bool,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed {
    pub content: Option<String>,
    pub width: Option<Option<i64>>,
    pub scrolling: Option<bool>,
    pub height: Option<Option<i64>>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMedia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub oembed: Oembed,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oembed {
    #[serde(rename = "provider_url")]
    pub provider_url: String,
    pub description: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "author_name")]
    pub author_name: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub html: String,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    pub version: String,
    #[serde(rename = "provider_name")]
    pub provider_name: String,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    pub url: Option<String>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed {
    pub content: Option<String>,
    pub width: Option<Option<i64>>,
    pub scrolling: Option<bool>,
    #[serde(rename = "media_domain_url")]
    pub media_domain_url: Option<String>,
    pub height: Option<Option<i64>>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings {}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub images: Vec<Image>,
    pub enabled: bool,
    #[serde(rename = "reddit_video_preview")]
    pub reddit_video_preview: Option<RedditVideoPreview>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub variants: Variants,
    pub id: String,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub url: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub url: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variants {}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditVideoPreview {
    #[serde(rename = "fallback_url")]
    pub fallback_url: String,
    pub height: Option<i64>,
    pub width: Option<i64>,
    #[serde(rename = "scrubber_media_url")]
    pub scrubber_media_url: String,
    #[serde(rename = "dash_url")]
    pub dash_url: String,
    pub duration: Option<i64>,
    #[serde(rename = "hls_url")]
    pub hls_url: String,
    #[serde(rename = "is_gif")]
    pub is_gif: bool,
    #[serde(rename = "transcoding_status")]
    pub transcoding_status: String,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "type")]
    pub type_field: String,
    pub oembed: Oembed2,
}

#[derive(Hash, Eq, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oembed2 {
    #[serde(rename = "provider_url")]
    pub provider_url: String,
    pub description: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "author_name")]
    pub author_name: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub html: String,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    pub version: String,
    #[serde(rename = "provider_name")]
    pub provider_name: String,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    pub url: Option<String>,
}
