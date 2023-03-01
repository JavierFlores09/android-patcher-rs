use std::env;
use std::io;
use std::path::Path;
use std::fs::File;

fn main() -> io::Result<()> {
    let path = env::args().skip(1).next();

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

        let mut file = File::open(path)?;
        decompress_apk(&mut file);
    }
    //while let Some(argument) = env::args().skip(1).next() {
    //    if argument == "" {
    //        break;
    //    }
    //}
    Ok(())
}


fn decompress_apk(file: &mut File) {
    todo!("Implement zip decompressor, all the libraries suck ass");
}
fn usage(err: &str) {
    eprintln!("Error: {err}");
    eprintln!("");
    eprintln!("Usage:");
    eprintln!("    android-patcher some-apk.apk");
    std::process::exit(1);
}
