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
    let language = langstr("man");
    let decoded: Translation = from_str(&language).unwrap();
    decoded
}