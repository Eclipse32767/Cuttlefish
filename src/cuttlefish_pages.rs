use gettextrs::gettext as tr;
use iced::widget::{Column, Text, pick_list, Button, Row};

use crate::{Configurator, Message, libstyle::{ThemeCustom, TextStyle}, libcfg::{ShortcutKey, OurTheme, BindKey, BarWidget, WorkAnimation, WindowAnimation}, ShrinkValue, CaptureInput, WidgetBank, IncrVal};




impl Configurator {
    pub fn main_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        let seltext = TextStyle {color: style.application.success};
        //let selectionmarker: Text = Text::new("=>");
        let primarypick = pick_list(
            &ShortcutKey::ALL[..], 
            self.primary_key, 
            Message::PrimaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let secondarypick = pick_list(
            &ShortcutKey::ALL[..], 
            self.secondary_key, 
            Message::SecondaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let primarytxt;
        let temp_primary = format!("{}{}", tr("Primary Shortcut Key"), tr("-- Control and shift not recommended"));
        let secondarytxt;
        let temp_secondary = format!("{}{}", tr("Secondary Shortcut Key"), tr("-- used for more advanced shortcuts"));
        if self.width == ShrinkValue::Full {
            primarytxt = temp_primary;
            secondarytxt = temp_secondary;
        } else {
            primarytxt = tr("Primary Shortcut Key");
            secondarytxt = tr("Secondary Shortcut Key");
        }
        
        let mut primarylabel: Text = Text::new(primarytxt);
        let mut secondarylabel: Text = Text::new(secondarytxt);

        let lighttxt = Text::new(tr("Light"));
        let darktxt = Text::new(tr("Dark"));
        let customtxt = Text::new(tr("Custom"));
        let mut light = Button::new(lighttxt)
            .on_press(Message::ThemeChanged(OurTheme::Light));
        let mut dark = Button::new(darktxt)
            .on_press(Message::ThemeChanged(OurTheme::Dark));
        let mut custom = Button::new(customtxt)
            .on_press(Message::ThemeChanged(OurTheme::Custom));
        let mut themelabel = Text::new(tr("UI Theme for Configurator"));
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
        let mut themerow = Row::new().spacing(10);
        let mut primaryrow = Row::new().spacing(10);
        let mut secondaryrow = Row::new().spacing(10);

        if self.index == 0 {
            themelabel = themelabel.style(seltext.mk_theme())
        } else if self.index == 1 {
            primarylabel = primarylabel.style(seltext.mk_theme())
        } else if self.index == 2 {
            secondarylabel = secondarylabel.style(seltext.mk_theme());
        }
        themerow = themerow
            .push(themelabel)
            .push(light)
            .push(dark)
            .push(custom);
        primaryrow = primaryrow
            .push(primarylabel)
            .push(primarypick);
        secondaryrow = secondaryrow
            .push(secondarylabel)
            .push(secondarypick);
        settings.push(themerow).push(primaryrow).push(secondaryrow).spacing(10)
    }
    pub fn bind_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        //let selectionmarker: Text = Text::new("=>");
        let seltext = TextStyle {color: style.application.success};
        let primarypick = pick_list(
            &ShortcutKey::ALL[..], 
            self.primary_key, 
            Message::PrimaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let secondarypick = pick_list(
            &ShortcutKey::ALL[..], 
            self.secondary_key, 
            Message::SecondaryKeyChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let primarytxt;
        let temp_primary = format!("{}{}", tr("Primary Shortcut Key"), tr("-- Control and shift not recommended"));
        let secondarytxt;
        let temp_secondary = format!("{}{}", tr("Secondary Shortcut Key"), tr("-- used for more advanced shortcuts"));
        if self.width == ShrinkValue::Full {
            primarytxt = temp_primary;
            secondarytxt = temp_secondary;
        } else {
            primarytxt = tr("Primary Shortcut Key");
            secondarytxt = tr("Secondary Shortcut Key");
        }
        let mut primarylabel: Text = Text::new(primarytxt);
        let mut secondarylabel: Text = Text::new(secondarytxt);


        let mut exitsclabel = Text::new(tr("Exit the Desktop Session"));
        let exitheaderselect = pick_list(
        &BindKey::ALL[..], 
        self.exit_header, 
        Message::ExitHeaderChanged,
        )
        .placeholder("choose")
        .style(style.list.mk_theme());
        let exitkey = Text::new(self.exit_key.clone());
        let mut exitkeyselect = Button::new(exitkey).on_press(Message::Capture(CaptureInput::ExitKey)).width(50);
        let mut launchsclabel: Text = Text::new(tr("Open the App Launcher"));
        let launchheaderselect = pick_list(
            &BindKey::ALL[..], 
            self.launch_header, 
            Message::LaunchHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let launchkey = Text::new(self.launch_key.clone());
        let mut launchkeyselect = Button::new(launchkey).on_press(Message::Capture(CaptureInput::LaunchKey)).width(50);
        let mut killsclabel: Text = Text::new(tr("Close the Currently Focused App"));
        let killheaderselect = pick_list(
            &BindKey::ALL[..], 
            self.kill_header, 
            Message::KillHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let killkey = Text::new(self.kill_key.clone());
        let mut killkeyselect = Button::new(killkey).on_press(Message::Capture(CaptureInput::KillKey)).width(50);
        let mut minisclabel: Text = Text::new(tr("Minimize the Focused App"));
        let miniheaderselect = pick_list(
         &BindKey::ALL[..], 
         self.minimize_header, 
         Message::MiniHeaderChanged,
         )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let minikey = Text::new(self.minimize_key.clone());
        let mut minikeyselect = Button::new(minikey).on_press(Message::Capture(CaptureInput::MiniKey)).width(50);
        let mut scratchsclabel: Text = Text::new(tr("Retrieve App from Minimization"));
        let scratchheaderselect = pick_list(
            &BindKey::ALL[..], 
            self.scratch_header, 
            Message::ScratchHeaderChanged,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let scratchkey = Text::new(self.scratch_key.clone());
        let mut scratchkeyselect = Button::new(scratchkey).on_press(Message::Capture(CaptureInput::ScratchKey)).width(50);
        
        match self.capturenext.as_ref().unwrap() {
            CaptureInput::NoKey => {
            }
            CaptureInput::ExitKey => {
                exitkeyselect = exitkeyselect.style(style.secondary.mk_theme());
            }
            CaptureInput::KillKey => {
                killkeyselect = killkeyselect.style(style.secondary.mk_theme());
            }
            CaptureInput::LaunchKey => {
                launchkeyselect = launchkeyselect.style(style.secondary.mk_theme());
            }
            CaptureInput::MiniKey => {
                minikeyselect = minikeyselect.style(style.secondary.mk_theme());
            }
            CaptureInput::ScratchKey => {
                scratchkeyselect = scratchkeyselect.style(style.secondary.mk_theme());
            }
        }
        let mut primaryrow = Row::new();
        let mut secondaryrow = Row::new();
        let mut exitscrow = Row::new();
        let mut launchscrow = Row::new();
        let mut killscrow = Row::new();
        let mut miniscrow = Row::new();
        let mut scratchscrow = Row::new();
        if self.index == 0 {
            primarylabel = primarylabel.style(seltext.mk_theme());
        } else if self.index == 1 {
            secondarylabel = secondarylabel.style(seltext.mk_theme());
        } else if self.index == 2 {
            exitsclabel = exitsclabel.style(seltext.mk_theme());
        } else if self.index == 3 {
            launchsclabel = launchsclabel.style(seltext.mk_theme());
        } else if self.index == 4 {
            killsclabel = killsclabel.style(seltext.mk_theme());
        } else if self.index == 5 {
            minisclabel = minisclabel.style(seltext.mk_theme());
        } else if self.index == 6 {
            scratchsclabel = scratchsclabel.style(seltext.mk_theme());
        }
        primaryrow = primaryrow
            .push(primarylabel)
            .push(primarypick)
            .spacing(10);
        secondaryrow = secondaryrow
            .push(secondarylabel)
            .push(secondarypick)
            .spacing(10);
        exitscrow = exitscrow
            .push(exitsclabel)
            .push(exitheaderselect)
            .push(exitkeyselect)
            .spacing(10);
        launchscrow = launchscrow
            .push(launchsclabel)
            .push(launchheaderselect)
            .push(launchkeyselect)
            .spacing(10);
        killscrow = killscrow
            .push(killsclabel)
            .push(killheaderselect)
            .push(killkeyselect)
            .spacing(10);
        miniscrow = miniscrow
            .push(minisclabel)
            .push(miniheaderselect)
            .push(minikeyselect)
            .spacing(10);
        scratchscrow = scratchscrow
            .push(scratchsclabel)
            .push(scratchheaderselect)
            .push(scratchkeyselect)
            .spacing(10);
        settings
            .push(primaryrow)
            .push(secondaryrow)
            .push(exitscrow)
            .push(launchscrow)
            .push(killscrow)
            .push(miniscrow)
            .push(scratchscrow).spacing(10)
    }
    pub fn bar_page(&self, style: ThemeCustom) -> Column<Message> {
        let settings = Column::new();
        let selectionmarker: Text = Text::new("=>");
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
        let barleft = Button::new(Text::new(tr("Left"))).on_press(Message::PushWidget(WidgetBank::Left));
        let barcenter = Button::new(Text::new(tr("Center"))).on_press(Message::PushWidget(WidgetBank::Center));
        let barright = Button::new(Text::new(tr("Right"))).on_press(Message::PushWidget(WidgetBank::Right));
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
        let removeleft = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Left));
        let removecenter = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Center));
        let removeright = Button::new(Text::new(tr("Remove"))).on_press(Message::RemoveWidget(WidgetBank::Right));
        let labelleft = Text::new(left_contents);
        let labelright = Text::new(right_contents);
        let labelcenter = Text::new(center_contents);

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
            widget_row_i = widget_row_i.push(selectionmarker)
        } else if self.index == 1 {
            widget_row_ii = widget_row_ii.push(selectionmarker)
        } else if self.index == 2 {
            widget_row_iii = widget_row_iii.push(selectionmarker)
        } else if self.index == 3 {
            widget_row_iv = widget_row_iv.push(selectionmarker)
        } else if self.index == 4 {
            widget_row_v = widget_row_v.push(selectionmarker)
        } else if self.index == 5 {
            left_row = left_row.push(selectionmarker) 
        } else if self.index == 6 {
            center_row = center_row.push(selectionmarker)
        } else if self.index == 7 {
            right_row = right_row.push(selectionmarker)
        }
        
        left_row = left_row.push(barleft).push(labelleft).push(removeleft).spacing(10);
        center_row = center_row.push(barcenter).push(labelcenter).push(removecenter).spacing(10);
        right_row = right_row.push(barright).push(labelright).push(removeright).spacing(10);
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
        //let selectionmarker: Text = Text::new("=>");
        let seltext = TextStyle {color: style.application.success};
        let widthincr = Button::new("+").on_press(Message::Incr(IncrVal::WidthVal)).width(30);
        let mut widthdecr = Button::new("-").on_press(Message::Decr(IncrVal::WidthVal)).width(30);
        let widthvaluepeek = Text::new(format!("{}", self.border.width));
        let mut widthlabel = Text::new(tr("The Width of The Window Borders:"));

        let mut widthrow = Row::new().spacing(10);

        let gapsincr = Button::new("+").on_press(Message::Incr(IncrVal::GapsVal)).width(30);
        let mut gapsdecr = Button::new("-").on_press(Message::Decr(IncrVal::GapsVal)).width(30);
        let gapsvaluepeek = Text::new(format!("{}", self.border.gaps));
        let mut gapslabel = Text::new(tr("The Size of The Standard Window Gaps:"));

        let mut gapsrow = Row::new().spacing(10);

        let radincr = Button::new("+").on_press(Message::Incr(IncrVal::RadiusVal)).width(30);
        let mut raddecr = Button::new("-").on_press(Message::Decr(IncrVal::RadiusVal)).width(30);
        let radvaluepeek = Text::new(format!("{}", self.border.radius));
        let mut radlabel = Text::new(tr("The roundedness of window corners:"));

        let mut radrow = Row::new().spacing(10);

        let winpick = pick_list(
            &WindowAnimation::ALL[..], 
            self.window_anim, 
            Message::ChangeWindowAnim,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let mut winlabel = Text::new(tr("The Window Animations To Be Used:"));

        let mut winrow = Row::new().spacing(10);

        let workpick = pick_list(
            &WorkAnimation::ALL[..],
            self.work_anim,
            Message::ChangeWorkAnim,
            )
            .placeholder("choose")
            .style(style.list.mk_theme());
        let mut worklabel = Text::new(tr("The Workspace Animations To Be Used:"));

        let mut workrow = Row::new().spacing(10);

        let enable = Text::new(tr("Enable"));
        let disable = Text::new(tr("Disable"));
        let enabled = Text::new(tr("Enabled"));
        let disabled = Text::new(tr("Disabled"));
        let mut blurlabel = Text::new(tr("Whether or not to use window blur"));
        let mut bluron = Button::new(enable).on_press(Message::BlurToggled(true));
        let mut bluroff = Button::new(disable).on_press(Message::BlurToggled(false));
        if self.blur {
            bluron = Button::new(enabled).on_press(Message::BlurToggled(true)).style(style.secondary.mk_theme());
        } else {
            bluroff = Button::new(disabled).on_press(Message::BlurToggled(false)).style(style.secondary.mk_theme());
        }
        let mut blurrow = Row::new().spacing(10);

        if self.border.width == 0 {
            widthdecr = widthdecr.style(style.secondary.mk_theme());
        }
        if self.border.gaps == 0 {
            gapsdecr = gapsdecr.style(style.secondary.mk_theme());
        }
        if self.border.radius == 0 {
            raddecr = raddecr.style(style.secondary.mk_theme());
        }

        if self.index == 0 {
            widthlabel = widthlabel.style(seltext.mk_theme());
        } else if self.index == 1 {
            gapslabel = gapslabel.style(seltext.mk_theme());
        } else if self.index == 2 {
            radlabel = radlabel.style(seltext.mk_theme());
        } else if self.index == 3 {
            winlabel = winlabel.style(seltext.mk_theme());
        } else if self.index == 4 {
            worklabel = worklabel.style(seltext.mk_theme());
        } else if self.index == 5 {
            blurlabel = blurlabel.style(seltext.mk_theme());
        }

        widthrow = widthrow
            .push(widthlabel)
            .push(widthdecr)
            .push(widthvaluepeek)
            .push(widthincr);
        gapsrow = gapsrow
            .push(gapslabel)
            .push(gapsdecr)
            .push(gapsvaluepeek)
            .push(gapsincr);
        radrow = radrow
            .push(radlabel)
            .push(raddecr)
            .push(radvaluepeek)
            .push(radincr);
        winrow = winrow
            .push(winlabel)
            .push(winpick);
        workrow = workrow
            .push(worklabel)
            .push(workpick);
        blurrow = blurrow
            .push(blurlabel)
            .push(bluroff)
            .push(bluron);
        settings
            .push(widthrow)
            .push(gapsrow)
            .push(radrow)
            .push(winrow)
            .push(workrow)
            .push(blurrow).spacing(10)
    }
}