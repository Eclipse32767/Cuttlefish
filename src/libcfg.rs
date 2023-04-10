use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use std::fs::read_to_string;
use std::env;

#[derive(Deserialize, Debug, Serialize)]
pub struct FileData {
    pub(crate) theme: String,
    pub(crate) border: String,
    pub(crate) width: i32,
    pub(crate) primary: String,
    pub(crate) secondary: String,
    pub(crate) exith: String,
    pub(crate) exitk: String,
    pub(crate) launchh: String,
    pub(crate) launchk: String,
    pub(crate) killh: String,
    pub(crate) killk: String,
    pub(crate) minih: String,
    pub(crate) minik: String,
    pub(crate) scratchh: String,
    pub(crate) scratchk: String
}
pub fn get_home() -> String {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => var,
        Err(..) => match env::var("HOME") {
            Ok(var) => format!("{var}/.config"),
            Err(..) => panic!("Failed to find config directory, make sure XDG_CONFIG_HOME or HOME are set")
        }
    }
}
pub fn getcfgdata() -> FileData {
    let home = get_home();
    let path = format!("{home}/swaycfg/swaycfg.toml");
    let file = match read_to_string(path) {
        Ok(var) => var,
        Err(..) => match read_to_string("/etc/swaycfg/swaycfg.toml") {
            Ok(var) => var,
            Err(..) => panic!("Failed to find swaycfg.toml in any valid directory")
        }
    };
    let decoded: FileData = from_str(&file).unwrap();
    decoded
}
