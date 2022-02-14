use colored::{Colorize, ColoredString};
use std::{i64, num::{ParseIntError}};


#[derive(Debug, Clone)]
pub struct TextColorParam {
    pub front_color: Option<String>,
    pub back_color: Option<String>,
    pub no_newline: bool,
    pub bold: bool,
    pub under: bool,
    pub italic: bool,
    pub dimmed: bool,
    pub reverse: bool,
    pub blink: bool,
    pub hidden: bool,
    pub strike: bool,
}

impl TextColorParam {
    pub fn new() -> TextColorParam{
        let param = TextColorParam {
            front_color: None,
            back_color: None,
            no_newline: false,
            bold: false,
            under: false,
            italic: false,
            dimmed: false,
            reverse: false,
            blink: false,
            hidden: false,
            strike: false,
        };
        return param
    }
    
    pub fn clear(&mut self){
        self.front_color = None;
        self.back_color = None;
        self.no_newline = false;
        self.bold = false;
        self.under = false;
        self.italic = false;
        self.dimmed = false;
        self.dimmed = false;
        self.reverse = false;
        self.blink = false;
        self.hidden = false;
        self.strike = false;
    }
    
    fn hex_to_rgb(&self, colorcode: &String) -> (u8, u8, u8){
        let red: Result<i64, ParseIntError>     = i64::from_str_radix(&colorcode[1..3], 16);
        let green: Result<i64, ParseIntError>   = i64::from_str_radix(&colorcode[3..5], 16);
        let blue: Result<i64, ParseIntError>    = i64::from_str_radix(&colorcode[5..7], 16);
        
        let red:u8 = match red {
            Ok(n) => n as u8,
            Err(_) => 0,
        };
        let green: u8 = match green {
            Ok(n) => n as u8,
            Err(_) => 0,
        };
        let blue: u8 = match blue {
            Ok(n) => n as u8,
            Err(_) => 0,
        };

        return (red, green, blue)
    }
    
    pub fn render(&self, text: String) -> ColoredString{
        let mut colored_text = text.normal();
        match self.front_color.as_ref() {
            Some(color) => match color.to_lowercase().as_str() {
                "black" =>      colored_text = text.black(),
                "red" =>        colored_text = text.red(),
                "green" =>      colored_text = text.green(),
                "yellow" =>     colored_text = text.yellow(),
                "blue" =>       colored_text = text.blue(),
                "magenta" =>    colored_text = text.magenta(),
                "purple" =>     colored_text = text.purple(),
                "cyan" =>       colored_text = text.cyan(),
                "white" =>      colored_text = text.white(),
                "bright_black" =>   colored_text = text.bright_black(),
                "bright_red" =>     colored_text = text.bright_red(),
                "bright_green" =>   colored_text = text.bright_green(),
                "bright_yellow" =>  colored_text = text.bright_yellow(),
                "bright_blue" =>    colored_text = text.bright_blue(),
                "bright_magenta" => colored_text = text.bright_magenta(),
                "bright_purple" =>  colored_text = text.bright_purple(),
                "bright_cyan" =>    colored_text = text.bright_cyan(),
                "bright_white" =>   colored_text = text.bright_white(),
                "silver" =>     colored_text = text.truecolor(0xDD, 0xDD, 0xDD),
                "gray" =>       colored_text = text.truecolor(0xAA, 0xAA, 0xAA),
                "maroon" =>     colored_text = text.truecolor(0x85, 0x14, 0x4B),
                "olive" =>      colored_text = text.truecolor(0x3D, 0x99, 0x79),
                "lime" =>       colored_text = text.truecolor(0x01, 0xFF, 0x70),
                "aqua" =>       colored_text = text.truecolor(0x7F, 0xDB, 0xFF),
                "teal" =>       colored_text = text.truecolor(0x39, 0xCC, 0xCC),
                "navy" =>       colored_text = text.truecolor(0x00, 0x1F, 0x3F),
                "fuchsia" =>    colored_text = text.truecolor(0xF0, 0x12, 0xBE),
                _ => {
                    if color.len() == 7 && color.chars().nth(0).unwrap_or(' ') == '#'{
                        let (r,g,b) = self.hex_to_rgb(&color);
                        colored_text = text.truecolor(r, g, b)
                    }else{
                        colored_text = text.normal()
                    }
                }
            },
            None => colored_text = text.normal(),
        }
        
        match self.back_color.as_ref() {
            Some(color) => match color.to_lowercase().as_str() {
                "black" =>      colored_text = colored_text.on_black(),
                "red" =>        colored_text = colored_text.on_red(),
                "green" =>      colored_text = colored_text.on_green(),
                "yellow" =>     colored_text = colored_text.on_yellow(),
                "blue" =>       colored_text = colored_text.on_blue(),
                "magenta" =>    colored_text = colored_text.on_magenta(),
                "purple" =>     colored_text = colored_text.on_purple(),
                "cyan" =>       colored_text = colored_text.on_cyan(),
                "white" =>      colored_text = colored_text.on_white(),
                "bright_black" =>      colored_text = colored_text.on_bright_black(),
                "bright_red" =>        colored_text = colored_text.on_bright_red(),
                "bright_green" =>      colored_text = colored_text.on_bright_green(),
                "bright_yellow" =>     colored_text = colored_text.on_bright_yellow(),
                "bright_blue" =>       colored_text = colored_text.on_bright_blue(),
                "bright_magenta" =>    colored_text = colored_text.on_bright_magenta(),
                "bright_purple" =>     colored_text = colored_text.on_bright_purple(),
                "bright_cyan" =>       colored_text = colored_text.on_bright_cyan(),
                "bright_white" =>      colored_text = colored_text.on_bright_white(),
                "silver" =>     colored_text = text.on_truecolor(0xDD, 0xDD, 0xDD),
                "gray" =>       colored_text = text.on_truecolor(0xAA, 0xAA, 0xAA),
                "maroon" =>     colored_text = text.on_truecolor(0x85, 0x14, 0x4B),
                "olive" =>      colored_text = text.on_truecolor(0x3D, 0x99, 0x79),
                "lime" =>       colored_text = text.on_truecolor(0x01, 0xFF, 0x70),
                "aqua" =>       colored_text = text.on_truecolor(0x7F, 0xDB, 0xFF),
                "teal" =>       colored_text = text.on_truecolor(0x39, 0xCC, 0xCC),
                "navy" =>       colored_text = text.on_truecolor(0x00, 0x1F, 0x3F),
                "fuchsia" =>    colored_text = text.on_truecolor(0xF0, 0x12, 0xBE),
                _ => {
                    if color.len() == 7 && color.chars().nth(0).unwrap_or(' ') == '#'{
                        let (r,g,b) = self.hex_to_rgb(&color);
                        colored_text = colored_text.on_truecolor(r, g, b)
                    }else{
                        colored_text = colored_text
                    }
                }
            },
            None => colored_text = colored_text
        }
        
        let mut deco_text = colored_text;

        if self.bold{ deco_text = deco_text.bold(); }
        if self.under{ deco_text = deco_text.underline(); }
        if self.italic{ deco_text = deco_text.italic(); }
        if self.dimmed{ deco_text = deco_text.dimmed(); }
        if self.reverse{ deco_text = deco_text.reversed(); }
        if self.blink{ deco_text = deco_text.blink(); }
        if self.hidden{ deco_text = deco_text.hidden(); }
        if self.strike{ deco_text = deco_text.strikethrough(); }
        
        return deco_text
    }
    
    pub fn show(&self, text: String) {
        let deco_text = self.render(text);
        if self.no_newline {
            print!("{}", deco_text);
        }else{
            println!("{}", deco_text)
        }
    }
}