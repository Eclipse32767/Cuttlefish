use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str, to_string};
use std::fs::read_to_string;
use std::env;
use std::process::Command;
use std::fs;
use langswaycfg::get_lang;
use crate::langswaycfg;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Border {
    #[default]
    No,
    Normal,
    Csd,
    Pixel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShortcutKey {
    #[default]
    Super,
    Alt,
    Shift,
    Ctrl
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindKey {
    #[default]
    PrimaryKey,
    SecondaryKey,
    BothKey
}

impl Border {
    pub(crate) const ALL: [Border; 4] = [
        Border::No,
        Border::Normal,
        Border::Csd,
        Border::Pixel,
    ];
}
impl ShortcutKey {
    pub(crate) const ALL: [ShortcutKey; 4] = [
        ShortcutKey::Super,
        ShortcutKey::Alt,
        ShortcutKey::Shift,
        ShortcutKey::Ctrl,
    ];
}
impl BindKey {
    pub(crate) const ALL: [BindKey; 3] = [
        BindKey::PrimaryKey,
        BindKey::SecondaryKey,
        BindKey::BothKey,
    ];
}

impl std::fmt::Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                Border::No => pretty.borderno,
                Border::Normal => pretty.bordernormal,
                Border::Csd => pretty.bordercsd,
                Border::Pixel => pretty.borderpixel,
            }
        )
    }
}

impl std::fmt::Display for ShortcutKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                ShortcutKey::Super => pretty.keysuper,
                ShortcutKey::Alt => pretty.keyalt,
                ShortcutKey::Shift => pretty.keyshift,
                ShortcutKey::Ctrl => pretty.keyctrl,
            }
        )
    }
}
impl std::fmt::Display for BindKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                BindKey::PrimaryKey => pretty.bindpri,
                BindKey::SecondaryKey => pretty.bindsec,
                BindKey::BothKey => pretty.bindboth
            }
        )
    }
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
pub fn decodetheme(x: &str, default: iced::Theme) -> iced::Theme {
    match x {
        "dark" => iced::Theme::Dark,
        "light" => iced::Theme::Light,
        &_ => default
    }
}
pub fn decodeborder(x: &str, default: Border) -> Option<Border> {
    Some(match x {
        "none" => Border::No,
        "csd" => Border::Csd,
        "normal" => Border::Normal,
        "pixel" => Border::Pixel,
        &_ => default
    })
}
pub fn decodepri(x: &str, default: ShortcutKey) -> Option<ShortcutKey> {
    Some(match x {
        "super" => ShortcutKey::Super,
        "alt" => ShortcutKey::Alt,
        "control" => ShortcutKey::Ctrl,
        "shift" => ShortcutKey::Shift,
        &_ => default
    })
}
pub fn decodeheader(x: &str, default: BindKey) -> Option<BindKey> {
    Some(match x {
        "pri" => BindKey::PrimaryKey,
        "sec" => BindKey::SecondaryKey,
        "both" => BindKey::BothKey,
        &_ => default
    })
}
pub fn encodetheme(x: iced::Theme) -> &'static str {
    match x {
        iced::Theme::Dark => "dark",
        iced::Theme::Light => "light",
        iced::Theme::Custom(..) => "light"
    }
}
pub fn encodeborder(x: Option<Border>) -> &'static str {
    match x.unwrap() {
        Border::No => "none",
        Border::Normal => "normal",
        Border::Csd => "csd",
        Border::Pixel => "pixel"
    }
}
pub fn encodepri(x: Option<ShortcutKey>) -> &'static str {
    match x.unwrap() {
        ShortcutKey::Super => "super",
        ShortcutKey::Alt => "alt",
        ShortcutKey::Ctrl => "control",
        ShortcutKey::Shift => "shift"
    }
}
pub fn encodeheader(x: Option<BindKey>) -> &'static str {
    match x.unwrap() {
        BindKey::PrimaryKey => "pri",
        BindKey::SecondaryKey => "sec",
        BindKey::BothKey => "both"
    }
}
pub fn rip_shortcut(opt: Option<ShortcutKey>) -> &'static str {
    match opt.unwrap() {
        ShortcutKey::Alt => "Mod1",
        ShortcutKey::Ctrl => "Control",
        ShortcutKey::Shift => "Shift",
        ShortcutKey::Super => "Mod4"
    }
}
pub fn rip_bind(opt: Option<BindKey>) -> &'static str {
    match opt.unwrap() {
        BindKey::PrimaryKey => "$pri",
        BindKey::SecondaryKey => "$sec",
        BindKey::BothKey => "$pri+$sec"
    }
}
pub fn rip_border(opt:  Option<Border>) -> &'static str {
    match opt.unwrap() {
        Border::No => "none",
        Border::Csd => "csd",
        Border::Normal => "normal",
        Border::Pixel => "pixel"
    }
}
pub fn mkwmcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String, border: Option<Border>, width: i32) {
    let home = get_home();
    let data;
    let primary = rip_shortcut(primary_key);
    let secondary = rip_shortcut(secondary_key);
    let exith = rip_bind(exit_header);
    let exitk = &exit_key;
    let launchh = rip_bind(launch_header);
    let launchk = &launch_key;
    let killh = rip_bind(kill_header);
    let killk = &kill_key;
    let minih = rip_bind(mini_header);
    let minik = &mini_key;
    let scratchh = rip_bind(scratch_header);
    let scratchk = &scratch_key;
    let borderval = rip_border(border);
    let widthval = &width;
    let path = format!("{home}/sway/cfgvars");
    data = format!("#AUTO-GENERATED CONFIG, do not edit, any changed will be overwritten\ndefault_border {borderval} {widthval} \nset $pri {primary}\nset $sec {secondary}\n \nset $exit {exith}+{exitk}\nset $launch {launchh}+{launchk}\nset $kill {killh}+{killk}\nset $mini {minih}+{minik}\nset $scratch {scratchh}+{scratchk}");

    fs::write(path, data).expect("failed to write file");

    Command::new("swaymsg")
        .arg("reload")
        .spawn()
        .expect("oops, swaymsg failed, do you have sway installed?");
}
pub fn mkselfcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String, border: Option<Border>, width: i32, theme: iced::Theme) {
    let home = get_home();
    let path = format!("{home}/swaycfg/swaycfg.toml");
    let data = FileData{
        theme: encodetheme(theme.clone()).to_string(),
        border: encodeborder(border).to_string(),
        width: width,
        primary: encodepri(primary_key).to_string(),
        secondary: encodepri(secondary_key).to_string(),
        exith: encodeheader(exit_header).to_string(),
        exitk: exit_key.clone(),
        launchh: encodeheader(launch_header).to_string(),
        launchk: launch_key.clone(),
        killh: encodeheader(kill_header).to_string(),
        killk: kill_key.clone(),
        minih: encodeheader(mini_header).to_string(),
        minik: mini_key.clone(),
        scratchh: encodeheader(scratch_header).to_string(),
        scratchk: scratch_key.clone()
    };
    let toml = to_string(&data).expect("failed to generate toml");
    fs::write(path, toml).expect("failed to write swaycfg.toml");
}