use image;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::path::Path;
use std::{fs, io::Write};

use std::fs::read_dir;

use img_hash::{HashAlg, HashBytes, HasherConfig, ImageHash};

static HASH_SIZE: u32 = 16;
static HASH_ALG: HashAlg = HashAlg::Gradient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HashData {
    hash: String,
    id: String,
}

struct HighHash {
    dist: u32,
    id: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    let path = Path::new(&query);

    if !path.exists() {
        panic!("File {} doesn't exist", query);
    }

    let hasher = HasherConfig::new()
        .hash_size(HASH_SIZE, HASH_SIZE)
        .hash_alg(HASH_ALG)
        .to_hasher();

    let image = match image::open(&path) {
        Ok(e) => e,
        Err(e) => {
            panic!("Cannot open file {}: {}", query, e);
        }
    };

    let hash = hasher.hash_image(&image);

    println!("Hash: {}", hash.to_base64());

    let hash_data: Vec<HashData> = serde_json::from_str(
        &fs::read_to_string("./output/hashes.json").expect("cannot open hashes.json"),
    )
    .expect("Cannot read json from hashes.json");

    let mut high: Option<HighHash> = None;

    let pb = ProgressBar::new(hash_data.len() as u64);

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {eta} {msg})")
        .unwrap()
        .progress_chars("#>-"));

    for data in hash_data {
        let data_hash = ImageHash::<Box<[u8]>>::from_base64(data.hash.clone().as_str())
            .expect("Cannot extract hash");

        let dist = data_hash.dist(&hash);

        match high {
            Some(ref e) => {
                if dist < e.dist {
                    high = Some(HighHash {
                        dist,
                        id: data.id.clone(),
                    });
                }
            }
            None => {
                high = Some(HighHash {
                    dist,
                    id: data.id.clone(),
                });
            }
        }

        pb.inc(1);
    }

    let high = match high {
        Some(e) => e,
        None => panic!("Somethnig fucked up"),
    };

    println!("ID: {}", high.id);
    println!("Dist: {}", high.dist);
}
