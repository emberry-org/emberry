use lazy_static::lazy_static;
use log::info;
use std::{env::args, path::PathBuf};

lazy_static! {
  pub static ref DATA: PathBuf = data_dir();
  pub static ref CACHE: PathBuf = cache_dir();
  pub static ref CONFIG: PathBuf = config_dir();
}

fn data_dir() -> PathBuf {
  let warehouse = if let Some(mut data_dir) = tauri::api::path::data_dir() {
    data_dir.push("emberry");
    data_dir
  } else if let Some(data_dir) = args().next() {
    let mut data_dir = PathBuf::from(data_dir);
    data_dir.pop();
    data_dir.push(".data");
    data_dir
  } else {
    "./.data".into()
  };

  info!("using data dir at: {:?}", warehouse);
  warehouse
}

fn cache_dir() -> PathBuf {
  let cache = if let Some(mut cache_dir) = tauri::api::path::cache_dir() {
    cache_dir.push("emberry");
    cache_dir
  } else if let Some(cache_dir) = args().next() {
    let mut cache_dir = PathBuf::from(cache_dir);
    cache_dir.pop();
    cache_dir.push(".cache");
    cache_dir
  } else {
    "./.cache".into()
  };

  info!("Using cache dir at: {:?}", cache);
  cache
}

fn config_dir() -> PathBuf {
  let config = if let Some(mut config_dir) = tauri::api::path::config_dir() {
    config_dir.push("emberry");
    config_dir
  } else if let Some(config_dir) = args().next() {
    let mut config_dir = PathBuf::from(config_dir);
    config_dir.pop();
    config_dir.push(".config");
    config_dir
  } else {
    "./.config".into()
  };

  info!("Using config dir at: {:?}", config);
  config
}
