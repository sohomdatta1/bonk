use crate::print_error::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use termion::color;

pub fn main_strings(filename: &str) {
    let b_arr_size: usize = 0x4000;
    let mut f = File::open(filename).expect("File not found");
    let mut s = Strings::new();
    loop {
        let mut b_arr = Vec::with_capacity(b_arr_size);
        let n = Read::by_ref(&mut f)
            .take(b_arr_size as u64)
            .read_to_end(&mut b_arr)
            .expect("Error reading file");
        if n == 0 {
            break;
        }
        for i in 0..n {
            s.process_byte(b_arr[i]);
        }
        if n < b_arr_size {
            break;
        }
    }
}

struct Strings {
    byte_count: u64,
    curr_byte_offset: u64,
    curr_string: String,
}

impl Strings {
    fn new() -> Strings {
        Strings {
            byte_count: 0,
            curr_byte_offset: 0,
            curr_string: String::new(),
        }
    }

    fn is_print(curr_byte: u8) -> bool {
        curr_byte >= 0x20 && curr_byte <= 0x7E
    }

    fn process_byte(&mut self, curr_byte: u8) {
        if Strings::is_print(curr_byte) {
            self.curr_string.push(curr_byte as char);
            self.byte_count += 1;
        } else {
            if self.curr_string.len() == 0 {
                self.byte_count += 1;
                self.curr_byte_offset = self.byte_count;
            } else {
                // paging + Linux being weird here
                pe(writeln!(
                    io::stdout(),
                    "{}{:#08x}{}: \"{}{}{}\"",
                    color::Fg(color::Rgb(0xdc, 0x32, 0x2f)),
                    self.curr_byte_offset,
                    color::Fg(color::Reset),
                    color::Fg(color::Rgb(0x3c, 0xae, 0xa3)),
                    self.curr_string,
                    color::Fg(color::Reset)
                ));
                self.curr_string.clear();
                self.byte_count += 1;
            }
        }
        self.byte_count = self.byte_count + 1;
    }
}
