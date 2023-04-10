use std::fs::read_to_string;
use libcfg::get_home;
use crate::libcfg;

pub fn langstr(app: &str) -> String {
    let locale = whoami::lang().collect::<Vec<String>>();
    let lang = locale[1].clone();
    let home = get_home();
    let langpath = format!("{home}/swaycfg/locale/{app}/{lang}.toml");
    let langfile = read_to_string(langpath).expect("no locale found");
    langfile
}