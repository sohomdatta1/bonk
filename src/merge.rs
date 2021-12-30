use std::fs::File;
use std::io::prelude::*;

pub fn main_merge(files: &[String]) -> Result<(), String> {
    let outfilename = &files[files.len() - 1];
    let mut foutfile = File::create(outfilename).map_err(|e| format!("Cannot create {}\n\nCaused by:\n {}", outfilename, e))?;
    for i in &files[0..files.len() - 1] {
        let mut finfile = File::open(i).map_err(|e| format!("Cannot open {}\n\nCaused by:\n {}", i, e))?;
        handle_chunked_copy(&mut finfile, &mut foutfile)?;
        println!("Wrote contents of {} into {}.", i, outfilename);
    }
    Ok(())
}

fn handle_chunked_copy(finfile: &mut File, foutfile: &mut File) -> Result<(), String> {
    let buf_size = 0x4000;
    let mut buffer = vec![0; buf_size];
    loop {
        let n = Read::by_ref(finfile)
            .take(buf_size as u64)
            .read_to_end(&mut buffer)
            .map_err(|e| {
                format!(
                    "Cannot read from <infile> in chunked mode:\n\nCaused by:\n {}",
                    e
                )
            })?;
        if n == 0 {
            break;
        }
        foutfile.write_all(&mut buffer).map_err(|e| {
            format!(
                "Cannot write to <outfile> in chunked mode:\n\nCaused by:\n {}",
                e
            )
        })?;
        if n < buf_size {
            break;
        }
    }
    Ok(())
}
