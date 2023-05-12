#![allow(dead_code)]
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str, to_string};
use std::fs::read_to_string;
use std::{env};
use std::process::Command;
use std::fs;
use langcfg::get_lang;
use crate::langcfg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OurTheme {
    Light,
    #[default]
    Dark,
    Custom
}

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

pub enum BarWidget {
    #[default]
    None,
    Audio,
    Backlight,
    Battery,
    Bluetooth,
    Clock,
    CPU,
    Disk,
    KeyboardState,
    RAM,
    Network,
    Temperature,
    Tray,
    Taskbar,
    Workspaces,
    User
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
    let path = format!("{home}/Oceania/cfg.toml");
    let backup_path = format!("{home}/Oceania");
    let placeholder = String::from(r#"
    theme = "light"
    primary = "super"
    secondary = "shift"
    exith = "both"
    exitk = "E"
    launchh = "pri"
    launchk = "Tab"
    killh = "both"
    killk = "Q"
    minih = "both"
    minik = "Z"
    scratchh = "pri"
    scratchk = "Z"
    winanim = "popin"
    workanim = "slidev"
    blur = "y"
    
    [border]
    width = 5
    radius = 15
    gaps = 10"#);
    let file = match read_to_string(path.clone()) {
        Ok(var) => var,
        Err(..) => match read_to_string("/etc/Oceania/cfg.toml") {
            Ok(var) => var,
            Err(..) => {
                std::process::Command::new("mkdir").arg("-p").arg(backup_path).output().expect("uh oh");
                fs::write(path, placeholder.clone()).expect("failed to write backup file");
                placeholder
            }
        }
    };
    let decoded: FileData = from_str(&file).unwrap();
    decoded
}
pub fn decodetheme(x: &str, default: OurTheme) -> OurTheme {
    match x {
        "dark" => OurTheme::Dark,
        "light" => OurTheme::Light,
        "custom" => OurTheme::Custom,
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
pub fn encodetheme(x: OurTheme) -> &'static str {
    match x {
        OurTheme::Dark => "dark",
        OurTheme::Light => "light",
        OurTheme::Custom => "custom"
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
        ShortcutKey::Alt => "ALT",
        ShortcutKey::Ctrl => "CONTROL",
        ShortcutKey::Shift => "SHIFT",
        ShortcutKey::Super => "SUPER"
    }
}
pub fn rip_bind(opt: Option<BindKey>, pri: Option<ShortcutKey>, sec: Option<ShortcutKey>) -> String {
    let pristr = match pri.unwrap() {
        ShortcutKey::Super => "SUPER",
        ShortcutKey::Alt => "ALT",
        ShortcutKey::Ctrl => "CONTROL",
        ShortcutKey::Shift => "SHIFT"
    };
    let secstr = match sec.unwrap() {
        ShortcutKey::Super => "SUPER",
        ShortcutKey::Alt => "ALT",
        ShortcutKey::Ctrl => "CONTROL",
        ShortcutKey::Shift => "SHIFT"
    };
    match opt.unwrap() {
        BindKey::PrimaryKey => pristr.to_string(),
        BindKey::SecondaryKey => secstr.to_string(),
        BindKey::BothKey => format!("{pristr}_{secstr}")
    }
}
pub fn rip_win_anim(opt: Option<WindowAnimation>) -> String{
    match opt.unwrap() {
        WindowAnimation::None => "0,1,default".to_string(),
        WindowAnimation::Popin => "1,8,default,popin".to_string(),
        WindowAnimation::Slide => "1,8,default,slide".to_string()
    }
}
pub fn rip_work_anim(opt: Option<WorkAnimation>) -> String {
    match opt.unwrap() {
        WorkAnimation::None => "0,8,default".to_string(),
        WorkAnimation::Fade => "1,8,default,fade".to_string(),
        WorkAnimation::Slide => "1,8,default,slide".to_string(),
        WorkAnimation::SlideVert => "1,8,default,slidevert".to_string()
    }
}
pub fn mkwmcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String, border: Option<Border>, winanim: Option<WindowAnimation>, workanim: Option<WorkAnimation>, blur: bool) {
    let home = get_home();
    let data;
    let prik = rip_shortcut(primary_key);
    let seck = rip_shortcut(secondary_key);
    let exith = rip_bind(exit_header, primary_key, secondary_key);
    let exitk = &exit_key;
    let launchh = rip_bind(launch_header, primary_key, secondary_key);
    let launchk = &launch_key;
    let killh = rip_bind(kill_header, primary_key, secondary_key);
    let killk = &kill_key;
    let minih = rip_bind(mini_header, primary_key, secondary_key);
    let minik = &mini_key;
    let scratchh = rip_bind(scratch_header, primary_key, secondary_key);
    let scratchk = &scratch_key;
    let gaps = border.unwrap().gaps;
    let width = border.unwrap().width;
    let radius = border.unwrap().radius;
    let win_anim = rip_win_anim(winanim);
    let work_anim = rip_work_anim(workanim);
    let sector_head = r#"{"#;
    let sector_tail = r#"}"#;
    let path = format!("{home}/hypr/hyprland.conf");
    data = format!("#AUTO-GENERATED CONFIG, DO NOT EDIT, CHANGES WILL BE OVERWRITTEN \n \
    source {home}/hypr/usercfg.conf\n \
    exec_once={home}/hypr/autostart\n \
    bind={exith},{exitk},exec,killall Hyprland\n \
    bind={launchh},{launchk},exec,rofi\n \
    bind={killh},{killk},killactive\n \
    bind={minih},{minik},movetoworkspace,special\n \
    bind={scratchh},{scratchk},togglespecialworkspace\n \
    bind = {prik}, left, movefocus, l\n \
    bind = {prik}, right, movefocus, r\n \
    bind = {prik}, up, movefocus, u\n \
    bind = {prik}, down, movefocus, d\n \
    bind = {prik}_{seck}, left, movewindow, l\n \
    bind = {prik}_{seck}, right, movewindow, r\n \
    bind = {prik}_{seck}, up, movewindow, u\n \
    bind = {prik}_{seck}, down, movewindow, d\n \
    bind = {prik},1, workspace, 1 \n \
    bind = {prik},2, workspace, 2 \n \
    bind = {prik},3, workspace, 3 \n \
    bind = {prik},4, workspace, 4 \n \
    bind = {prik},5, workspace, 5 \n \
    bind = {prik},6, workspace, 6 \n \
    bind = {prik},7, workspace, 7 \n \
    bind = {prik},8, workspace, 8 \n \
    bind = {prik},9, workspace, 9 \n \
    bind = {prik},0, workspace, 10 \n \
    bind = {prik}_{seck},1,movetoworkspacesilent,1 \n \
    bind = {prik}_{seck},2,movetoworkspacesilent,2 \n \
    bind = {prik}_{seck},3,movetoworkspacesilent,3 \n \
    bind = {prik}_{seck},4,movetoworkspacesilent,4 \n \
    bind = {prik}_{seck},5,movetoworkspacesilent,5 \n \
    bind = {prik}_{seck},6,movetoworkspacesilent,6 \n \
    bind = {prik}_{seck},7,movetoworkspacesilent,7 \n \
    bind = {prik}_{seck},8,movetoworkspacesilent,8 \n \
    bind = {prik}_{seck},9,movetoworkspacesilent,9 \n \
    bind = {prik}_{seck},0,movetoworkspacesilent,10 \n \
    general {sector_head}\n \
    gaps_in = {gaps}\n \
    gaps_out = {gaps}\n \
    border_size = {width}\n \
    {sector_tail}\n \
    decoration {sector_head}\n \
    rounding = {radius}\n \
    blur = {blur}\n \
    blur_size = 3\n \
    blur_passes = 3\n \
    blur_new_optimizations = true\n \
    drop_shadow = false\n \
    shadow_ignore_window = true\n \
    shadow_offset = 0\n \
    shadow_range = 0\n \
    shadow_render_power = 0\n \
    col.shadow = rgba(00000099)\n \
    {sector_tail}\n \
    animations {sector_head}\n \
    enabled = true\n \
    animation = windows,{win_anim}\n \
    animation = workspaces,{work_anim}\n \
    {sector_tail}\n \
    ");

    fs::write(path, data).expect("failed to write file");

    Command::new("hyprctl")
        .arg("reload")
        .spawn()
        .expect("oops, hyprctl failed, do you have sway installed?");
}
pub fn mkselfcfg(primary_key: Option<ShortcutKey>, secondary_key: Option<ShortcutKey>, exit_header: Option<BindKey>, exit_key: String, launch_header: Option<BindKey>, launch_key: String, kill_header: Option<BindKey>, kill_key: String, mini_header: Option<BindKey>, mini_key: String, scratch_header: Option<BindKey>, scratch_key: String, theme: OurTheme, border: Option<Border>, winanim: Option<WindowAnimation>, workanim: Option<WorkAnimation>, blur: bool) {
    let home = get_home();
    let path = format!("{home}/Oceania/cfg.toml");
    let backup_path = format!("{home}/Oceania");
    std::process::Command::new("mkdir").arg("-p").arg(backup_path).output().expect("uh oh");
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
    fs::write(path, toml).expect("failed to write cfg.toml");
}