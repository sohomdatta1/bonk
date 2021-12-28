use std::io;
use std::io::ErrorKind;

pub fn pe(res: Result<(), io::Error>) {
    match res {
        Ok(_) => {}
        Err(e) => {
            match e.kind() {
                ErrorKind::BrokenPipe => {
                    std::process::exit(1); // exit gracefully without panicking cause it's not our problem
                }
                _ => {
                    panic!("{}", e); // panick!!!!!!!!!!!!
                }
            }
        }
    }
}
