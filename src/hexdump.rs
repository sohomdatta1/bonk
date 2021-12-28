use crate::print_error::*;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use crate::chrome::*;

pub fn main_hexdump(filename: &str, is_output_colored: bool) -> Result<(), String> {
    let b_arr_size: usize = 0x4000;
    let mut f = File::open(filename).map_err(|e| format!("Cannot open {}\n\nCaused by:\n {}",filename, e))?;
    let chrome_instance = ChromeInstance::new(is_output_colored);
    let mut h = Hexdump::new(chrome_instance);
    loop {
        let mut b_arr = Vec::with_capacity(b_arr_size);
        let n = Read::by_ref(&mut f)
            .take(b_arr_size as u64)
            .read_to_end(&mut b_arr).map_err(|e| format!("Cannot read from {}\n\nCaused by:\n {}",filename, e))?;
        if n == 0 {
            break;
        }
        let mut i = 16;
        while i <= n {
            h.process_bytes(&b_arr[i - 16..i]);
            i += 16
        }
        if n - (i - 16) != 0 {
            h.process_bytes(&b_arr[(i - 16)..n]);
        }
    }

    pe(writeln!(
        io::stdout(),
        "+----------+-----------------------------------------+------------------+"
    ));
    Ok(())
}

struct Hexdump {
    byte_count: u64,
    space_req: bool,
    chrome_instance: ChromeInstance
}

impl Hexdump {
    fn new(mut chrome_instance: ChromeInstance) -> Hexdump {
        pe(writeln!(
            io::stdout(),
            "+----------+-----------------------------------------+------------------+"
        ));
        pe(writeln!(
            io::stdout(),
            "| {}-offset- : 0 1  2 3  4 5  6 7  8 9  A B  C D  E F  | 0123456789ABCDEF{} |",
            chrome_instance.get_bold(),
            chrome_instance.get_reset()
        ));
        pe(writeln!(
            io::stdout(),
            "+----------+-----------------------------------------+------------------+"
        ));
        Hexdump {
            byte_count: 0,
            space_req: false,
            chrome_instance
        }
    }

    fn is_print(curr_byte: u8) -> bool {
        curr_byte >= 0x20 && curr_byte <= 0x7E
    }

    fn process_bytes(&mut self, curr_byte_arr: &[u8]) {
        pe(write!(
            io::stdout(),
            "| {}{:08x}{} : ",
            self.chrome_instance.get_red(),
            self.byte_count,
            self.chrome_instance.get_green()
        ));
        for curr_byte in curr_byte_arr {
            if Hexdump::is_print(*curr_byte) {
                pe(write!(
                    io::stdout(),
                    "{}{:02x}{}",
                    self.chrome_instance.get_green(),
                    curr_byte,
                    self.chrome_instance.get_reset()
                ));
            } else if *curr_byte == 0x00 {
                pe(write!(
                    io::stdout(),
                    "{}{}{}",
                    self.chrome_instance.get_grey(),
                    "00",
                    self.chrome_instance.get_reset()
                ));
            } else {
                pe(write!(io::stdout(), "{:02x}", curr_byte));
            }

            if self.space_req {
                pe(write!(io::stdout(), " "));
            }

            self.space_req = !self.space_req;

            self.byte_count += 1;
        }

        if 16 - curr_byte_arr.len() != 0 {
            for _ in 0..(16 - curr_byte_arr.len()) {
                pe(write!(io::stdout(), ".."));

                if self.space_req {
                    pe(write!(io::stdout(), " "));
                }

                self.space_req = !self.space_req;
                self.byte_count += 1;
            }
        }

        pe(write!(io::stdout(), "| "));

        for curr_byte in curr_byte_arr {
            if Hexdump::is_print(*curr_byte) {
                pe(write!(io::stdout(), "{}", *curr_byte as char,));
            } else {
                pe(write!(io::stdout(), "."));
            }
        }

        if 16 - curr_byte_arr.len() != 0 {
            for _ in 0..(16 - curr_byte_arr.len()) {
                pe(write!(io::stdout(), "."));
            }
        }

        pe(writeln!(io::stdout(), " |"));
    }
}
