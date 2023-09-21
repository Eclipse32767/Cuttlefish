use iced::keyboard::KeyCode;

use crate::{Configurator, CaptureInput, Page, lib_cfg::{OurTheme, ShortcutKey, BindKey, WindowAnimation, WorkAnimation, BarWidget}};

impl Configurator {
    pub fn kb_parse(&mut self, evt: iced::keyboard::Event) {
        match evt {
            iced::keyboard::Event::KeyPressed { key_code, modifiers} => { // code for handling key presses
                match self.capture_next.as_ref().unwrap() { //check if next input should be captured
                    &CaptureInput::NoKey => { // if no captures are wanted, go through this parsing block
                        if key_code == KeyCode::Up {
                            if iced::keyboard::Modifiers::shift(modifiers) {//go up a page
                                self.current_page = match self.current_page {
                                    Page::Main => {
                                        self.index_max = 1;
                                        Page::Init
                                    }
                                    Page::Bind => {
                                        self.index_max = 4;
                                        Page::Main
                                    }
                                    Page::Anim => {
                                        self.index_max = 7;
                                        Page::Bind
                                    }
                                    Page::Bar => {
                                        self.index_max = 6;
                                        Page::Anim
                                    }
                                    Page::Init => {
                                        self.index_max = 8;
                                        Page::Bar
                                    }
                                };
                                if self.index > self.index_max {
                                    self.index = self.index_max;
                                }
                            } else { //move the mini cursor up
                                if self.index != 0 {
                                    self.index = self.index -1;
                                }
                            }
                        } else if key_code == KeyCode::Down {
                            if iced::keyboard::Modifiers::shift(modifiers) {//go down a page
                                self.current_page = match self.current_page {
                                    Page::Main => {
                                        self.index_max = 7;
                                        Page::Bind
                                    }
                                    Page::Bind => {
                                        self.index_max = 6;
                                        Page::Anim
                                    }
                                    Page::Anim => {
                                        self.index_max = 8;
                                        Page::Bar
                                    }
                                    Page::Bar => {
                                        self.index_max = 1;
                                        Page::Init
                                    }
                                    Page::Init => {
                                        self.index_max = 4;
                                        Page::Main
                                    }
                               };
                               if self.index > self.index_max {
                                    self.index = self.index_max;
                                }
                            } else { //move the mini cursor down
                                if self.index < self.index_max {
                                    self.index = self.index +1;
                                }
                            }
                        } else if key_code == KeyCode::S { //save
                            if self.unsaved {
                                self.mk_config();
                            }
                            self.unsaved = false;
                        } else if key_code == KeyCode::Enter { // if the enter key is pressed, interact with certain widgets
                            match self.current_page {
                                Page::Main => {
                                    if self.index == 0 { // if theme selector block is marked
                                        self.theme = match self.theme {
                                            OurTheme::Light => OurTheme::Dark,
                                            OurTheme::Dark => OurTheme::Custom,
                                            OurTheme::Custom => OurTheme::Light,
                                        };
                                        self.unsaved = true;
                                    }
                                }
                                Page::Bind => { // set the captures if needed
                                    if self.index == 2 {
                                        self.capture_next = Some(CaptureInput::ExitKey);
                                    } else if self.index == 3 {
                                        self.capture_next = Some(CaptureInput::LaunchKey);
                                    } else if self.index == 4 {
                                        self.capture_next = Some(CaptureInput::KillKey);
                                    } else if self.index == 5 {
                                        self.capture_next = Some(CaptureInput::MiniKey);
                                    } else if self.index == 6 {
                                        self.capture_next = Some(CaptureInput::ScratchKey);
                                    }
                                }
                                Page::Bar => {
                                    if self.index >= 5 {
                                        match self.next_widget {
                                            Some(x) => {
                                                if self.index == 5 {
                                                    self.bar_left.push(x);
                                                } else if self.index == 6 {
                                                    self.bar_center.push(x);
                                                } else if self.index == 7 {
                                                    self.bar_right.push(x);
                                                }
                                            }
                                            None => {}
                                        }
                                        println!("{:?}", self.bar_left);
                                        println!("{}", self.bar_center.len());
                                        self.next_widget = None;
                                        self.unsaved = true;
                                    }
                                }
                                Page::Init => {

                                }
                                Page::Anim => {//toggle blur if relevant
                                    if self.index == 5 {
                                        self.blur = !self.blur;
                                        self.unsaved = true;
                                    }
                                }
                            }
                            if self.index == self.index_max {
                                if self.unsaved {
                                    self.mk_config();
                                }
                                self.unsaved = false;
                            }
                        } else if key_code == KeyCode::Key1 {//dropdown management with number keys
                            if self.current_page == Page::Main {
                                if self.index == 1 {
                                    self.primary_key = Some(ShortcutKey::Super);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.secondary_key = Some(ShortcutKey::Super);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bind {
                                if self.index == 0 {
                                    self.primary_key = Some(ShortcutKey::Super);
                                    self.unsaved = true;
                                } else if self.index == 1 {
                                    self.secondary_key = Some(ShortcutKey::Super);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.exit_header = Some(BindKey::PrimaryKey);
                                    self.unsaved = true;
                                } else if self.index == 3 {
                                    self.launch_header = Some(BindKey::PrimaryKey);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.kill_header = Some(BindKey::PrimaryKey);
                                    self.unsaved = true;
                                } else if self.index == 5 {
                                    self.minimize_header = Some(BindKey::PrimaryKey);
                                    self.unsaved = true;
                                } else if self.index == 6 {
                                    self.scratch_header = Some(BindKey::PrimaryKey);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Anim {
                                if self.index == 3 {
                                    self.window_anim = Some(WindowAnimation::None);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.work_anim = Some(WorkAnimation::None);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bar {
                                if self.index == 0 {
                                    self.next_widget = Some(BarWidget::Audio);
                                } else if self.index == 1 {
                                    self.next_widget = Some(BarWidget::Bluetooth);
                                } else if self.index == 2 {
                                    self.next_widget = Some(BarWidget::Disk);
                                } else if self.index == 3 {
                                    self.next_widget = Some(BarWidget::RAM);
                                } else if self.index == 4 {
                                    self.next_widget = Some(BarWidget::Tray);
                                }
                            }
                        } else if key_code == KeyCode::Key2 {
                            if self.current_page == Page::Main {
                                if self.index == 1 {
                                    self.primary_key = Some(ShortcutKey::Alt);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.secondary_key = Some(ShortcutKey::Alt);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bind {
                                if self.index == 0 {
                                    self.primary_key = Some(ShortcutKey::Alt);
                                    self.unsaved = true;
                                } else if self.index == 1 {
                                    self.secondary_key = Some(ShortcutKey::Alt);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.exit_header = Some(BindKey::SecondaryKey);
                                    self.unsaved = true;
                                } else if self.index == 3 {
                                    self.launch_header = Some(BindKey::SecondaryKey);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.kill_header = Some(BindKey::SecondaryKey);
                                    self.unsaved = true;
                                } else if self.index == 5 {
                                    self.minimize_header = Some(BindKey::SecondaryKey);
                                    self.unsaved = true;
                                } else if self.index == 6 {
                                    self.scratch_header = Some(BindKey::SecondaryKey);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Anim {
                                if self.index == 3 {
                                    self.window_anim = Some(WindowAnimation::PopIn);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.work_anim = Some(WorkAnimation::Slide);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bar {
                                if self.index == 0 {
                                    self.next_widget = Some(BarWidget::Backlight);
                                } else if self.index == 1 {
                                    self.next_widget = Some(BarWidget::CPU);
                                } else if self.index == 2 {
                                    self.next_widget = Some(BarWidget::KeyboardState);
                                } else if self.index == 3 {
                                    self.next_widget = Some(BarWidget::Taskbar);
                                } else if self.index == 4 {
                                    self.next_widget = Some(BarWidget::User);
                                }
                            }
                        } else if key_code == KeyCode::Key3 {
                            if self.current_page == Page::Main {
                                if self.index == 1 {
                                    self.primary_key = Some(ShortcutKey::Shift);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.secondary_key = Some(ShortcutKey::Shift);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bind {
                                if self.index == 0 {
                                    self.primary_key = Some(ShortcutKey::Shift);
                                    self.unsaved = true;
                                } else if self.index == 1 {
                                    self.secondary_key = Some(ShortcutKey::Shift);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.exit_header = Some(BindKey::BothKey);
                                    self.unsaved = true;
                                } else if self.index == 3 {
                                    self.launch_header = Some(BindKey::BothKey);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.kill_header = Some(BindKey::BothKey);
                                    self.unsaved = true;
                                } else if self.index == 5 {
                                    self.minimize_header = Some(BindKey::BothKey);
                                    self.unsaved = true;
                                } else if self.index == 6 {
                                    self.scratch_header = Some(BindKey::BothKey);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Anim {
                                if self.index == 3 {
                                    self.window_anim = Some(WindowAnimation::Slide);
                                    self.unsaved = true;
                                } else if self.index == 4 {
                                    self.work_anim = Some(WorkAnimation::SlideVert);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bar {
                                if self.index == 0 {
                                    self.next_widget = Some(BarWidget::Battery);
                                } else if self.index == 1 {
                                    self.next_widget = Some(BarWidget::Clock);
                                } else if self.index == 2 {
                                    self.next_widget = Some(BarWidget::Network);
                                } else if self.index == 3 {
                                    self.next_widget = Some(BarWidget::Temperature);
                                } else if self.index == 4 {
                                    self.next_widget = Some(BarWidget::Workspaces);
                                }
                            }
                        } else if key_code == KeyCode::Key4 {
                            if self.current_page == Page::Main {
                                if self.index == 1 {
                                    self.primary_key = Some(ShortcutKey::Ctrl);
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.secondary_key = Some(ShortcutKey::Ctrl);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Bind {
                                if self.index == 0 {
                                    self.primary_key = Some(ShortcutKey::Ctrl);
                                    self.unsaved = true;
                                } else if self.index == 1 {
                                    self.secondary_key = Some(ShortcutKey::Ctrl);
                                    self.unsaved = true;
                                }
                            } else if self.current_page == Page::Anim {
                                if self.index == 4 {
                                    self.work_anim = Some(WorkAnimation::Fade);
                                    self.unsaved = true;
                                }
                            }
                        } else if key_code == KeyCode::Right {//increment values with right presses
                            if self.current_page == Page::Anim {
                                if self.index == 0 {
                                    self.border.width = self.border.width + 1;
                                    self.unsaved = true;
                                } else if self.index == 1 {
                                    self.border.gaps = self.border.gaps + 1;
                                    self.unsaved = true;
                                } else if self.index == 2 {
                                    self.border.radius = self.border.radius + 1;
                                    self.unsaved = true;
                                } else if self.index == 5 {
                                    self.blur = !self.blur;
                                }
                            }
                        } else if key_code == KeyCode::Left { // decrement values with left presses
                            if self.current_page == Page::Anim {
                                if self.index == 0 && self.border.width > 0 {
                                    self.border.width = self.border.width - 1;
                                    self.unsaved = true;
                                } else if self.index == 1 && self.border.gaps > 0 {
                                    self.border.gaps = self.border.gaps - 1;
                                    self.unsaved = true;
                                } else if self.index == 2 && self.border.radius > 0 {
                                    self.border.radius = self.border.radius - 1;
                                    self.unsaved = true;
                                } else if self.index == 5 {
                                    self.blur = !self.blur;
                                }
                            }
                        } else if key_code == KeyCode::Backspace {
                            if self.current_page == Page::Bar {
                                if self.index >= 5 {
                                    let left = self.bar_left.len();
                                    let right = self.bar_right.len();
                                    let center = self.bar_center.len();
                                    if self.index == 5 && left > 0 {
                                        let val = left - 1;
                                        self.bar_left.remove(val);
                                    } else if self.index == 6 && center > 0 {
                                        let val = center - 1;
                                        self.bar_center.remove(val);
                                    } else if self.index == 7 && right > 0 {
                                        let val = right - 1;
                                        self.bar_right.remove(val);
                                    }
                                    self.unsaved = true;
                                }
                            }
                        }
                    } 
                    &CaptureInput::ExitKey => {
                        self.exit_key = format!("{:?}", key_code);
                        self.capture_next = Some(CaptureInput::NoKey);
                        self.unsaved = true;
                    }
                    &CaptureInput::LaunchKey => {
                        self.launch_key = format!("{:?}", key_code);
                        self.capture_next = Some(CaptureInput::NoKey);
                        self.unsaved = true;
                    }
                    &CaptureInput::KillKey => {
                        self.kill_key = format!("{:?}", key_code);
                        self.capture_next = Some(CaptureInput::NoKey);
                        self.unsaved = true;
                    }
                    &CaptureInput::MiniKey => {
                        self.minimize_key = format!("{:?}", key_code);
                        self.capture_next = Some(CaptureInput::NoKey);
                        self.unsaved = true;
                    }
                    &CaptureInput::ScratchKey => {
                        self.scratch_key = format!("{:?}", key_code);
                        self.capture_next = Some(CaptureInput::NoKey);
                        self.unsaved = true;
                    }
                }
            }
            iced::keyboard::Event::KeyReleased {..} => {

            }
            iced::keyboard::Event::CharacterReceived(..) => {

            }
            iced::keyboard::Event::ModifiersChanged(..) => {

            }
        }
    }
}