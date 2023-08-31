use toml::to_string;
use std::fs;
use std::process::Command;
use crate::libcfg::*;


use crate::Configurator;
impl Configurator {
    pub fn mkconfig(&self) {
        //selfcfg
        let home = get_home();
        let path = format!("{home}/Oceania/cfg.toml");
        let backup_path = format!("{home}/Oceania");
        let mut leftwidgets = vec![];
        let mut centerwidgets = vec![];
        let mut rightwidgets = vec![];
        for i in 0..self.bar_left.len() {
            leftwidgets.push(encodewidget(self.bar_left[i]))
        }
        for i in 0..self.bar_center.len() {
            centerwidgets.push(encodewidget(self.bar_center[i]))
        }
        for i in 0..self.bar_right.len() {
            rightwidgets.push(encodewidget(self.bar_right[i]))
        }
        std::process::Command::new("mkdir").arg("-p").arg(backup_path).output().expect("uh oh");
        let data = FileData {
            theme: encodetheme(self.theme.clone()).to_string(),
            primary: encodepri(self.primary_key).to_string(),
            secondary: encodepri(self.secondary_key).to_string(),
            exith: encodeheader(self.exit_header).to_string(),
            exitk: self.exit_key.clone(),
            launchh: encodeheader(self.launch_header).to_string(),
            launchk: self.launch_key.clone(),
            killh: encodeheader(self.kill_header).to_string(),
            killk: self.kill_key.clone(),
            minih: encodeheader(self.minimize_header).to_string(),
            minik: self.minimize_key.clone(),
            scratchh: encodeheader(self.scratch_header).to_string(),
            scratchk: self.scratch_key.clone(),
            border: self.border.clone(),
            winanim: encodewinanim(self.window_anim).to_string(),
            workanim: encodeworkanim(self.work_anim).to_string(),
            blur: encodeblur(self.blur).to_string(),
            widgetsleft: leftwidgets,
            widgetscenter: centerwidgets,
            widgetsright: rightwidgets
        };
        let toml = to_string(&data).expect("failed to generate toml");
        fs::write(path, toml).expect("failed to write cfg.toml");
        let home = get_home();
        let data;
        let prik = rip_shortcut(self.primary_key);
        let seck = rip_shortcut(self.secondary_key);
        let exith = rip_bind(self.exit_header, self.primary_key, self.secondary_key);
        let exitk = &self.exit_key;
        let launchh = rip_bind(self.launch_header, self.primary_key, self.secondary_key);
        let launchk = &self.launch_key;
        let killh = rip_bind(self.kill_header, self.primary_key, self.secondary_key);
        let killk = &self.kill_key;
        let minih = rip_bind(self.minimize_header, self.primary_key, self.secondary_key);
        let minik = &self.minimize_key;
        let scratchh = rip_bind(self.scratch_header, self.primary_key, self.secondary_key);
        let scratchk = &self.scratch_key;
        let gaps = self.border.gaps;
        let width = self.border.width;
        let radius = self.border.radius;
        let win_anim = rip_win_anim(self.window_anim);
        let work_anim = rip_work_anim(self.work_anim);
        let blur = self.blur;
        let sector_head = r#"{"#;
        let sector_tail = r#"}"#;
        let path = format!("{home}/hypr/hyprland.conf");
    data = format!("#AUTO-GENERATED CONFIG, DO NOT EDIT, CHANGES WILL BE OVERWRITTEN \n \
    source={home}/hypr/usercfg.conf\n \
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
    blur {sector_head} \n \
    enabled={blur} \n \
    size=3 \n \
    passes=3 \n \
    new_optimizations=true \n \
    {sector_tail}
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
            .expect("oops, hyprctl failed, do you have Hyprland installed?");
    }
}