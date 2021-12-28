use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn main_cut(infile: &str, offset1_str: &str, offset2_str: &str, outfile: &str) {
    let offset1: usize =
        usize::from_str_radix(offset1_str, 16).expect("Offset 1 needs to be a hex number");
    let offset2: usize =
        usize::from_str_radix(offset2_str, 16).expect("Offset 2 needs to be a hex number");
    let mut finfile = File::open(infile).expect("File not found");
    let mut foutfile = File::create(outfile).expect("Outfile cannot be created");
    if offset2 - offset1 <= 0 {
        println!("Offset 2 needs to be greater than offset 1");
        return;
    } else if offset2 - offset1 > 0x4000 {
        handle_normal_copy(&mut finfile, offset1, offset2, &mut foutfile);
    } else {
        handle_chunked_copy(&mut finfile, offset1, offset2, &mut foutfile);
    }
}

fn handle_normal_copy(finfile: &mut File, offset1: usize, offset2: usize, foutfile: &mut File) {
    let mut buffer = vec![0; offset2 - offset1];
    finfile
        .seek(SeekFrom::Start(offset1 as u64))
        .expect("Error seeking in file");
    finfile.read_exact(&mut buffer).expect("Error reading file");
    foutfile
        .write_all(&buffer)
        .expect("Error writing to outfile");
}

fn handle_chunked_copy(finfile: &mut File, offset1: usize, offset2: usize, foutfile: &mut File) {
    let mut buffer = vec![0; 0x4000];
    let mut curr_offset = offset1;
    finfile
        .seek(SeekFrom::Start(curr_offset as u64))
        .expect("Error seeking in file");
    loop {
        finfile.read_exact(&mut buffer).expect("Error reading file");
        if curr_offset + buffer.len() >= offset2 {
            foutfile
                .write_all(&buffer[0..offset2 - curr_offset])
                .expect("Error writing to outfile");
            return;
        } else {
            foutfile
                .write_all(&buffer[0..buffer.len()])
                .expect("Error writing to outfile");
            curr_offset += buffer.len();
        }
    }
}
