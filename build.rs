extern crate cc;

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

fn config(config_file: &str, source_dir: &str, target_dir: &str) {
    // let config_file = "config.h";
    let source_dir = Path::new(source_dir);
    let target_dir = Path::new(target_dir);

    println!("config file: {}", config_file);
    println!("source dir: {}", source_dir.display());
    println!("target dir: {}", target_dir.display());

    if !target_dir.join(config_file).exists() {
        let mut source_file = File::open(source_dir.join(config_file)).expect("open file failed");
        let mut target_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(target_dir.join(config_file))
            .expect("create file failed");

        let mut buffer = [0; 1024];
        loop {
            let bytes_read = source_file
                .read(&mut buffer)
                .expect("read config file failed");
            if bytes_read == 0 {
                break;
            }
            target_file
                .write_all(&buffer[0..bytes_read])
                .expect("write config file failed");
        }
    }
}

fn get_files(path: &str, extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read directory entry");
        let path = entry.path();
        for extension in extensions.iter() {
            if path.extension().unwrap_or_default() == *extension {
                files.push(path);
                break;
            }
        }
    }
    files
}

fn main() {
    const SOURCE_EXTENSIONS: [&str; 2] = ["cpp", "c"];
    const BASE_PATH: &str = "SEAL/native/src";

    const CONFIG_PATH: &str = "config";
    const UTIL_PATH: &str = "SEAL/native/src/seal/util";
    const CONFIG_H: &str = "config.h";

    config(CONFIG_H, CONFIG_PATH, UTIL_PATH);

    let mut base_config = cc::Build::new();

    let seal_files = get_files(&format!("{}/{}", BASE_PATH, "seal"), &SOURCE_EXTENSIONS);
    let util_files = get_files(
        &format!("{}/{}", BASE_PATH, "seal/util"),
        &SOURCE_EXTENSIONS,
    );

    base_config
        .cpp(true)
        .include(BASE_PATH)
        .files(seal_files)
        .files(util_files)
        .flag("-O3")
        .compile("libseal.a");

    // bindgen::Builder::default()
    //     // .header(format!("{}{}", BASE_PATH, "/seal/seal.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/batchencoder.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/ciphertext.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/ckks.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/decryptor.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/context.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/decryptor.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/dynarray.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/encryptionparams.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/encryptor.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/evaluator.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/galoiskeys.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/keygenerator.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/memorymanager.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/modulus.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/plaintext.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/publickey.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/randomgen.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/randomtostd.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/relinkeys.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/secretkey.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/serializable.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/serialization.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/valcheck.h"))
    //     .header(format!("{}{}", BASE_PATH, "/seal/version.h"))
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file(
    //         Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR env not exists"))
    //             .join("bindings.rs"),
    //     )
    //     .expect("Couldn't write bindings!");
}
