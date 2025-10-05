mod logseq;

use std::{fs, process::Command};

use rand::rng;
use rand::seq::SliceRandom;
use serde::Deserialize;

use abi_stable::std_types::*;
use anyrun_plugin::*;

use crate::logseq::{get_logseq_pages, LogseqPage, LogseqTag};

struct State {
    items: Vec<LogseqPage>,
    config: Config,
}

#[derive(Deserialize, Clone)]
struct Config {
    max_entries: usize,
    graph: String,
    command_holder: Option<String>,
}

#[init]
fn init(config_dir: RString) -> State {
    let config = fs::read_to_string(format!("{}/logseq.ron", config_dir)).unwrap();
    let config: Config = ron::from_str(&config).unwrap();

    State {
        items: get_logseq_pages().unwrap(),
        config,
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Logseq".into(),
        icon: "logseq".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let search_term = input.trim().to_lowercase();
    let mut pages = state.items.clone();

    pages.sort_by(|a, b| match (a.updated_at, b.updated_at) {
        (Some(a_time), Some(b_time)) => b_time.cmp(&a_time),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    // Take top 5 most recently updated pages and shuffle the rest
    let pages_len = pages.len();
    let split_index = if pages_len > 5 { 5 } else { pages_len };

    let (recent_pages, remaining_pages) = pages.split_at_mut(split_index);

    // Shuffle the remaining pages
    remaining_pages.shuffle(&mut rng());

    // Combine recent pages (first 5) with shuffled remaining pages
    let mut sorted_pages = recent_pages.to_vec();
    sorted_pages.extend_from_slice(remaining_pages);

    let filtered_pages: Vec<_> = sorted_pages
        .into_iter()
        .filter(|page| {
            if search_term.is_empty() {
                true
            } else {
                page.title.to_lowercase().contains(&search_term)
                    || page
                        .tags
                        .iter()
                        .any(|tag| tag.name.to_string().to_lowercase().contains(&search_term))
            }
        })
        .take(state.config.max_entries)
        .collect();

    RVec::from(
        filtered_pages
            .into_iter()
            .map(|page| {
                let description = if page.tags.is_empty() {
                    RNone
                } else {
                    RSome(RString::from(
                        page.tags
                            .iter()
                            .filter(|tag| &tag.name != &"Page")
                            .map(|tag| format!("#{}", tag.name))
                            .collect::<Vec<String>>()
                            .join(" "),
                    ))
                };

                let icon = get_emoji_for_tags(&page.tags);

                Match {
                    title: RString::from(format!("{} {}", icon, page.title)),
                    id: RNone,
                    icon: RNone,
                    use_pango: false,
                    description,
                }
            })
            .collect::<Vec<_>>(),
    )
}

#[handler]
fn handler(selection: Match, state: &State) -> HandleResult {
    if let Some(page) = state
        .items
        .iter()
        .find(|page| selection.title.contains(&page.title))
    {
        let xdg_open_command = format!(
            "xdg-open logseq://graph/{}?block-id={}",
            state.config.graph, page.uuid
        );
        let command = if let Some(command_holder) = &state.config.command_holder {
            command_holder.replace("{}", &xdg_open_command)
        } else {
            xdg_open_command
        };

        eprintln!("command: {}", command);

        Command::new("sh").arg("-c").arg(command).spawn().unwrap();
    }
    HandleResult::Close
}

fn get_emoji_for_tags(tags: &[LogseqTag]) -> String {
    if let Some(tag) = tags
        .iter()
        .filter(|t| {
            let excluded = [
                "Task",
                "Page",
                "DONE",
                "TODAY",
                "INBOX",
                "SOMEDAY",
                "DELEGATE",
                "GTD-PROJECT",
            ];
            !excluded.contains(&t.name.as_str())
        })
        .next()
    {
        if let Some(icon) = &tag.icon {
            if icon.icon_type == "emoji" {
                if let Some(emoji) = emojis::get_by_shortcode(&icon.id) {
                    return emoji.to_string();
                }
            }
        }
    }

    if tags
        .iter()
        .any(|tag| matches!(tag.name.as_str(), "Task" | "GTD" | "GTD-PROJECT"))
    {
        return "☑".to_string();
    }
    "📝".to_string()
}
