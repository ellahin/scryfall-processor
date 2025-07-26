use console::{style, Emoji};
use image;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;
use std::{fs, io::Write};

use std::fs::read_dir;

use img_hash::{HashAlg, HasherConfig};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HashData {
    hash: String,
    id: String,
}

static TASK_SIZE: u64 = 10;
static HASH_SIZE: u32 = 16;
static HASH_ALG: HashAlg = HashAlg::Gradient;

#[tokio::main]
async fn main() {
    let export_image_root = Path::new("./output/images");

    if !export_image_root.exists() {
        panic!("Cannot access image folder");
    }

    let export_file = Path::new("./output/hashes.json");

    if export_file.exists() {
        fs::remove_file(&export_file).expect("Cannot delete hashes.json");
    }

    let dir = read_dir(export_image_root).expect("Cannot read files in image folder");

    let mut count: u64 = 0;

    let mut task_split = Vec::new();

    let mut split_buff = Vec::new();

    for file in dir {
        if split_buff.len() as u64 >= TASK_SIZE {
            task_split.push(split_buff.clone());
            split_buff = Vec::new();
        }

        let temp = file.unwrap();

        let metta = temp.metadata().expect(&format!(
            "Cannot read file: {}",
            temp.path().to_str().unwrap()
        ));

        if metta.is_file() {
            split_buff.push(temp.path());
            count += 1;
        }
    }

    task_split.push(split_buff);

    let pb = ProgressBar::new(count);

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {eta} {msg})")
        .unwrap()
        .progress_chars("#>-"));

    let mut threads = tokio::task::JoinSet::new();

    for files in task_split {
        let pb_clone = pb.clone();

        threads.spawn(async move {
            let mut hashes: Vec<HashData> = Vec::new();
            let hasher = HasherConfig::new()
                .hash_size(HASH_SIZE, HASH_SIZE)
                .hash_alg(HASH_ALG)
                .to_hasher();

            for file in files {
                let image = match image::open(&file) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Cannot open file {}: {}", file.to_str().unwrap(), e);
                        continue;
                    }
                };

                let hash = hasher.hash_image(&image).to_base64();

                let id = file.file_stem().unwrap().to_string_lossy().to_string();

                hashes.push(HashData { hash, id });

                pb_clone.inc(1);
            }

            return hashes;
        });
    }

    let hash_matrix = threads.join_all().await;

    let mut hashes: Vec<HashData> = Vec::new();

    for hvec in hash_matrix {
        for hash in hvec {
            hashes.push(hash.clone());
        }
    }

    let data = serde_json::to_string(&hashes).unwrap();

    let mut hash_json_file = fs::File::create(&export_file).expect("cannot created hashes.json");

    hash_json_file
        .write_all(&data.as_bytes())
        .expect("cannot write to hashes.json");

    hash_json_file.flush().unwrap();
}
