use colored::{Colorize};
use clap::Parser;
use std::i64;

#[derive(Parser, Debug)]
#[clap(
    name = "ecohoo",
    version = "1.0.0",
    author = "CustomTea",
    about = "colorized echo"
)]
struct Opts {
    /// Set Front Color (black, red, green, yellow, blue, magenta, cyan, white, "#85144B", bright_red,...)
    #[clap(short, long)]
    front_color: Option<String>,
    
    /// Set Background Color (black, red, green, yellow, blue, magenta, cyan, white, "#3D9979", bright_red,...)
    #[clap(short, long)]
    back_color: Option<String>,
    
    /// no Newline
    #[clap(short, long)]
    no_newline: bool,

    /// Bold
    #[clap(long)]
    bold: bool,

    /// Underline
    #[clap(long)]
    under: bool,

    /// Italic
    #[clap(long)]
    italic: bool,

    /// Dimmed
    #[clap(long)]
    dimmed: bool,

    /// Reversed
    #[clap(long)]
    reverse: bool,

    /// blink
    #[clap(long)]
    blink: bool,

    /// hidden
    #[clap(long)]
    hidden: bool,

    /// Strike through
    #[clap(long)]
    through: bool,

    
    /// Text
    #[clap(name = "String")]
    text: Vec<String>,
}

fn colorcode_convert(colorcode: String) -> (u8, u8, u8){
    let red = i64::from_str_radix(&colorcode[1..3], 16);
    let green = i64::from_str_radix(&colorcode[3..5], 16);
    let blue = i64::from_str_radix(&colorcode[5..7], 16);
    
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

fn main() {
    let opts = Opts::parse();
    
    let text = opts.text.join(" ");
    
    let mut colored_text = text.normal();
    match opts.front_color {
        Some(color) => match color.as_str() {
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
            _ => {
                if color.len() == 7 && color.chars().nth(0).unwrap() == '#'{
                    let (r,g,b) = colorcode_convert(color);
                    colored_text = text.truecolor(r, g, b)
                }else{
                    colored_text = text.normal()
                }
            }
        },
        None => colored_text = text.normal(),
    }
    
    match opts.back_color {
        Some(color) => match color.as_str() {
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
            _ => {
                if color.len() == 7 && color.chars().nth(0).unwrap() == '#'{
                    let (r,g,b) = colorcode_convert(color);
                    colored_text = colored_text.on_truecolor(r, g, b)
                }else{
                    colored_text = colored_text
                }
            }
        },
        None => colored_text = colored_text
    }
    
    let mut deco_text = colored_text;

    if opts.bold{
        deco_text = deco_text.bold();
    }
    if opts.under{
        deco_text = deco_text.underline();
    }
    if opts.italic{
        deco_text = deco_text.italic();
    }
    if opts.dimmed{
        deco_text = deco_text.dimmed();
    }
    if opts.reverse{
        deco_text = deco_text.reversed();
    }
    if opts.blink{
        deco_text = deco_text.blink();
    }
    if opts.hidden{
        deco_text = deco_text.hidden();
    }
    if opts.through{
        deco_text = deco_text.strikethrough();
    }

    if opts.no_newline {
        print!("{}", deco_text)
    }else{
        println!("{}", deco_text)
    }
}
