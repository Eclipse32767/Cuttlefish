#![allow(dead_code)]
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use std::fs::read_to_string;
use std::env;
use std::fs;
use gettextrs::gettext as tr;

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
    pub border: Border,
    pub winanim: String,
    pub workanim: String,
    pub blur: String,
    pub widgetsleft: Vec<String>,
    pub widgetscenter: Vec<String>,
    pub widgetsright:  Vec<String>,
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
        write!(
            f,
            "{}",
            match self {
                ShortcutKey::Super => tr("Windows/Command Key"),
                ShortcutKey::Alt => tr("Alt Key"),
                ShortcutKey::Shift => tr("Shift Key"),
                ShortcutKey::Ctrl => tr("Control Key"),
            }
        )
    }
}
impl std::fmt::Display for BindKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BindKey::PrimaryKey => tr("Primary Key"),
                BindKey::SecondaryKey => tr("Secondary Key"),
                BindKey::BothKey => tr("Primary + Secondary")
            }
        )
    }
}
impl std::fmt::Display for WindowAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WindowAnimation::None => tr("No Animation"),
                WindowAnimation::Popin => tr("Pop-in"),
                WindowAnimation::Slide => tr("Slide In")
            }
        )
    }
}
impl std::fmt::Display for WorkAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkAnimation::None => tr("No Animation"),
                WorkAnimation::Fade => tr("Fade In"),
                WorkAnimation::Slide => tr("Slide In Horizontally"),
                WorkAnimation::SlideVert => tr("Slide In Vertically")
            }
        )
    }
}
impl std::fmt::Display for BarWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BarWidget::Audio => tr("Audio"),
                BarWidget::Backlight => tr("Backlight"),
                BarWidget::Battery => tr("Battery"),
                BarWidget::Bluetooth => tr("Bluetooth"),
                BarWidget::CPU => tr("CPU"),
                BarWidget::Clock => tr("Clock"),
                BarWidget::Disk => tr("Disk"),
                BarWidget::KeyboardState => tr("Keyboard State"),
                BarWidget::Network => tr("Network"),
                BarWidget::RAM => tr("RAM"),
                BarWidget::Taskbar => tr("Taskbar"),
                BarWidget::Temperature => tr("Temperature"),
                BarWidget::Tray => tr("System Tray"),
                BarWidget::User => tr("Current User"),
                BarWidget::Workspaces => tr("Workspaces")
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
    widgetsleft = ["CPU", "RAM", "Temperature", "Current User"]
    widgetscenter = ["Workspaces", "Clock"]
    widgetsright = ["Audio", "Backlight", "System Tray"]
    
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
pub fn decodewidget(x: &str, default: BarWidget) -> BarWidget {
    match x {
        "Audio" => BarWidget::Audio,
        "Backlight" => BarWidget::Backlight,
        "Battery" => BarWidget::Battery,
        "Bluetooth" => BarWidget::Bluetooth,
        "CPU" => BarWidget::CPU,
        "Clock" => BarWidget::Clock,
        "Disk" => BarWidget::Disk,
        "Keyboard State" => BarWidget::KeyboardState,
        "Network" => BarWidget::Network,
        "RAM" => BarWidget::RAM,
        "Taskbar" => BarWidget::Taskbar,
        "Temperature" => BarWidget::Temperature,
        "System Tray" => BarWidget::Tray,
        "Current User" => BarWidget::User,
        "Workspaces" => BarWidget::Workspaces,
        &_ => default,
    }
}
pub fn encodetheme(x: OurTheme) -> String {
    match x {
        OurTheme::Dark => "dark".to_string(),
        OurTheme::Light => "light".to_string(),
        OurTheme::Custom => "custom".to_string()
    }
}
pub fn encodepri(x: Option<ShortcutKey>) -> String {
    match x.unwrap() {
        ShortcutKey::Super => "super".to_string(),
        ShortcutKey::Alt => "alt".to_string(),
        ShortcutKey::Ctrl => "control".to_string(),
        ShortcutKey::Shift => "shift".to_string()
    }
}
pub fn encodeheader(x: Option<BindKey>) -> String {
    match x.unwrap() {
        BindKey::PrimaryKey => "pri".to_string(),
        BindKey::SecondaryKey => "sec".to_string(),
        BindKey::BothKey => "both".to_string()
    }
}
pub fn encodeworkanim(x: Option<WorkAnimation>) -> String {
    match x.unwrap() {
        WorkAnimation::None => "none".to_string(),
        WorkAnimation::Fade => "fade".to_string(),
        WorkAnimation::Slide => "slide".to_string(),
        WorkAnimation::SlideVert => "slidev".to_string(),
    }
}
pub fn encodewinanim(x: Option<WindowAnimation>) -> String {
    match x.unwrap() {
        WindowAnimation::None => "none".to_string(),
        WindowAnimation::Popin => "popin".to_string(),
        WindowAnimation::Slide => "slide".to_string()
    }
}
pub fn encodeblur(x: bool) -> String {
    if x {
        "y".to_string()
    } else {
        "n".to_string()
    }
}
pub fn encodewidget(x: BarWidget) -> String {
    match x {
        BarWidget::Audio => "Audio",
        BarWidget::Backlight => "Backlight",
        BarWidget::Battery => "Battery",
        BarWidget::Bluetooth => "Bluetooth",
        BarWidget::CPU => "CPU",
        BarWidget::Clock => "Clock",
        BarWidget::Disk => "Disk",
        BarWidget::KeyboardState => "Keyboard State",
        BarWidget::Network => "Network",
        BarWidget::RAM => "RAM",
        BarWidget::Taskbar => "Taskbar",
        BarWidget::Temperature => "Temperature",
        BarWidget::Tray => "System Tray",
        BarWidget::User => "Current User",
        BarWidget::Workspaces => "Workspaces"
    }.to_string()
}
pub fn rip_shortcut(opt: Option<ShortcutKey>) -> String {
    match opt.unwrap() {
        ShortcutKey::Alt => "ALT".to_string(),
        ShortcutKey::Ctrl => "CONTROL".to_string(),
        ShortcutKey::Shift => "SHIFT".to_string(),
        ShortcutKey::Super => "SUPER".to_string()
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
pub fn rip_win_anim(opt: Option<WindowAnimation>) -> String {
    match opt.unwrap() {
        WindowAnimation::None => "0,1,default".to_string(),
        WindowAnimation::Popin => "1,3,default,popin".to_string(),
        WindowAnimation::Slide => "1,3,default,slide".to_string()
    }
}
pub fn rip_work_anim(opt: Option<WorkAnimation>) -> String {
    match opt.unwrap() {
        WorkAnimation::None => "0,3,default".to_string(),
        WorkAnimation::Fade => "1,3,default,fade".to_string(),
        WorkAnimation::Slide => "1,3,default,slide".to_string(),
        WorkAnimation::SlideVert => "1,3,default,slidevert".to_string()
    }
}
