use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::alloc;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::iter;
use std::path::Path;
use std::str;

use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkDataFiles {
    pub object: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<BulkDataFilesData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkDataFilesData {
    pub object: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub uri: String,
    pub name: String,
    pub description: String,
    pub size: i64,
    #[serde(rename = "download_uri")]
    pub download_uri: String,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "content_encoding")]
    pub content_encoding: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub object: String,
    pub id: String,
    #[serde(rename = "oracle_id")]
    pub oracle_id: Option<String>,
    #[serde(rename = "multiverse_ids")]
    pub multiverse_ids: Vec<i64>,
    pub name: String,
    #[serde(rename = "printed_name")]
    pub printed_name: Option<String>,
    pub lang: String,
    #[serde(rename = "released_at")]
    pub released_at: String,
    pub uri: String,
    #[serde(rename = "scryfall_uri")]
    pub scryfall_uri: String,
    pub layout: String,
    #[serde(rename = "highres_image")]
    pub highres_image: bool,
    #[serde(rename = "image_status")]
    pub image_status: String,
    #[serde(rename = "image_uris")]
    pub image_uris: Option<ImageUris>,
    #[serde(rename = "mana_cost")]
    pub mana_cost: Option<String>,
    pub cmc: Option<f64>,
    #[serde(rename = "type_line")]
    pub type_line: Option<String>,
    #[serde(rename = "printed_type_line")]
    pub printed_type_line: Option<String>,
    #[serde(rename = "oracle_text")]
    pub oracle_text: Option<String>,
    #[serde(rename = "printed_text")]
    pub printed_text: Option<String>,
    pub colors: Option<Vec<String>>,
    #[serde(rename = "color_identity")]
    pub color_identity: Vec<String>,
    pub keywords: Vec<String>,
    pub legalities: Legalities,
    pub games: Vec<String>,
    pub reserved: bool,
    #[serde(rename = "game_changer")]
    pub game_changer: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    #[serde(rename = "set_id")]
    pub set_id: String,
    pub set: String,
    #[serde(rename = "set_name")]
    pub set_name: String,
    #[serde(rename = "set_type")]
    pub set_type: String,
    #[serde(rename = "set_uri")]
    pub set_uri: String,
    #[serde(rename = "set_search_uri")]
    pub set_search_uri: String,
    #[serde(rename = "scryfall_set_uri")]
    pub scryfall_set_uri: String,
    #[serde(rename = "rulings_uri")]
    pub rulings_uri: String,
    #[serde(rename = "prints_search_uri")]
    pub prints_search_uri: String,
    #[serde(rename = "collector_number")]
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    #[serde(rename = "flavor_text")]
    pub flavor_text: Option<String>,
    #[serde(rename = "card_back_id")]
    pub card_back_id: Option<String>,
    pub artist: Option<String>,
    #[serde(rename = "artist_ids")]
    pub artist_ids: Option<Vec<String>>,
    #[serde(rename = "illustration_id")]
    pub illustration_id: Option<String>,
    #[serde(rename = "border_color")]
    pub border_color: String,
    pub frame: String,
    #[serde(rename = "full_art")]
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    #[serde(rename = "story_spotlight")]
    pub story_spotlight: bool,
    #[serde(rename = "edhrec_rank")]
    pub edhrec_rank: Option<i64>,
    #[serde(rename = "penny_rank")]
    pub penny_rank: Option<i64>,
    pub prices: Prices,
    #[serde(rename = "related_uris")]
    pub related_uris: RelatedUris,
    #[serde(rename = "purchase_uris")]
    pub purchase_uris: Option<PurchaseUris>,
    #[serde(rename = "tcgplayer_id")]
    pub tcgplayer_id: Option<i64>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub watermark: Option<String>,
    #[serde(rename = "all_parts")]
    pub all_parts: Option<Vec<AllPart>>,
    #[serde(rename = "security_stamp")]
    pub security_stamp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    #[serde(rename = "art_crop")]
    pub art_crop: String,
    #[serde(rename = "border_crop")]
    pub border_crop: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legalities {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub timeless: String,
    pub gladiator: String,
    pub pioneer: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub oathbreaker: String,
    pub standardbrawl: String,
    pub brawl: String,
    pub alchemy: String,
    pub paupercommander: String,
    pub duel: String,
    pub oldschool: String,
    pub premodern: String,
    pub predh: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub usd: Option<String>,
    #[serde(rename = "usd_foil")]
    pub usd_foil: Value,
    #[serde(rename = "usd_etched")]
    pub usd_etched: Value,
    pub eur: Value,
    #[serde(rename = "eur_foil")]
    pub eur_foil: Value,
    pub tix: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedUris {
    pub gatherer: Option<String>,
    #[serde(rename = "tcgplayer_infinite_articles")]
    pub tcgplayer_infinite_articles: Option<String>,
    #[serde(rename = "tcgplayer_infinite_decks")]
    pub tcgplayer_infinite_decks: Option<String>,
    pub edhrec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseUris {
    pub tcgplayer: String,
    pub cardmarket: String,
    pub cardhoarder: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPart {
    pub object: String,
    pub id: String,
    pub component: String,
    pub name: String,
    #[serde(rename = "type_line")]
    pub type_line: String,
    pub uri: String,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static DOWNARROW: Emoji<'_, '_> = Emoji("🔻 ", ":-)");

#[tokio::main]
async fn main() {
    let export_root = Path::new("./output");
    let export_image_root = Path::new("./output/images");

    if !export_root.exists() {
        fs::create_dir(&export_root).expect("Error cannot create export folder");
    }

    if !export_image_root.exists() {
        fs::create_dir(&export_image_root).expect("Error cannot create export folder");
    }

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Cannot Bulid Client");

    let bulk_data: BulkDataFiles = match client
        .get("https://api.scryfall.com/bulk-data")
        .send()
        .await
    {
        Ok(e) => match e.text().await {
            Ok(s) => match serde_json::from_str(&s) {
                Ok(d) => d,
                Err(_) => panic!("Cannot deserialize data from bulk api"),
            },
            Err(_) => panic!("Cannot get text from bulk-data api"),
        },
        Err(e) => panic!("Cannot get bulk-data api: {}", e),
    };

    let mut all_data_uri: Option<String> = None;

    for data in bulk_data.data {
        if data.name == "All Cards" {
            all_data_uri = Some(data.download_uri.clone());
        }
    }

    let all_data_uri = match all_data_uri {
        Some(e) => e,
        None => panic!("cannot find All Data URL"),
    };

    let all_data_json_path = export_root.join("all_data.json");

    if all_data_json_path.exists() {
        fs::remove_file(&all_data_json_path).expect("Cannot delete all_data.json");
    }

    println!(
        "{} {}Downloading json...",
        style("[1/4]").bold().dim(),
        DOWNARROW
    );

    let mut all_data_json_file =
        fs::File::create(&all_data_json_path).expect("cannot created all_data.json");

    let mut res = reqwest::get(all_data_uri).await.expect("test");

    let length = res.content_length().unwrap();

    let pb = ProgressBar::new(length);

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
        .unwrap()
        .progress_chars("#>-"));

    while let Some(chunk) = res.chunk().await.expect("test") {
        pb.inc(chunk.len() as u64);
        all_data_json_file
            .write_all(&chunk)
            .expect("cannot write to all_data.json");
    }

    pb.finish_with_message("downloaded");

    all_data_json_file
        .flush()
        .expect("cannot write to all_data.json");

    println!(
        "{} {}Loading json...",
        style("[2/4]").bold().dim(),
        LOOKING_GLASS
    );

    let card_data: Vec<CardData> = serde_json::from_str(
        &fs::read_to_string(&all_data_json_path).expect("cannot open all_data.json"),
    )
    .expect("Cannot read json from all_data.json");

    println!("{} {}Batching Tasks...", style("[3/4]").bold().dim(), TRUCK);

    let total_size = card_data.len() as u64;

    let mut task_split: Vec<Vec<CardData>> = Vec::new();

    let mut split_buff: Vec<CardData> = Vec::new();

    for card in card_data {
        if split_buff.len() >= 10000 {
            task_split.push(split_buff.clone());
            split_buff = Vec::new();
        }
        split_buff.push(card);
    }

    task_split.push(split_buff);

    println!(
        "{} {}Downloading Images...",
        style("[4/4]").bold().dim(),
        DOWNARROW
    );

    let pb_new = ProgressBar::new(total_size);

    pb_new.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {eta} {msg})")
        .unwrap()
        .progress_chars("#>-"));

    let mut threads = tokio::task::JoinSet::new();

    for task in task_split {
        let pb_clone = pb_new.clone();
        let client_clone = client.clone();
        threads.spawn(async move {
            for card in task {
                let card_path = export_image_root.join(format!("{}.png", card.id));

                if card_path.exists() {
                    pb_clone.inc(1);
                    continue;
                }

                match card.image_uris {
                    Some(e) => {
                        let image = client_clone
                            .get(&e.png)
                            .send()
                            .await
                            .expect(&format!("download image {}", e.png))
                            .bytes()
                            .await
                            .expect(&format!("download image {}", e.png));

                        let mut card_file = fs::File::create(&card_path).expect(&format!(
                            "cannot create file {}",
                            card_path.to_str().unwrap()
                        ));

                        card_file.write_all(&image).expect(&format!(
                            "cannot wirte to file {}",
                            card_path.to_str().unwrap()
                        ));

                        card_file.flush().expect(&format!(
                            "cannot wirte to file {}",
                            card_path.to_str().unwrap()
                        ));
                    }
                    None => (),
                };
                pb_clone.inc(1);
            }
        });
    }

    threads.join_all().await;

    pb_new.finish_with_message("downloaded");
}
