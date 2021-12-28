use termion::{color, style};
use crate::envman::get_env;

pub fn should_have_color_support() -> bool {
    termion::is_tty(&mut std::io::stdout()) && !get_env("NO_COLOR").len() > 0
}

pub struct ChromeInstance {
    color_is_supported: bool
}

impl ChromeInstance {
    pub fn get_green(&mut self) -> String {
        if self.color_is_supported {
            format!("{}", color::Fg(color::Rgb(0x3c, 0xae, 0xa3)))
        } else {
            String::new()
        }
    }

    pub fn get_red(&mut self) -> String {
        if self.color_is_supported {
            format!("{}", color::Fg(color::Rgb(0xdc, 0x32, 0x2f)))
        } else {
            String::new()
        }
    }

    pub fn get_grey(&mut self) -> String {
        if self.color_is_supported {
            format!("{}", color::Fg(color::Rgb(0x44, 0x44, 0x44)))
        } else {
            String::new()
        }
    }

    pub fn get_reset(&mut self) -> String {
        if self.color_is_supported {
            format!("{}", color::Fg(color::Reset))
        } else {
            String::new()
        }
    }

    pub fn get_bold(&mut self) -> String {
        if self.color_is_supported {
            format!("{}", style::Bold)
        } else {
            String::new()
        }
    }

    pub fn new(color_is_supported: bool) -> ChromeInstance {
        ChromeInstance {
            color_is_supported
        }
    }
}