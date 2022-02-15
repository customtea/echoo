use color_text::{TextColorParam, colorlist};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};

mod color_text;


#[derive(Parser, Debug)]
#[clap(
    name = "ecohoo",
    version = "1.2.1",
    author = "CustomTea",
    about = "colorized echo"
)]
struct Opts {
    /// Set Front Color 
    #[clap(short, long)]
    front_color: Option<String>,
    
    /// Set Background Color 
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
    strike: bool,
    
    /// Color list
    #[clap(short, long)]
    list: bool,

    /// Text
    #[clap(name = "String")]
    text: Vec<String>,
    
    /// File
    #[clap(long, name="FILE")]
    file: Option<String>,
}



fn text_out(opts: Opts){
    let text = opts.text.join(" ");
    
    let param = TextColorParam {
        front_color: opts.front_color,
        back_color: opts.back_color,
        no_newline: opts.no_newline,
        bold: opts.bold,
        under: opts.under,
        italic: opts.italic,
        dimmed: opts.dimmed,
        reverse: opts.reverse,
        blink: opts.blink,
        hidden: opts.hidden,
        strike: opts.strike,
    };
    param.show(text);
}

fn cmd_parser(param: &mut TextColorParam, cmd_buf: &Vec<char>){
    let mut is_prefix = false;
    let mut is_prefix_end = false;
    let mut is_style = Some(true);
    let mut is_frontcolor = None;
    let mut cmd:Vec<char> = Vec::new();
    for c in cmd_buf{
        match c {
            'f' => if !is_prefix && !is_prefix_end{
                is_prefix = true;
                is_frontcolor = Some(true);
            }else{
                cmd.push(*c);
            }
            'b' => if !is_prefix && !is_prefix_end{
                is_prefix = true;
                is_frontcolor = Some(false);
            }else{
                cmd.push(*c);
            }
            'n' => if !is_prefix && !is_prefix_end{
                is_prefix = true;
                is_style = Some(false);
            }else{
                cmd.push(*c);
            }
            '_' => if is_prefix{
                is_prefix = true;
                is_prefix_end = true;
            }else{
                cmd.push(*c);
            }
            _ => {
                cmd.push(*c);
            }
        }
    }
    let cmd = cmd.iter().collect::<String>();
    //println!("{}", cmd);
    match is_frontcolor {
        Some(front) => if front {
            match cmd.as_str() {
                "claer" => {
                    param.front_color = None;
                    return 
                    }
                _ => {
                    param.front_color = Some(cmd);
                    return
                }
            }
        }else{
            match cmd.as_str() {
                "clear" => {
                    param.back_color = None;
                    return 
                    }
                _ => {
                    param.back_color = Some(cmd);
                    return
                }
            }
        }
        None => (),
    }
    match is_style {
        Some(style_on) => match cmd.as_str(){
                "bold" => param.bold = style_on,
                "under" => param.under = style_on,
                "italic" => param.italic = style_on,
                "dimmed" => param.dimmed = style_on,
                "reverse" => param.reverse = style_on,
                "blink" => param.blink = style_on,
                "hidden" => param.hidden = style_on,
                "strike" => param.strike = style_on,
                "clear" => param.clear(),
                _ => (),
            }
        None => (),
    }
}


fn text_parser(param: &mut TextColorParam, text: String){
    let mut is_escape = false;
    let mut is_block = false;
    let mut cmd_buf: Vec<char> = Vec::new();
    let mut nom_buf: Vec<char> = Vec::new();
    param.no_newline = true;
    //let mut char_count = 0;
    for c in text.chars(){
        match c {
            '\\' => if is_escape{ is_escape = false; }else{ is_escape = true; }
            '{' => if is_escape{ is_block = true}else{ is_block = false}
            '}' => if is_escape && is_block {
                is_escape = false;
                is_block = false;
                let t_text = nom_buf.iter().collect::<String>();
                param.show(t_text);
                cmd_parser(param, &cmd_buf);
                
                cmd_buf.clear();
                nom_buf.clear();
            }
            _ => if is_escape && is_block{
                cmd_buf.push(c);
            } else {
                nom_buf.push(c);
                //println!("{}",c);
            }
        }
        //char_count += 1;
    }
    param.no_newline = false;
    let t_text = nom_buf.iter().collect::<String>();
    param.show(t_text);
    cmd_buf.clear();
    nom_buf.clear();
}



fn main() {
    let opts = Opts::parse();
    
    if opts.list {
        colorlist();
        return
    }
    
    if let Some(path) = opts.file {
        let file = File::open(path).expect("File Open Error");
        let file_reader = BufReader::new(file);
        
        let mut param: TextColorParam = TextColorParam::new();
        for line in file_reader.lines() {
            let line = line.unwrap();
            text_parser(&mut param, line);
        } 
        
    }else{
        text_out(opts);
    }
}
