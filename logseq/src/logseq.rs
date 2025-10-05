use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogseqIcon {
    #[serde(rename = "type")]
    pub icon_type: String,
    pub id: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogseqTag {
    pub name: String,
    pub icon: Option<LogseqIcon>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogseqPage {
    pub title: String,
    pub uuid: String,
    pub tags: Vec<LogseqTag>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LogseqBlock {
    #[serde(rename = "db/id")]
    pub id: Option<i64>,
    #[serde(rename = "block/title")]
    pub title: Option<String>,
    #[serde(rename = "block/uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "block/tags")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(rename = "block/updated-at")]
    pub updated_at: Option<i64>,
    #[serde(rename = "logseq.property/icon")]
    pub icon: Option<LogseqIcon>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TagRef {
    #[serde(rename = "db/id")]
    pub id: Option<i64>,
}

pub fn get_logseq_pages() -> Result<Vec<LogseqPage>, String> {
    // load lson from ~/.local/state/anyrun/logseq.json
    let json_str = fs::read_to_string(format!(
        "{}/.local/state/anyrun/logseq.json",
        env::var("HOME").unwrap()
    ))
    .unwrap();

    let blocks: Vec<LogseqBlock> =
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let tag_blocks = match get_logseq_tag_blocks() {
        Ok(blocks) => blocks,
        Err(_) => Vec::new(),
    };

    let pages: Vec<LogseqPage> = blocks
        .into_iter()
        .map(|block| {
            let title = block.title.unwrap_or_else(|| "Untitled".to_string());
            let uuid = block.uuid.unwrap_or_default();
            let tags = block
                .tags
                .unwrap_or_default()
                .into_iter()
                .filter_map(|tag_ref| {
                    tag_ref.id.and_then(|id| {
                        tag_blocks
                            .iter()
                            .find(|tag_block| tag_block.id == Some(id))
                            .map(|tag_block| LogseqTag {
                                name: tag_block.title.clone().unwrap_or_default(),
                                icon: tag_block.icon.clone(),
                            })
                    })
                })
                .collect();

            LogseqPage {
                title,
                uuid,
                tags,
                updated_at: block.updated_at,
            }
        })
        .collect();

    Ok(pages)
}

fn get_logseq_tag_blocks() -> Result<Vec<LogseqBlock>, String> {
    // load lson from ~/.local/state/anyrun/logseq-tags.json
    let json_str = fs::read_to_string(format!(
        "{}/.local/state/anyrun/logseq-tags.json",
        env::var("HOME").unwrap()
    ))
    .unwrap();

    let blocks: Vec<LogseqBlock> =
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(blocks)
}
