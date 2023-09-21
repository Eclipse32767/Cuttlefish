use toml::to_string;
use std::fs;
use std::process::Command;
use crate::lib_cfg::*;
use crate::lib_style::*;

use crate::Configurator;
impl Configurator {
    pub fn mk_config(&self) {
        //self-cfg
        {
        let home = get_home();
        let path = format!("{home}/Oceania/cfg.toml");
        let backup_path = format!("{home}/Oceania");
        let mut left_widgets = vec![];
        let mut center_widgets = vec![];
        let mut right_widgets = vec![];
        for i in 0..self.bar_left.len() {
            left_widgets.push(encode_widget(self.bar_left[i]))
        }
        for i in 0..self.bar_center.len() {
            center_widgets.push(encode_widget(self.bar_center[i]))
        }
        for i in 0..self.bar_right.len() {
            right_widgets.push(encode_widget(self.bar_right[i]))
        }
        Command::new("mkdir").arg("-p").arg(backup_path).output().expect("uh oh");
        let data = FileData {
            theme: encode_theme(self.theme.clone()).to_string(),
            primary: encode_pri(self.primary_key).to_string(),
            secondary: encode_pri(self.secondary_key).to_string(),
            wallpaper: self.wallpaper.clone(),
            exit_h: encode_header(self.exit_header).to_string(),
            exit_k: self.exit_key.clone(),
            launch_h: encode_header(self.launch_header).to_string(),
            launch_k: self.launch_key.clone(),
            kill_h: encode_header(self.kill_header).to_string(),
            kill_k: self.kill_key.clone(),
            mini_h: encode_header(self.minimize_header).to_string(),
            mini_k: self.minimize_key.clone(),
            scratch_h: encode_header(self.scratch_header).to_string(),
            scratch_k: self.scratch_key.clone(),
            border: self.border.clone(),
            win_anim: encode_win_anim(self.window_anim).to_string(),
            work_anim: encode_work_anim(self.work_anim).to_string(),
            blur: encode_blur(self.blur).to_string(),
            widgets_left: left_widgets,
            widgets_center: center_widgets,
            widgets_right: right_widgets
        };
        let toml = to_string(&data).expect("failed to generate toml");
        fs::write(path, toml).expect("failed to write cfg.toml");
        }
        //hyprland cfg
        {
            let home = get_home();
            let data;
            let pri_k = rip_shortcut(self.primary_key);
            let sec_k = rip_shortcut(self.secondary_key);
            let exit_h = rip_bind(self.exit_header, self.primary_key, self.secondary_key);
            let exit_k = &self.exit_key;
            let launch_h = rip_bind(self.launch_header, self.primary_key, self.secondary_key);
            let launch_k = &self.launch_key;
            let kill_h = rip_bind(self.kill_header, self.primary_key, self.secondary_key);
            let kill_k = &self.kill_key;
            let mini_h = rip_bind(self.minimize_header, self.primary_key, self.secondary_key);
            let mini_k = &self.minimize_key;
            let scratch_h = rip_bind(self.scratch_header, self.primary_key, self.secondary_key);
            let scratch_k = &self.scratch_key;
            let gaps = self.border.gaps;
            let width = self.border.width;
            let radius = self.border.radius;
            let win_anim = rip_win_anim(self.window_anim);
            let work_anim = rip_work_anim(self.work_anim);
            let blur = self.blur;
            let active_border = string_from_col(match self.theme {
                OurTheme::Light => &self.theme_set.light.application.primary,
                OurTheme::Dark => &self.theme_set.dark.application.primary,
                OurTheme::Custom => &self.theme_set.custom.application.primary
            });
            let sector_head = r#"{"#;
            let sector_tail = r#"}"#;
            let path = format!("{home}/hypr/hyprland.conf");
            data = format!("#AUTO-GENERATED CONFIG, DO NOT EDIT, CHANGES WILL BE OVERWRITTEN \n \
    exec-once=oceania-shell\n \
    exec-once={home}/hypr/autostart\n \
    bind={exit_h},{exit_k},exec,wlogout\n \
    bind={launch_h},{launch_k},exec,rofi -show drun\n \
    bind={kill_h},{kill_k},killactive\n \
    bind={mini_h},{mini_k},movetoworkspace,special\n \
    bind={scratch_h},{scratch_k},togglespecialworkspace\n \
    bind = {pri_k}, left, movefocus, l\n \
    bind = {pri_k}, right, movefocus, r\n \
    bind = {pri_k}, up, movefocus, u\n \
    bind = {pri_k}, down, movefocus, d\n \
    bind = {pri_k}_{sec_k}, left, movewindow, l\n \
    bind = {pri_k}_{sec_k}, right, movewindow, r\n \
    bind = {pri_k}_{sec_k}, up, movewindow, u\n \
    bind = {pri_k}_{sec_k}, down, movewindow, d\n \
    bind = {pri_k},1, workspace, 1 \n \
    bind = {pri_k},2, workspace, 2 \n \
    bind = {pri_k},3, workspace, 3 \n \
    bind = {pri_k},4, workspace, 4 \n \
    bind = {pri_k},5, workspace, 5 \n \
    bind = {pri_k},6, workspace, 6 \n \
    bind = {pri_k},7, workspace, 7 \n \
    bind = {pri_k},8, workspace, 8 \n \
    bind = {pri_k},9, workspace, 9 \n \
    bind = {pri_k},0, workspace, 10 \n \
    bind = {pri_k}_{sec_k},1,movetoworkspacesilent,1 \n \
    bind = {pri_k}_{sec_k},2,movetoworkspacesilent,2 \n \
    bind = {pri_k}_{sec_k},3,movetoworkspacesilent,3 \n \
    bind = {pri_k}_{sec_k},4,movetoworkspacesilent,4 \n \
    bind = {pri_k}_{sec_k},5,movetoworkspacesilent,5 \n \
    bind = {pri_k}_{sec_k},6,movetoworkspacesilent,6 \n \
    bind = {pri_k}_{sec_k},7,movetoworkspacesilent,7 \n \
    bind = {pri_k}_{sec_k},8,movetoworkspacesilent,8 \n \
    bind = {pri_k}_{sec_k},9,movetoworkspacesilent,9 \n \
    bind = {pri_k}_{sec_k},0,movetoworkspacesilent,10 \n \
    general {sector_head}\n \
    gaps_in = {gaps}\n \
    gaps_out = {gaps}\n \
    border_size = {width}\n \
    col.active_border = rgb({active_border})
    {sector_tail}\n \
    decoration {sector_head}\n \
    rounding = {radius}\n \
    blur {sector_head} \n \
    enabled={blur} \n \
    size=3 \n \
    passes=3 \n \
    new_optimizations=true \n \
    {sector_tail}
    {sector_tail}\n \
    animations {sector_head}\n \
    enabled = true\n \
    animation = windows,{win_anim}\n \
    animation = workspaces,{work_anim}\n \
    {sector_tail}\n \
    source={home}/hypr/usercfg.conf
    ");
            let user_cfg_path = format!("{home}/hypr/usercfg.conf");
            let autostart_path = format!("{home}/hypr/autostart");
            Command::new("touch")
                .arg(user_cfg_path)
                .arg(autostart_path.clone())
                .output().expect("uh oh");
            Command::new("chmod")
                .arg("a+x")
                .arg(autostart_path)
                .output().expect("uh oh");
            fs::write(path, data).expect("failed to write file");

            Command::new("hyprctl")
                .arg("reload")
                .spawn()
                .expect("oops, hyprctl failed, do you have Hyprland installed?");
        }
        //waybar cfg
        {
            let left_widgets = rip_widget_vec(self.bar_left.clone());
            let center_widgets = rip_widget_vec(self.bar_center.clone());
            let right_widgets = rip_widget_vec(self.bar_right.clone());
            println!("{left_widgets}\n{center_widgets}\n{right_widgets}")
        }
        //wallpaper set
        {
            let home = get_home();
            Command::new("ln")
                .arg("-sf")
                .arg(self.wallpaper.clone())
                .arg(format!("{home}/Oceania/wallpaper"))
                .output()
                .expect("oops!");
            Command::new("swww")
                .arg("img")
                .arg(format!("{home}/Oceania/wallpaper"))
                .spawn()
                .expect("oops someone forgot to install swww");
        }
    }
}
