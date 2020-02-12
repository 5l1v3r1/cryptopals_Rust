extern crate hex;

use std::fs::File;
use std::io::{BufReader,prelude::*};

struct ByteStreams {
    length: usize,
    bytestream: String,
    line_num: u64,
}

fn handle_line(line: &String, line_num: u64) -> ByteStreams {
    let l = line.clone();
    let rline = l.clone();

    let mut bytes: Vec<u8> = hex::decode(l).unwrap();

    if (bytes.len() % 16) == 0 {

        bytes.sort();
        bytes.dedup();
    }

    ByteStreams {
        length: bytes.len(),
        bytestream: rline,
        line_num: line_num,
    }
}

fn read_lines(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let br = BufReader::new(f);
    let mut lines = Vec::<String>::new();
    for line in br.lines() {
        lines.push(line.unwrap().trim().to_string());
    }

    lines
}

fn main() {
    
    let lines = read_lines("8.txt");
    let mut lowest: Vec<ByteStreams> = Vec::new();
    let mut line_num: u64 = 1;
    for line in lines {
        lowest.push(handle_line(&line,line_num));
        line_num += 1;
    }

    lowest.sort_by(|a,b| a.length.cmp(&b.length));
    println!("[+] Line Number: {}\n[+] Unique bytes: {}\n[+] ECB: {}",lowest[0].line_num,lowest[0].length,lowest[0].bytestream);
}