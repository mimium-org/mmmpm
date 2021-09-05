extern crate clap;
extern crate dirs;
extern crate log;

mod constant;

use std::path::PathBuf;

use log::{error, info, LevelFilter};

use mmmpm_host::Archive;
use mmmpm_package::UndeterminedPackage;
use mmmpm_storage::{Path, StorageOperation};
use mmmpm_storage_filesystem::FilesystemStorage;

struct MmmpmConfig {
    fs: FilesystemStorage,
}

fn configure_mmmpm() -> MmmpmConfig {
    let mut fs: Option<FilesystemStorage> = None;

    match dirs::home_dir() {
        Some(mut path) => {
            path.push(constant::MMMPM_DIR);
            info!("mmmpm directory = {:?}", path);
            fs = Some(FilesystemStorage::new(&path));

            if let Err(err) = config.fs.connect() {
                error!("cannot start filesystem session because {:?}", err);
                return;
            }
        }

        None => {
            error!("cannot found your home directory.");
            return;
        }
    }

    MmmpmConfig { fs: fs.unwrap() }
}

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    env_logger::builder()
        .filter_level(if matches.is_present("verbose") {
            LevelFilter::Info
        } else {
            LevelFilter::Error
        })
        .format_level(false)
        .format_timestamp(None)
        .init();

    let config = configure_mmmpm();

    match matches.subcommand() {
        ("install", Some(_)) => println!("subcommand: install"),

        ("list", Some(_)) => println!("subcommand: list"),
        ("run", Some(_)) => println!("subcommand: run"),
        _ => println!("{}", matches.usage()),
    }

    //// インストール処理の実験

    // パッケージ種類の判定っぽいコード
    let pkg_name = UndeterminedPackage::new("github.com:t-sin/koto".to_string());
    let mut archive: Option<Archive> = None;
    match pkg_name.determine() {
        Some(pkgdsn) => {
            println!("{:?}", pkgdsn.name());
            let host = pkgdsn.host();
            let mut pkg_exists = false;

            // パッケージが確定できたので存在確認
            match host.exists() {
                Ok(exists) => {
                    println!("result = {}", exists);
                    pkg_exists = true;
                }
                Err(err) => println!("error: {:?}", err),
            }

            if pkg_exists {
                // パッケージが存在したのでアーカイブの取得
                match host.retrieve() {
                    Ok(retrieved) => {
                        println!("retrived.");
                        archive = Some(retrieved);
                    }
                    Err(err) => println!("error: {:?}", err),
                }
            }
        }
        None => {
            println!("none!!");
            return;
        }
    }

    // GitHubパッケージのフォルタ存在確認（なければつくる）
    let github_dir = Path::new(vec![constant::MMMPM_DIR, "GitHub"]);
    match config.fs.object_exists(&github_dir) {
        Ok(exists) => {
            if exists {
                println!("{:?} exists.", github_dir);
            } else {
                println!("{:?} does not exists.", github_dir);
                match config.fs.create_dir(&github_dir) {
                    Ok(_) => {}
                    Err(err) => panic!("cannot create {:?}", &github_dir),
                }
                println!("{:?} is created.", github_dir);
            }
        }
        Err(err) => panic!("error: {:?}"),
    }

    // パッケージをGitHubのところに置く
    panic!("not implemented!");
}
