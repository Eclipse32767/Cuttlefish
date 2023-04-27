#![allow(dead_code)]
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str, to_string};
use std::fs::read_to_string;
use std::{env};
use std::process::Command;
use std::fs;
use langswaycfg::get_lang;
use crate::langswaycfg;

#[derive(Deserialize, Debug, Serialize)]
pub struct FileData {
    pub theme: String,
    pub primary: String,
    pub secondary: String,
    pub exith: String,
    pub exitk: String,
    pub launchh: String,
    pub launchk: String,
    pub killh: String,
    pub killk: String,
    pub minih: String,
    pub minik: String,
    pub scratchh: String,
    pub scratchk: String,
    pub border: Option<Border>,
    pub winanim: String,
    pub workanim: String,
    pub blur: String
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Border {
    pub width: i32,
    pub radius: i32,
    pub gaps: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowAnimation {
    Slide,
    Popin,
    #[default]
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkAnimation {
    Slide,
    SlideVert,
    Fade,
    #[default]
    None
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
impl WindowAnimation {
    pub const ALL: [WindowAnimation; 3] = [
        WindowAnimation::None,
        WindowAnimation::Popin,
        WindowAnimation::Slide
    ];
}
impl WorkAnimation {
    pub const ALL: [WorkAnimation; 4] = [
        WorkAnimation::None,
        WorkAnimation::Slide,
        WorkAnimation::SlideVert,
        WorkAnimation::Fade
    ];
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
impl std::fmt::Display for WindowAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                WindowAnimation::None => pretty.winnone,
                WindowAnimation::Popin => pretty.winpop,
                WindowAnimation::Slide => pretty.winslide
            }
        )
    }
}
impl std::fmt::Display for WorkAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                WorkAnimation::None => pretty.worknone,
                WorkAnimation::Fade => pretty.workfade,
                WorkAnimation::Slide => pretty.workslide,
                WorkAnimation::SlideVert => pretty.workslidev
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
pub fn decodeworkanim(x: &str, default: WorkAnimation) -> Option<WorkAnimation> {
    Some(match x {
        "none" => WorkAnimation::None,
        "fade" => WorkAnimation::Fade,
        "slide" => WorkAnimation::Slide,
        "slidev" => WorkAnimation::SlideVert,
        &_ => default
    })
}
pub fn decodewinanim(x: &str, default: WindowAnimation) -> Option<WindowAnimation> {
    Some(match x {
        "none" => WindowAnimation::None,
        "popin" => WindowAnimation::Popin,
        "slide" => WindowAnimation::Slide,
        &_ => default
    })
}
pub fn decodeblur(x: &str) -> bool{
    match x {
        "y" => true,
        "n" => false,
        &_ => true
    }
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
pub fn encodeworkanim(x: Option<WorkAnimation>) -> &'static str {
    match x.unwrap() {
        WorkAnimation::None => "none",
        WorkAnimation::Fade => "fade",
        WorkAnimation::Slide => "slide",
        WorkAnimation::SlideVert => "slidev",
    }
}
pub fn encodewinanim(x: Option<WindowAnimation>) -> &'static str{
    match x.unwrap() {
        WindowAnimation::None => "none",
        WindowAnimation::Popin => "popin",
        WindowAnimation::Slide => "slide"
    }
}
pub fn encodeblur(x: bool) -> &'static str {
    if x {
        "y"
    } else {
        "n"
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
pub fn mkwmcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String) {
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
    let path = format!("{home}/sway/cfgvars");
    data = format!("#AUTO-GENERATED CONFIG, do not edit, any changed will be overwritten\nset $pri {primary}\nset $sec {secondary}\n \nset $exit {exith}+{exitk}\nset $launch {launchh}+{launchk}\nset $kill {killh}+{killk}\nset $mini {minih}+{minik}\nset $scratch {scratchh}+{scratchk}");

    fs::write(path, data).expect("failed to write file");

    Command::new("swaymsg")
        .arg("reload")
        .spawn()
        .expect("oops, swaymsg failed, do you have sway installed?");
}
pub fn mkselfcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String, theme: iced::Theme, border: Option<Border>, winanim: Option<WindowAnimation>, workanim: Option<WorkAnimation>, blur: bool) {
    let home = get_home();
    let path = format!("{home}/swaycfg/swaycfg.toml");
    let data = FileData{
        theme: encodetheme(theme.clone()).to_string(),
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
        scratchk: scratch_key.clone(),
        border: border.clone(),
        winanim: encodewinanim(winanim).to_string(),
        workanim: encodeworkanim(workanim).to_string(),
        blur: encodeblur(blur).to_string(),
    };
    let toml = to_string(&data).expect("failed to generate toml");
    fs::write(path, toml).expect("failed to write swaycfg.toml");
}