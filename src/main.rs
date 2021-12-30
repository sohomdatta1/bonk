use pager::Pager;
use std::env;
use std::io;
use std::process;

mod asciiart;
mod chrome;
mod cut;
mod envman;
mod hexdump;
mod merge;
mod print_error;
mod strings;

static ERROR_EXIT_CODE: i32 = -1;

fn help(prog_name: &str) {
    println!("Usage: {} <operation> [filename] [...options]", prog_name);
    println!("Operations:");
    println!(" help - you're looking at it");
    println!(" version - print version");
    println!(" str <file> - extract all strings in the file");
    println!(" hex <file> - hexdump the file");
    println!(" cut <file> <start> <end> <outfile> - extract a section of the file");
    println!(" merge <infile1> <infile2> .... <outfile> - concat two files");
    println!(" rev <file> <outfile> - reverse the file");
    println!(" diff <file1> <file2> - diff two files at the binary level");
    println!(" file <file> - print file information");
    println!(" asciiart - print some cool ascii art");
    println!(" elf <file> - parse as elf and output useful information");
    println!(" elf dump <file> <offset> - print x86 disassembly");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help(&args[0]);
        process::exit(ERROR_EXIT_CODE);
    }

    let operation = &args[1];
    match operation.as_str() {
        "help" => help(&args[0]),
        "version" => println!("0.0.1"),
        "str" => {
            if args.len() < 3 {
                println!("Usage: {} str <filename>", args[0]);
                process::exit(ERROR_EXIT_CODE);
            }

            let should_have_color_support = chrome::should_have_color_support();

            Pager::with_pager("less -R").setup();

            match strings::main_strings(&args[2], should_have_color_support) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "hex" => {
            if args.len() < 3 {
                println!("Usage: {} hex <filename>", args[0]);
                process::exit(ERROR_EXIT_CODE);
            }

            let should_have_color_support = chrome::should_have_color_support();

            if termion::is_tty(&mut io::stdout()) {
                envman::set_env("LESS", "-Ps| -offset- \\: 0 1  2 3  4 5  6 7  8 9  A B  C D  E F  | 0123456789ABCDEF |");
                // now that's what I call a hack :)
            }

            let mut pager = Pager::with_pager("less -R");
            pager.setup();

            match hexdump::main_hexdump(&args[2], should_have_color_support) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "cut" => {
            if args.len() < 6 {
                println!("Usage: {} cut <filename> <start> <end> <outfile>", args[0]);
                process::exit(ERROR_EXIT_CODE);
            }
            match cut::main_cut(&args[2], &args[3], &args[4], &args[5]) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "merge" => {
            if args.len() < 4 {
                print!(
                    "Usage : {} merge <infile1> ...more files <outfile>",
                    args[0]
                );
                process::exit(ERROR_EXIT_CODE);
            }
            match merge::main_merge(&args[2..args.len()]) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "rev" => println!("NOT IMPLEMENTED"),
        "diff" => println!("NOT IMPLEMENTED"),
        "file" => println!("NOT IMPLEMENTED"),
        "elf" => println!("NOT IMPLEMENTED"),
        "asciiart" => asciiart::asciiart(),
        _ => {
            println!("Unknown operation: {}", operation);
            help(&args[0]);
            process::exit(ERROR_EXIT_CODE);
        }
    }
}
