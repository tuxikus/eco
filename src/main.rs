use std::env;

enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    None,
}

struct TextInput {
    lst: Vec<String>
}

fn prnt(text_input: &TextInput, new_line: bool, color: Color) {
    let l = text_input.lst.len();
    let mut i = 0;

    let color_prefix = match color {
        Color::Black => "\x1b[1;30m",
        Color::Red =>  "\x1b[1;31m",
        Color::Green => "\x1b[1;32m",
        Color::Yellow => "\x1b[1;33m",
        Color::Blue => "\x1b[1;34m",
        Color::Magenta => "\x1b[1;35m",
        Color::Cyan => "\x1b[1;36m",
        Color::White => "\x1b[1;37m",
        Color::None => "",
    };
    
    for text in &text_input.lst {
        if i == l-1 {
            print!("{}{}", color_prefix, text);
        } else {
            print!("{}{} ", color_prefix, text);
        }

        i = i+1;
    }

    if new_line {
        println!("\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // no args => just print a newline
    if args.len() == 1 {
        println!();
    } else {
        match args[1].as_str() {
            // print no newline
            "-n" => {
                let text_input: TextInput = TextInput{
                    lst: args[2..].to_vec(),
                };

                prnt(&text_input, false, Color::None);
            },
            
            "-c" => {
                let text_input: TextInput = TextInput{
                    lst: args[3..].to_vec(),
                };

                let color = match args[2].as_str() {
                    "black" | "BLACK" => Color::Black,
                    "red" | "RED" => Color::Red,
                    "green" | "GREEN" => Color::Green,
                    "yellow" | "YELLOW" => Color::Yellow,
                    "blue" | "BLUE" => Color::Blue,
                    "magenta" | "MAGENTA" => Color::Magenta,
                    "cyan" | "CYAN" => Color::Cyan,
                    "white" | "WHITE" => Color::White,
                    _ => todo!(),
                };

                prnt(&text_input, true, color)
            },
            // print the input
            _ => {
                let text_input: TextInput = TextInput{
                    lst: args[1..].to_vec(),
                };

                prnt(&text_input, true, Color::None);
            }
        }
    }
}
