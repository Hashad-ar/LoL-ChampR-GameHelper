use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub label: String,
    pub value: String,
    pub is_aram: Option<bool>,
    pub is_urf: Option<bool>,
}

pub const CDN_UNPKG: &str = "https://unpkg.com";
pub const CDN_DDRAGON: &str = "https://ddragon.leagueoflegends.com";

pub async fn fetch_lol_latest_version() -> anyhow::Result<String> {
    let url = format!("{cdn}/api/versions.json", cdn = CDN_DDRAGON);
    let resp = reqwest::get(url).await?;
    let list = resp.json::<Vec<String>>().await?;
    Ok(list.first().unwrap().to_string())
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampListResp {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampInfo {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    // pub blurb: String,
    // pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    // pub partype: String,
    // pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub async fn fetch_champ_list(version: &String) -> anyhow::Result<ChampListResp> {
    let url = format!(
        "{cdn}/cdn/{version}/data/en_US/champion.json",
        cdn = CDN_DDRAGON,
        version = version
    );
    let resp = reqwest::get(url).await?;
    let data = resp.json::<ChampListResp>().await?;
    Ok(data)
}

pub async fn fetch_latest_champion_list() -> anyhow::Result<ChampListResp> {
    let v = fetch_lol_latest_version().await?;
    let list = fetch_champ_list(&v).await?;
    Ok(list)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneListItem {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub runes: Vec<SlotRuneItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotRuneItem {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
}

pub async fn fetch_rune_list(version: &String) -> anyhow::Result<Vec<RuneListItem>> {
    let url = format!("{CDN_DDRAGON}/cdn/{version}/data/en_US/runesReforged.json");
    let resp = reqwest::get(url).await?;
    let data = resp.json::<Vec<RuneListItem>>().await?;
    Ok(data)
}

pub async fn fetch_latest_rune_list() -> anyhow::Result<(Vec<RuneListItem>, String)> {
    let v = fetch_lol_latest_version().await?;
    let list = fetch_rune_list(&v).await?;
    Ok((list, v))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampData {
    pub index: u32,
    pub id: String,
    pub version: String,
    pub official_version: String,
    pub timestamp: u64,
    pub alias: String,
    pub name: String,
    pub position: String,
    pub skills: Option<Vec<String>>,
    pub spells: Option<Vec<String>>,
    pub item_builds: Vec<ItemBuild>,
    pub runes: Vec<Rune>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBuild {
    pub title: String,
    pub associated_maps: Vec<u32>,
    pub associated_champions: Vec<u32>,
    pub blocks: Vec<Block>,
    pub map: String,
    pub mode: String,
    pub preferred_item_slots: Option<Vec<Value>>,
    pub sortrank: u32,
    pub started_from: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Option<Vec<Item>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub alias: String,
    pub name: String,
    pub position: String,
    pub pick_count: u64,
    pub win_rate: String,
    pub primary_style_id: u32,
    pub sub_style_id: u32,
    pub selected_perk_ids: Vec<u32>,
    pub score: f64,
}

pub async fn fetch_champ_file(
    source: &String,
    version: &String,
    champ_name: &String,
) -> anyhow::Result<Option<Vec<ChampData>>> {
    let url = format!(
        "{cdn}/{source}@{version}/{champ_name}.json",
        cdn = CDN_UNPKG,
        source = source,
        version = version,
        champ_name = champ_name,
    );
    println!("fetching champ file {}", &url);

    let resp = reqwest::get(&url).await?;
    if !resp.status().is_success() {
        println!("fetch champ file failed, {} {}", &source, &champ_name);
    }

    match resp.json::<Vec<ChampData>>().await {
        Ok(data) => Ok(Some(data)),
        Err(e) => {
            println!("failed {}, {:?}", url, e.to_string());
            Ok(None)
        }
    }
}

pub fn get_alias_from_champion_map(
    champion_map: &HashMap<String, ChampInfo>,
    champion_id: i64,
) -> String {
    let mut ret = String::new();
    for (alias, c) in champion_map.into_iter() {
        if c.key.eq(&champion_id.to_string()) {
            ret = alias.to_string();
            break;
        }
    }

    ret
}

pub async fn get_latest_source_version(source: &String) -> anyhow::Result<String> {
    let url = format!("https://mirrors.cloud.tencent.com/npm/@champ-r/{source}/latest");
    let pkg: Value = reqwest::get(url).await?.json().await?;
    let v = pkg["sourceVersion"].as_str().unwrap();
    Ok(v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_latest_champs() {
        let v = match fetch_lol_latest_version().await {
            Ok(v) => v,
            Err(e) => panic!("get latest lol version error, {:?}", e),
        };

        match fetch_champ_list(&v).await {
            Ok(resp) => {
                println!("{:?}", resp);
                println!("Total: {:?}, version: {:?}", resp.data.len(), resp.version);
            }
            Err(e) => panic!("get champ list error, {:?}", e),
        };
    }
}
