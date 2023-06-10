use crate::drive::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CloudFile {
    pub name: String,

    #[serde(rename = "file_id")]
    pub id: String,

    pub r#type: FileType,

    pub created_at: DateTime,

    pub updated_at: DateTime,

    #[serde(default)]
    pub size: u64,

    pub url: Option<String>,
}

impl From<ListFileItem> for CloudFile {
    fn from(f: ListFileItem) -> Self {
        Self {
            name: f.name,
            id: f.id,
            r#type: f.r#type,
            created_at: f.created_at,
            updated_at: f.updated_at,
            size: f.size,
            // 文件列表接口返回的图片下载地址经常是有问题的, 不使用它
            url: if matches!(f.category.as_deref(), Some("image")) {
                None
            } else {
                f.url
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListFileRequest<'a> {
    pub drive_id: &'a str,
    pub parent_file_id: &'a str,
    pub limit: u64,
    pub all: bool,
    pub image_thumbnail_process: &'a str,
    pub image_url_process: &'a str,
    pub video_thumbnail_process: &'a str,
    pub fields: &'a str,
    pub order_by: &'a str,
    pub order_direction: &'a str,
    pub marker: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFileResponse {
    pub items: Vec<ListFileItem>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFileItem {
    pub name: String,

    pub category: Option<String>,
    #[serde(rename = "file_id")]
    pub id: String,

    pub r#type: FileType,

    pub created_at: DateTime,

    pub updated_at: DateTime,
    #[serde(default)]
    pub size: u64,

    pub url: Option<String>,
}
