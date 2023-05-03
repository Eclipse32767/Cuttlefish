use std::fs::read_to_string;
use libcfg::get_home;
use crate::libcfg;

pub fn langstr(app: &str, placeholder: &str) -> String {
    let locale = whoami::lang().collect::<Vec<String>>();
    let lang = locale[1].clone();
    let home = get_home();
    let langpath = format!("{home}/Oceania/locale/{app}/{lang}.toml");
    let backup_path = format!("{home}/Oceania/locale/{app}");
    let langfile = match read_to_string(langpath.clone()) {
        Ok(x) => x,
        Err(..) => {
            std::process::Command::new("mkdir").arg("-p").arg(backup_path).output().expect("this should never happen");
            std::fs::write(langpath, placeholder.clone()).expect("failed to write backup locale"); 
            String::from(placeholder)
        }
    };
    langfile
}