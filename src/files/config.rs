use crate::files::database::MODELS;
use crate::models::config_file::ConfigFile;
use crate::models::service::MessagingService;
use directories::{BaseDirs, ProjectDirs};
use native_db::Builder;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "Julien-cpsn";
const APPLICATION: &str = "mayday";

pub fn parse_config_directory<'a>() -> Vec<MessagingService<'a>> {
    let config_dir = get_config_dir();
    let data_dir = get_data_dir();

    let mut services = vec![];
    let paths = config_dir.read_dir().expect("Could not read config directory");

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir() {
            continue;
        }

        //dbg!(&path);

        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".toml") {
            let message_service = parse_config_file(&path, &data_dir);
            services.push(message_service);
        }
    }

    return services;
}

fn parse_config_file<'a>(path: &PathBuf, data_dir: &PathBuf) -> MessagingService<'a> {
    let mut file_content = String::new();

    let mut config_file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open config file");

    config_file.read_to_string(&mut file_content).expect("Could not read config file");

    let config_file = toml::from_str::<ConfigFile>(&file_content).expect("Could not parse config file");

    let driver = config_file.driver.get_driver_config();

    let db_path = data_dir.join(config_file.uuid.to_string());
    let db = Builder::new().create(&MODELS, db_path).expect("Could not create database");
    
    MessagingService {
        uuid: config_file.uuid,
        discussion_name: config_file.discussion_name,
        tmp_messages: vec![],
        db,
        driver,
    }
}

fn get_config_dir() -> PathBuf {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(project_dir) => {
            let config_dir = project_dir.config_local_dir();

            if !config_dir.exists() {
                fs::create_dir_all(config_dir).expect(&format!("Could not recursively create folder \"{}\"", config_dir.display()));
            }

            config_dir.to_path_buf()
        },
        None => panic!("Could not get config directory")
    }
}

fn get_data_dir() -> PathBuf {
    let base_dir = BaseDirs::new().unwrap();

    let data_dir = base_dir.data_local_dir().join("mayday");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect(&format!("Could not create data directory \"{}\"", data_dir.display()));
    }

    data_dir.to_path_buf()
}