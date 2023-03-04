use std::env;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::error::Error;

use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1);

    if path.is_none() {
        usage("No path was provided");
    }

    if let Some(path) = path {
        let path = Path::new(&path);

        if !path.exists() {
            usage("Path doesn't exist");
        }

        if path.is_dir() {
            usage("Directories are not supported, please provide an APK file");
        }

        if let Some(path) = path.extension() {
            if path != "apk" {
                usage("Not an APK file");
            }
        }

        let file = File::open(path).expect("Couldn't open file");
        // Does the buffered reader even make a difference here?
        let buf = BufReader::new(file);
        // TODO: Make my own zip crate at some point
        let mut archive = ZipArchive::new(buf).expect("Couldn't read archive");

        let index: usize;
        let mut exact: bool = false;
        if let Some(i) = env::args().nth(2) {
            index = i.parse().unwrap();

            if env::args().nth(3).is_some() {
                exact = true
            }

            if index > archive.len() {
                usage("There's no file at this index in the APK");
            }
        } else {
            index = 0;
        }
        let start = if exact { index } else { 0 };

        let mut last: Vec<String> = Vec::new();
        for i in start..=index {
            let file = archive.by_index_raw(i)?;
            let tree = path_tree(file.name());

            let last_len = last.len();
            let curr_len = tree.len();

            if last_len != 0 &&
            last_len != 1 &&
            last_len == curr_len &&
            last[last_len - 2] == tree[curr_len - 2] {
                println!("{}", &tree[curr_len - 1]);
                continue;
            }


            for p in &tree {
                println!("{}", p);
            }
            last = tree;
        }
    }
    Ok(())
}

fn path_tree(path: &str) -> Vec<String> {
    let splitter = path.split("/");
    splitter.enumerate().map(|(depth, directory)| {
        let padding = "-".repeat(depth * 2) + "| ";
        format!("{}{}", padding, directory)
    }).collect()
}

fn usage(err: &str) {
    eprintln!("Error: {err}");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("    android-patcher <apk file> [file index]");
    eprintln!("Example:");
    eprintln!("   android-patcher sample-apk/tiktok_28.5.3.apk");
    std::process::exit(1);
}
