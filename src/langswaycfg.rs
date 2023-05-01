use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use liblocale::langstr;
use crate::liblocale;

#[derive(Deserialize, Debug, Serialize)]
pub struct Translation {
    pub(crate) global: Option<PageGlobals>,
    pub(crate) mainpage: Option<MainPage>,
    pub(crate) bindpage: Option<BindPage>,
    pub(crate) autopage: Option<AutoPage>,
    pub(crate) barpage: Option<BarPage>,
    pub(crate) prettyprint: Option<PrettyPrint>,
    pub(crate) animpage: Option<AnimPage>,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct MainPage {
    pub(crate) borders: String,
    pub(crate) width: String,
    pub(crate) theme: String,
    pub(crate) light: String,
    pub(crate) dark: String,
    pub(crate) custom: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct BindPage {
    pub(crate) exit: String,
    pub(crate) keyplaceholder: String,
    pub(crate) launch: String,
    pub(crate) kill: String,
    pub(crate) mini: String,
    pub(crate) scratch: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct AutoPage {

}
#[derive(Deserialize, Debug, Serialize)]
pub struct BarPage {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnimPage {
    pub(crate) width: String,
    pub(crate) radius: String,
    pub(crate) gaps: String,
    pub(crate) winanim: String,
    pub(crate) workanim: String,
    pub(crate) blur: String,
    pub(crate) enableblur: String,
    pub(crate) enabledblur: String,
    pub(crate) disableblur: String,
    pub(crate) disabledblur: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct PrettyPrint {
    pub(crate) borderno: String,
    pub(crate) bordernormal: String,
    pub(crate) bordercsd: String,
    pub(crate) borderpixel: String,
    pub(crate) keysuper: String,
    pub(crate) keyalt: String,
    pub(crate) keyshift: String,
    pub(crate) keyctrl: String,
    pub(crate) bindpri: String,
    pub(crate) bindsec: String,
    pub(crate) bindboth: String,
    pub(crate) pagemain: String,
    pub(crate) pagebind: String,
    pub(crate) pagebar: String,
    pub(crate) pageinit: String,
    pub(crate) pageanim: String,
    pub(crate) winnone: String,
    pub(crate) winpop: String,
    pub(crate) winslide: String,
    pub(crate) worknone: String,
    pub(crate) workfade: String,
    pub(crate) workslide: String,
    pub(crate) workslidev: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct PageGlobals {
    pub(crate) title: String,
    pub(crate) label: String,
    pub(crate) main: String,
    pub(crate) bind: String,
    pub(crate) bar: String,
    pub(crate) init: String,
    pub(crate) anim: String,
    pub(crate) save: String,
    pub(crate) saved: String,
    pub(crate) primary: String,
    pub(crate) secondary: String,
}

pub fn get_lang() -> Translation {
    let placeholder = r#"[global]
    title = "Cuttlefish Configurator -- "
    label = "Available Pages"
    main = "Main Page"
    bind = "Bindings Page"
    bar = "Bar Page"
    init = "Autostart Page"
    anim = "Animations Page"
    save = "Save"
    saved = "Saved!"
    primary = "Primary Shortcut Key-- Control and shift not recommended"
    secondary = "Secondary Shortcut Key-- used for more advanced shortcuts"
    
    [mainpage]
    borders = "Window Borders"
    width = "Width: "
    theme = "UI Theme for Configurator"
    light = "Light"
    dark = "Dark"
    custom = "Custom"
    
    [bindpage]
    exit = "Exit the Desktop Session"
    keyplaceholder = "Key"
    launch = "Open the App Launcher"
    kill = "Close the Currently Focused App"
    mini = "Minimize the Focused App"
    scratch = "Retrieve App from Minimization"
    
    [barpage]
    
    [autopage]
    
    [animpage]
    width = "The Width of The Window Borders:"
    radius = "The roundedness of window corners:"
    gaps = "The Size of The Standard Window Gaps:"
    winanim = "The Window Animations To Be Used:"
    workanim = "The Animation to be used for Workspaces"
    blur = "Whether or not to use window blur"
    enableblur = "Enable"
    enabledblur = "Enabled" 
    disableblur = "Disable"
    disabledblur = "Disabled"
    
    
    [prettyprint]
    borderno = "No Border"
    bordernormal = "Outline With Title Bars"
    bordercsd = "Client-Side Decoration"
    borderpixel = "Just an Outline, no Title Bars"
    keysuper = "Windows/Command Key"
    keyalt = "Alt Key"
    keyshift = "Shift Key"
    keyctrl = "Control Key"
    bindpri = "Primary Key"
    bindsec = "Secondary Key"
    bindboth = "Primary + Secondary"
    pagemain = "Main Page"
    pagebind = "Keybindings Page"
    pagebar = "Status Bar Page"
    pageinit = "Autostart Page"
    pageanim = "Animations Page"
    winnone = "No Animation"
    winpop = "Pop-in"
    winslide = "Slide in"
    worknone = "No Animation"
    workfade = "Fade In"
    workslide = "Slide in Horizontally"
    workslidev = "Slide in Vertically""#;
    let language = langstr("cfg", placeholder);
    let decoded: Translation = from_str(&language).unwrap();
    decoded
}