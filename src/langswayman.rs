use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use liblocale::langstr;
use crate::liblocale;


#[derive(Deserialize, Debug, Serialize)]
pub struct Translation {
    pub(crate) globals: Option<GlobalStrings>,
    pub(crate) navigation: Option<NavStrings>,
    pub(crate) advanced: Option<AdvStrings>,
    pub(crate) workspaces: Option<WorkStrings>,
    pub(crate) minimization: Option<MiniStrings>
}
#[derive(Deserialize, Debug, Serialize)]
pub struct GlobalStrings {
    pub(crate) title: String,
    pub(crate) backtxt: String,
    pub(crate) forwardtxt: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct NavStrings {
    pub(crate) title: String,
    pub(crate) prefocus: String,
    pub(crate) focus: String,
    pub(crate) postfocus: String,
    pub(crate) premove: String,
    pub(crate) movetxt: String,
    pub(crate) postmove: String,
    pub(crate) immutable: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct AdvStrings {
    pub(crate) title: String,
    pub(crate) presearch: String,
    pub(crate) search: String,
    pub(crate) postsearch: String,
    pub(crate) prekill: String,
    pub(crate) kill: String,
    pub(crate) postkill: String,
    pub(crate) preexit: String,
    pub(crate) exit: String,
    pub(crate) postexit: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct WorkStrings {
    pub(crate) title: String,
    pub(crate) head: String,
    pub(crate) prefocus: String,
    pub(crate) focus: String,
    pub(crate) postfocus: String,
    pub(crate) premove: String,
    pub(crate) movetxt: String,
    pub(crate) postmove: String,
    pub(crate) immutable: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct MiniStrings {
    pub(crate) title: String,
    pub(crate) premove: String,
    pub(crate) movetxt: String,
    pub(crate) postmove: String,
    pub(crate) prefocus: String,
    pub(crate) focus: String,
    pub(crate) postfocus: String
}

pub fn get_lang() -> Translation {
    let placeholder = r#"[globals]
    title = "Sunfish Manual"
    backtxt = "Back"
    forwardtxt = "Forwards"
    
    [navigation]
    title = "Basic Navigation"
    prefocus = "To shift focus between applications, press:\n"
    focus = "+An Arrow Key.\n"
    postfocus = "This will shift the interface's focus in the direction you pressed.\n \n"
    premove = "To move applications around, press:\n"
    movetxt = "+An Arrow Key.\n"
    postmove = "This should swap applications in that direction.\n\n"
    immutable = "As of now, these bindings are inferred and cannot be directly changed."
    
    [advanced]
    title = "Basic Navigation, Continued"
    presearch = "To open the application search, press:\n" 
    search = ".\n"
    postsearch = "This will open a search menu that you can use to run the apps you want.\n\n"
    prekill = "To close the currently focused application, press:\n"
    kill = ".\n"
    postkill = "This will close the currently focused application, potentially destroying unsaved work. \n\n"
    preexit = "To return to the login screen, press:\n"
    exit = ".\n"
    postexit = "This will close out the desktop entirely, destroying all unsaved work."
    
    [workspaces]
    title = "Workspaces"
    head = "There are 10 workspaces in this environment-\n In effect each one is its own desktop where you can move applications to or visit the applications located there.\n\n"
    prefocus = "To move yourself to a workspace, press:\n"
    focus = "+A Number Key.\n"
    postfocus = "This will move you to the workspace corresponding to the number you pressed.\n \n"
    premove = "To move the currently focused application to a workspace, press:\n"
    movetxt = "+A Number Key.\n"
    postmove = "This will banish the application to the corresponding workspace.\n\n"
    immutable = "As of now, these bindings are inferred and cannot be manually changed."
    
    [minimization]
    title = "Minimization"
    premove = "To minimize the focused application, press:\n"
    movetxt = ".\n"
    postmove = "This will minimize said application, temporarily removing it from the current workspace.\n\n"
    prefocus = "To cycle through currently minimized apps, press:\n"
    focus = ".\n"
    postfocus = "This will cycle through them, making one visible while hiding the others.""#;
    let language = langstr("man", placeholder);
    let decoded: Translation = from_str(&language).unwrap();
    decoded
}