extern crate cc;

use std::{
    fs::{read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

fn config(source_path: &Path, destination_path: &Path) {
    let mut source = File::open(source_path).expect("config file not found");
    let mut destination = File::create(destination_path).expect("config file create failed");

    let mut buffer = [0; 1024];
    loop {
        let read_count = source.read(&mut buffer).expect("read config file error");
        if read_count == 0 {
            break;
        }
        destination
            .write_all(&buffer[..read_count])
            .expect("write config file failed");
    }
}

fn get_files(path: &str, extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in read_dir(path).expect("Unable to read directory") {
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

    #[cfg(feature = "cpp17")]
    const CONFIG_SOURCE: &str = "config/config-stdc++17.h";

    #[cfg(not(feature = "cpp17"))]
    const CONFIG_SOURCE: &str = "config/config.h";

    const CONFIG_H: &str = "SEAL/native/src/seal/util/config.h";

    config(Path::new(CONFIG_SOURCE), Path::new(CONFIG_H));

    let mut base_config = cc::Build::new();

    let seal_files = get_files(&format!("{}/{}", BASE_PATH, "seal"), &SOURCE_EXTENSIONS);
    let util_files = get_files(
        &format!("{}/{}", BASE_PATH, "seal/util"),
        &SOURCE_EXTENSIONS,
    );

    #[cfg(feature = "cpp17")]
    base_config.flag("-std=c++17");

    base_config
        .cpp(true)
        .include(BASE_PATH)
        .files(seal_files)
        .files(util_files)
        .flag("-O3")
        .flag("-Wno-all")
        .flag("-Wno-extra")
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
