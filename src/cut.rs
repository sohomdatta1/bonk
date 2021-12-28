use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use crate::print_error::*;

pub fn main_cut(
    infile: &str,
    start_str: &str,
    end_str: &str,
    outfile: &str,
) -> Result<(), String> {
    let start: usize = usize::from_str_radix(start_str, 16).map_err(|e| format!("start must be a hex number\n\nCaused by:\n {}", e))?;
    let end: usize = usize::from_str_radix(end_str, 16).map_err(|e| format!("end must be a hex number\n\nCaused by:\n {}", e))?;
    let mut finfile = File::open(infile).map_err(|e| format!("Unable to open `{}`\n\nCaused by:\n {}", infile, e))?;
    let mut foutfile = File::create(outfile).map_err(|e| format!("Unable to open `{}`\n\nCaused by:\n {}", outfile, e))?;
    if end - start <= 0 {
        return Err(format!("end offset must be greater than start offset"));
    } else if end - start < 0x4000 {
        handle_normal_copy(&mut finfile, start, end, &mut foutfile)?;
    } else {
        handle_chunked_copy(&mut finfile, start, end, &mut foutfile)?;
    }
    pe(writeln!(io::stdout(), "Wrote {} ({:#x}) bytes to {}.", end - start,end - start, outfile));
    Ok(())
}

fn handle_normal_copy(
    finfile: &mut File,
    start: usize,
    end: usize,
    foutfile: &mut File,
) -> Result<(), String> {
    let mut buffer = vec![0; end - start];
    finfile
        .seek(SeekFrom::Start(start as u64)).map_err(|e| format!("Cannot seek to {} in <infile>:\n\nCaused by:\n {}", start, e))?;
    finfile.read_exact(&mut buffer).map_err(|e| format!("Cannot read from <infile>:\n\nCaused by:\n {}", e))?;
    foutfile
        .write_all(&buffer)
        .map_err(|e| format!("Cannot write to <outfile>:\n\nCaused by:\n {}", e))?;
    Ok(())
}

fn handle_chunked_copy(
    finfile: &mut File,
    start: usize,
    end: usize,
    foutfile: &mut File,
) -> Result<(), String> {
    let mut buffer = vec![0; 0x4000];
    let mut curr_offset = start;
    finfile
        .seek(SeekFrom::Start(curr_offset as u64))
        .map_err(|e| format!("seek: {}", e))?;
    loop {
        finfile.read_exact(&mut buffer).map_err(|e| format!("Cannot read from <infile> in chunked mode:\n\nCaused by:\n {}", e))?;
        if curr_offset + buffer.len() >= end {
            foutfile
                .write_all(&buffer[0..(end - curr_offset + 1)])
                .map_err(|e| format!("Cannot write to <outfile> in chunked mode:\n\nCaused by:\n {}", e))?;
            return Ok(());
        } else {
            foutfile
                .write_all(&buffer[0..buffer.len()])
                .map_err(|e| format!("Cannot write to <outfile> in chunked mode:\n\nCaused by:\n {}", e))?;
            curr_offset += buffer.len();
        }
    }
}
