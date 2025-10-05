mod raindrop;

use serde::Deserialize;
use shellexpand::tilde;
use std::{fs, process::Command};

use rand::seq::SliceRandom;

use abi_stable::std_types::*;
use anyrun_plugin::*;
use raindrop::*;
use types::Item;

struct State {
    items: Vec<Item>,
    config: Config,
}

#[derive(Deserialize, Clone)]
struct Config {
    max_entries: usize,
    command_holder: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: 15,
            command_holder: None,
        }
    }
}

fn search(items: Vec<Item>, search: &str) -> Vec<Item> {
    let mut items = items;
    items.sort_by(|a, b| b.last_update.cmp(&a.last_update));

    return if search.trim().len() == 0 {
        items
    } else {
        for keyword in search.split(" ") {
            items = if keyword.starts_with("#") && keyword.len() > 1 {
                items
                    .into_iter()
                    .filter(|i| {
                        i.tags
                            .iter()
                            .filter(|t| t.to_lowercase().contains(&keyword[1..]))
                            .count()
                            > 0
                    })
                    .collect()
            } else {
                items
                    .into_iter()
                    .filter(|i| i.title.to_lowercase().contains(&keyword))
                    .collect()
            };
        }
        items
    };
}

#[init]
fn init(config_dir: RString) -> State {
    let config = if let Ok(config_str) = fs::read_to_string(format!("{}/raindrop.ron", config_dir))
    {
        if let Ok(config) = ron::from_str::<Config>(&config_str) {
            config
        } else {
            eprintln!("[raindrop-run] Failed to parse config");
            Config::default()
        }
    } else {
        Config::default()
    };

    let path = format!("{}/raindrop.ron", config_dir);
    let config = match fs::read_to_string(&path)
        .ok()
        .and_then(|s| ron::from_str::<Config>(&s).ok())
    {
        Some(cfg) => cfg,
        None => {
            eprintln!("[raindrop-run] Failed to load or parse config, using default");
            Config::default()
        }
    };
    let cache = raindrop::cache::FileItemCache::default();
    State {
        items: cache.load_cache().unwrap(),
        config,
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Raindrop".into(),
        icon: "raindrop".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let mut items = search(state.items.clone(), input.as_str());
    let mut rng = rand::rng();
    items.shuffle(&mut rng);

    items
        .into_iter()
        .map(|i| Match {
            title: RString::from(i.title),
            description: RSome(RString::from(format!(
                "{} {}",
                i.tags
                    .iter()
                    .map(|t| format!("#{}", t))
                    .collect::<Vec<_>>()
                    .join(" "),
                i.excerpt.chars().take(50).collect::<String>() + "..."
            ))),
            use_pango: false,
            icon: RSome(RString::from(&*tilde(&format!(
                "~/.local/state/anyrun/raindrop-favicons/{}.ico",
                i.id
            )))),
            id: RNone,
        })
        .take(state.config.max_entries)
        .collect()
}

#[handler]
fn handler(selection: Match, state: &State) -> HandleResult {
    let item = state
        .items
        .iter()
        .find(|i| i.title == selection.title.as_str());

    if let Some(item) = item {
        let command = if let Some(command_holder) = &state.config.command_holder {
            command_holder.replace("{}", &format!("xdg-open {}", &item.link))
        } else {
            format!("xdg-open {}", &item.link)
        };
        eprintln!("command: {}", command);
        Command::new("sh").arg("-c").arg(command).spawn().unwrap();
    }
    HandleResult::Close
}
