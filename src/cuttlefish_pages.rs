use gettextrs::gettext as tr;
use iced::widget::{Column, Text, pick_list, Button, Row};

use crate::{Configurator, Message, lib_style::{ThemeCustom, TextStyle}, lib_cfg::{ShortcutKey, OurTheme, BindKey, BarWidget, WorkAnimation, WindowAnimation}, ShrinkValue, CaptureInput, WidgetBank, IncrVal};




impl Configurator {
    pub fn main_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        let sel_text = TextStyle {color: style.application.success};
        //let selection_marker: Text = Text::new("=>");
        let primary_pick = pick_list(
            &ShortcutKey::ALL[..], 
            self.primary_key, 
            Message::PrimaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let secondary_pick = pick_list(
            &ShortcutKey::ALL[..], 
            self.secondary_key, 
            Message::SecondaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let primary_txt;
        let temp_primary = format!("{}{}", tr("Primary Shortcut Key"), tr("-- Control and shift not recommended"));
        let secondary_txt;
        let temp_secondary = format!("{}{}", tr("Secondary Shortcut Key"), tr("-- used for more advanced shortcuts"));
        if self.width == ShrinkValue::Full {
            primary_txt = temp_primary;
            secondary_txt = temp_secondary;
        } else {
            primary_txt = tr("Primary Shortcut Key");
            secondary_txt = tr("Secondary Shortcut Key");
        }
        
        let mut primary_label: Text = Text::new(primary_txt);
        let mut secondary_label: Text = Text::new(secondary_txt);

        let light_txt = Text::new(tr("Light"));
        let dark_txt = Text::new(tr("Dark"));
        let custom_txt = Text::new(tr("Custom"));
        let mut light = Button::new(light_txt)
            .on_press(Message::ThemeChanged(OurTheme::Light));
        let mut dark = Button::new(dark_txt)
            .on_press(Message::ThemeChanged(OurTheme::Dark));
        let mut custom = Button::new(custom_txt)
            .on_press(Message::ThemeChanged(OurTheme::Custom));
        let mut theme_label = Text::new(tr("UI Theme for Configurator"));
        match self.theme {
            OurTheme::Light => {
                light = light.style(style.secondary.mk_theme());
            }
            OurTheme::Dark => {
                dark = dark.style(style.secondary.mk_theme());
            }
            OurTheme::Custom => {
                custom = custom.style(style.secondary.mk_theme());
            }
        }
        let mut wallpaper_label = Text::new(tr("The Wallpaper to use"));
        let wallpaper_select = Button::new(Text::new(self.wallpaper.clone())).on_press(Message::WallpaperPrompt);


        let mut theme_row = Row::new().spacing(10);
        let mut primary_row = Row::new().spacing(10);
        let mut secondary_row = Row::new().spacing(10);
        let mut wallpaper_row = Row::new().spacing(10);

        if self.index == 0 {
            theme_label = theme_label.style(sel_text.mk_theme())
        } else if self.index == 1 {
            primary_label = primary_label.style(sel_text.mk_theme())
        } else if self.index == 2 {
            secondary_label = secondary_label.style(sel_text.mk_theme());
        } else if self.index == 3 {
            wallpaper_label = wallpaper_label.style(sel_text.mk_theme());
        }
        theme_row = theme_row
            .push(theme_label)
            .push(light)
            .push(dark)
            .push(custom);
        primary_row = primary_row
            .push(primary_label)
            .push(primary_pick);
        secondary_row = secondary_row
            .push(secondary_label)
            .push(secondary_pick);
        wallpaper_row = wallpaper_row
            .push(wallpaper_label)
            .push(wallpaper_select);
        settings.push(theme_row).push(primary_row).push(secondary_row).push(wallpaper_row).spacing(10)
    }
    pub fn bind_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        //let selection marker: Text = Text::new("=>");
        let sel_text = TextStyle {color: style.application.success};
        let primary_pick = pick_list(
            &ShortcutKey::ALL[..], 
            self.primary_key, 
            Message::PrimaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let secondary_pick = pick_list(
            &ShortcutKey::ALL[..], 
            self.secondary_key, 
            Message::SecondaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let primary_txt;
        let temp_primary = format!("{}{}", tr("Primary Shortcut Key"), tr("-- Control and shift not recommended"));
        let secondary_txt;
        let temp_secondary = format!("{}{}", tr("Secondary Shortcut Key"), tr("-- used for more advanced shortcuts"));
        if self.width == ShrinkValue::Full {
            primary_txt = temp_primary;
            secondary_txt = temp_secondary;
        } else {
            primary_txt = tr("Primary Shortcut Key");
            secondary_txt = tr("Secondary Shortcut Key");
        }
        let mut primary_label: Text = Text::new(primary_txt);
        let mut secondary_label: Text = Text::new(secondary_txt);


        let mut exit_sc_label = Text::new(tr("Exit the Desktop Session"));
        let exit_header_select = pick_list(
        &BindKey::ALL[..], 
        self.exit_header, 
        Message::ExitHeaderChanged,
        )
        .placeholder("choose")
        .style(style.list.mk_theme());
        let exit_key = Text::new(self.exit_key.clone());
        let mut exit_key_select = Button::new(exit_key).on_press(Message::Capture(CaptureInput::ExitKey)).width(50);
        let mut launch_sc_label: Text = Text::new(tr("Open the App Launcher"));
        let launch_header_select = pick_list(
            &BindKey::ALL[..], 
            self.launch_header, 
            Message::LaunchHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let launch_key = Text::new(self.launch_key.clone());
        let mut launch_key_select = Button::new(launch_key).on_press(Message::Capture(CaptureInput::LaunchKey)).width(50);
        let mut kill_sc_label: Text = Text::new(tr("Close the Currently Focused App"));
        let kill_header_select = pick_list(
            &BindKey::ALL[..], 
            self.kill_header, 
            Message::KillHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let kill_key = Text::new(self.kill_key.clone());
        let mut kill_key_select = Button::new(kill_key).on_press(Message::Capture(CaptureInput::KillKey)).width(50);
        let mut mini_sc_label: Text = Text::new(tr("Minimize the Focused App"));
        let mini_header_select = pick_list(
         &BindKey::ALL[..], 
         self.minimize_header, 
         Message::MiniHeaderChanged,
         )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let mini_key = Text::new(self.minimize_key.clone());
        let mut mini_key_select = Button::new(mini_key).on_press(Message::Capture(CaptureInput::MiniKey)).width(50);
        let mut scratch_sc_label: Text = Text::new(tr("Retrieve App from Minimization"));
        let scratch_header_select = pick_list(
            &BindKey::ALL[..], 
            self.scratch_header, 
            Message::ScratchHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let scratch_key = Text::new(self.scratch_key.clone());
        let mut scratch_key_select = Button::new(scratch_key).on_press(Message::Capture(CaptureInput::ScratchKey)).width(50);
        
        match self.capture_next.as_ref().unwrap() {
            CaptureInput::NoKey => {
            }
            CaptureInput::ExitKey => {
                exit_key_select = exit_key_select.style(style.secondary.mk_theme());
            }
            CaptureInput::KillKey => {
                kill_key_select = kill_key_select.style(style.secondary.mk_theme());
            }
            CaptureInput::LaunchKey => {
                launch_key_select = launch_key_select.style(style.secondary.mk_theme());
            }
            CaptureInput::MiniKey => {
                mini_key_select = mini_key_select.style(style.secondary.mk_theme());
            }
            CaptureInput::ScratchKey => {
                scratch_key_select = scratch_key_select.style(style.secondary.mk_theme());
            }
        }
        let mut primary_row = Row::new();
        let mut secondary_row = Row::new();
        let mut exit_sc_row = Row::new();
        let mut launch_sc_row = Row::new();
        let mut kill_sc_row = Row::new();
        let mut mini_sc_row = Row::new();
        let mut scratch_sc_row = Row::new();
        if self.index == 0 {
            primary_label = primary_label.style(sel_text.mk_theme());
        } else if self.index == 1 {
            secondary_label = secondary_label.style(sel_text.mk_theme());
        } else if self.index == 2 {
            exit_sc_label = exit_sc_label.style(sel_text.mk_theme());
        } else if self.index == 3 {
            launch_sc_label = launch_sc_label.style(sel_text.mk_theme());
        } else if self.index == 4 {
            kill_sc_label = kill_sc_label.style(sel_text.mk_theme());
        } else if self.index == 5 {
            mini_sc_label = mini_sc_label.style(sel_text.mk_theme());
        } else if self.index == 6 {
            scratch_sc_label = scratch_sc_label.style(sel_text.mk_theme());
        }
        primary_row = primary_row
            .push(primary_label)
            .push(primary_pick)
            .spacing(10);
        secondary_row = secondary_row
            .push(secondary_label)
            .push(secondary_pick)
            .spacing(10);
        exit_sc_row = exit_sc_row
            .push(exit_sc_label)
            .push(exit_header_select)
            .push(exit_key_select)
            .spacing(10);
        launch_sc_row = launch_sc_row
            .push(launch_sc_label)
            .push(launch_header_select)
            .push(launch_key_select)
            .spacing(10);
        kill_sc_row = kill_sc_row
            .push(kill_sc_label)
            .push(kill_header_select)
            .push(kill_key_select)
            .spacing(10);
        mini_sc_row = mini_sc_row
            .push(mini_sc_label)
            .push(mini_header_select)
            .push(mini_key_select)
            .spacing(10);
        scratch_sc_row = scratch_sc_row
            .push(scratch_sc_label)
            .push(scratch_header_select)
            .push(scratch_key_select)
            .spacing(10);
        settings
            .push(primary_row)
            .push(secondary_row)
            .push(exit_sc_row)
            .push(launch_sc_row)
            .push(kill_sc_row)
            .push(mini_sc_row)
            .push(scratch_sc_row).spacing(10)
    }
    pub fn bar_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        let selection_marker: Text = Text::new("=>");
        let mut left_contents = String::from("");
        if self.bar_left.len() > 0 {
            for i in 0..self.bar_left.len() {
                  left_contents = format!("{left_contents}  {:#?}", self.bar_left[i]);
            }
        }
        let mut right_contents = String::from("");
        if self.bar_right.len() > 0 {
            for i in 0..self.bar_right.len() {
                right_contents = format!("{right_contents}  {:#?}", self.bar_right[i]);
            }
        }
        let mut center_contents = String::from("");
        if self.bar_center.len() > 0 {
            for i in 0..self.bar_center.len() {
                center_contents = format!("{center_contents}  {:#?}", self.bar_center[i]);
            }
        }
        let bar_left = Button::new(Text::new(tr("Left"))).on_press(Message::PushWidget(WidgetBank::Left));
        let bar_center = Button::new(Text::new(tr("Center"))).on_press(Message::PushWidget(WidgetBank::Center));
        let bar_right = Button::new(Text::new(tr("Right"))).on_press(Message::PushWidget(WidgetBank::Right));
        let mut audio = Button::new(Text::new(tr("Audio"))).on_press(Message::AwaitDestination(BarWidget::Audio));
        let mut backlight = Button::new(Text::new(tr("Backlight"))).on_press(Message::AwaitDestination(BarWidget::Backlight));
        let mut battery = Button::new(Text::new(tr("Battery"))).on_press(Message::AwaitDestination(BarWidget::Battery));
        let mut bluetooth = Button::new(Text::new(tr("Bluetooth"))).on_press(Message::AwaitDestination(BarWidget::Bluetooth));
        let mut cpu = Button::new(Text::new(tr("CPU"))).on_press(Message::AwaitDestination(BarWidget::CPU));
        let mut clock = Button::new(Text::new(tr("Clock"))).on_press(Message::AwaitDestination(BarWidget::Clock));
        let mut disk = Button::new(Text::new(tr("Disk"))).on_press(Message::AwaitDestination(BarWidget::Disk));
        let mut keyboard = Button::new(Text::new(tr("Keyboard State"))).on_press(Message::AwaitDestination(BarWidget::KeyboardState));
        let mut network = Button::new(Text::new(tr("Network"))).on_press(Message::AwaitDestination(BarWidget::Network));
        let mut ram = Button::new(Text::new(tr("RAM"))).on_press(Message::AwaitDestination(BarWidget::RAM));
        let mut taskbar = Button::new(Text::new(tr("Taskbar"))).on_press(Message::AwaitDestination(BarWidget::Taskbar));
        let mut temperature = Button::new(Text::new(tr("Temperature"))).on_press(Message::AwaitDestination(BarWidget::Temperature));
        let mut tray = Button::new(Text::new(tr("System Tray"))).on_press(Message::AwaitDestination(BarWidget::Tray));
        let mut user = Button::new(Text::new(tr("Current User"))).on_press(Message::AwaitDestination(BarWidget::User));
        let mut workspaces = Button::new(Text::new(tr("Workspaces"))).on_press(Message::AwaitDestination(BarWidget::Workspaces));
        let remove_left = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Left));
        let remove_center = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Center));
        let remove_right = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Right));
        let label_left = Text::new(left_contents);
        let label_right = Text::new(right_contents);
        let label_center = Text::new(center_contents);

        match self.next_widget {
            Some(value) => {
                match value {
                    BarWidget::Audio => audio = audio.style(style.secondary.mk_theme()),
                    BarWidget::Backlight => backlight = backlight.style(style.secondary.mk_theme()),
                    BarWidget::Battery => battery = battery.style(style.secondary.mk_theme()),
                    BarWidget::Bluetooth => bluetooth = bluetooth.style(style.secondary.mk_theme()),
                    BarWidget::Clock => clock = clock.style(style.secondary.mk_theme()),
                    BarWidget::CPU => cpu = cpu.style(style.secondary.mk_theme()),
                    BarWidget::Disk => disk = disk.style(style.secondary.mk_theme()),
                    BarWidget::KeyboardState => keyboard = keyboard.style(style.secondary.mk_theme()),
                    BarWidget::RAM => ram = ram.style(style.secondary.mk_theme()),
                    BarWidget::Network => network = network.style(style.secondary.mk_theme()),
                    BarWidget::Temperature => temperature = temperature.style(style.secondary.mk_theme()),
                    BarWidget::Tray => tray = tray.style(style.secondary.mk_theme()),
                    BarWidget::Taskbar => taskbar = taskbar.style(style.secondary.mk_theme()),
                    BarWidget::Workspaces => workspaces = workspaces.style(style.secondary.mk_theme()),
                    BarWidget::User => user = user.style(style.secondary.mk_theme()),
                }
            },
            None => {}
        }
        
        let mut left_row = Row::new();
        let mut center_row = Row::new();
        let mut right_row = Row::new();
        let mut widget_row_i = Row::new();
        let mut widget_row_ii = Row::new();
        let mut widget_row_iii = Row::new();
        let mut widget_row_iv = Row::new();
        let mut widget_row_v = Row::new();

        if self.index == 0 {
            widget_row_i = widget_row_i.push(selection_marker)
        } else if self.index == 1 {
            widget_row_ii = widget_row_ii.push(selection_marker)
        } else if self.index == 2 {
            widget_row_iii = widget_row_iii.push(selection_marker)
        } else if self.index == 3 {
            widget_row_iv = widget_row_iv.push(selection_marker)
        } else if self.index == 4 {
            widget_row_v = widget_row_v.push(selection_marker)
        } else if self.index == 5 {
            left_row = left_row.push(selection_marker)
        } else if self.index == 6 {
            center_row = center_row.push(selection_marker)
        } else if self.index == 7 {
            right_row = right_row.push(selection_marker)
        }
        
        left_row = left_row.push(bar_left).push(label_left).push(remove_left).spacing(10);
        center_row = center_row.push(bar_center).push(label_center).push(remove_center).spacing(10);
        right_row = right_row.push(bar_right).push(label_right).push(remove_right).spacing(10);
        widget_row_i = widget_row_i.push(audio).push(backlight).push(battery).spacing(10);
        widget_row_ii = widget_row_ii.push(bluetooth).push(cpu).push(clock).spacing(10);
        widget_row_iii = widget_row_iii.push(disk).push(keyboard).push(network).spacing(10);
        widget_row_iv = widget_row_iv.push(ram).push(taskbar).push(temperature).spacing(10);
        widget_row_v = widget_row_v.push(tray).push(user).push(workspaces).spacing(10);

        settings
            .push(widget_row_i)
            .push(widget_row_ii)
            .push(widget_row_iii)
            .push(widget_row_iv)
            .push(widget_row_v)
            .push(left_row)
            .push(center_row)
            .push(right_row).spacing(10)
    }
    pub fn anim_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        //let selection_marker: Text = Text::new("=>");
        let sel_text = TextStyle {color: style.application.success};
        let width_incr = Button::new("+").on_press(Message::Incr(IncrVal::WidthVal)).width(30);
        let mut width_decr = Button::new("-").on_press(Message::Decr(IncrVal::WidthVal)).width(30);
        let width_value_peek = Text::new(format!("{}", self.border.width));
        let mut width_label = Text::new(tr("The Width of The Window Borders:"));

        let mut width_row = Row::new().spacing(10);

        let gaps_incr = Button::new("+").on_press(Message::Incr(IncrVal::GapsVal)).width(30);
        let mut gaps_decr = Button::new("-").on_press(Message::Decr(IncrVal::GapsVal)).width(30);
        let gaps_value_peek = Text::new(format!("{}", self.border.gaps));
        let mut gaps_label = Text::new(tr("The Size of The Standard Window Gaps:"));

        let mut gaps_row = Row::new().spacing(10);

        let rad_incr = Button::new("+").on_press(Message::Incr(IncrVal::RadiusVal)).width(30);
        let mut rad_decr = Button::new("-").on_press(Message::Decr(IncrVal::RadiusVal)).width(30);
        let rad_value_peek = Text::new(format!("{}", self.border.radius));
        let mut rad_label = Text::new(tr("The roundedness of window corners:"));

        let mut rad_row = Row::new().spacing(10);

        let win_pick = pick_list(
            &WindowAnimation::ALL[..], 
            self.window_anim, 
            Message::ChangeWindowAnim,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let mut win_label = Text::new(tr("The Window Animations To Be Used:"));

        let mut win_row = Row::new().spacing(10);

        let work_pick = pick_list(
            &WorkAnimation::ALL[..],
            self.work_anim,
            Message::ChangeWorkAnim,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let mut work_label = Text::new(tr("The Workspace Animations To Be Used:"));

        let mut work_row = Row::new().spacing(10);

        let enable = Text::new(tr("Enable"));
        let disable = Text::new(tr("Disable"));
        let enabled = Text::new(tr("Enabled"));
        let disabled = Text::new(tr("Disabled"));
        let mut blur_label = Text::new(tr("Whether or not to use window blur"));
        let mut blur_on = Button::new(enable).on_press(Message::BlurToggled(true));
        let mut blur_off = Button::new(disable).on_press(Message::BlurToggled(false));
        if self.blur {
            blur_on = Button::new(enabled).on_press(Message::BlurToggled(true)).style(style.secondary.mk_theme());
        } else {
            blur_off = Button::new(disabled).on_press(Message::BlurToggled(false)).style(style.secondary.mk_theme());
        }
        let mut blur_row = Row::new().spacing(10);

        if self.border.width == 0 {
            width_decr = width_decr.style(style.secondary.mk_theme());
        }
        if self.border.gaps == 0 {
            gaps_decr = gaps_decr.style(style.secondary.mk_theme());
        }
        if self.border.radius == 0 {
            rad_decr = rad_decr.style(style.secondary.mk_theme());
        }

        if self.index == 0 {
            width_label = width_label.style(sel_text.mk_theme());
        } else if self.index == 1 {
            gaps_label = gaps_label.style(sel_text.mk_theme());
        } else if self.index == 2 {
            rad_label = rad_label.style(sel_text.mk_theme());
        } else if self.index == 3 {
            win_label = win_label.style(sel_text.mk_theme());
        } else if self.index == 4 {
            work_label = work_label.style(sel_text.mk_theme());
        } else if self.index == 5 {
            blur_label = blur_label.style(sel_text.mk_theme());
        }

        width_row = width_row
            .push(width_label)
            .push(width_decr)
            .push(width_value_peek)
            .push(width_incr);
        gaps_row = gaps_row
            .push(gaps_label)
            .push(gaps_decr)
            .push(gaps_value_peek)
            .push(gaps_incr);
        rad_row = rad_row
            .push(rad_label)
            .push(rad_decr)
            .push(rad_value_peek)
            .push(rad_incr);
        win_row = win_row
            .push(win_label)
            .push(win_pick);
        work_row = work_row
            .push(work_label)
            .push(work_pick);
        blur_row = blur_row
            .push(blur_label)
            .push(blur_off)
            .push(blur_on);
        settings
            .push(width_row)
            .push(gaps_row)
            .push(rad_row)
            .push(win_row)
            .push(work_row)
            .push(blur_row).spacing(10)
    }
}