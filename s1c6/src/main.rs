use std::io::{BufReader,prelude::*};
use std::fs::File;

extern crate hamming;
extern crate base64;

fn getNormKeySize(ba1: &Vec<u8>, ba2: &Vec<u8>) -> usize {

    let keysize = ba1.len();
    let hd: u64 = hamming::distance(ba1,ba2);
    let NormKeySize = hd as usize / keysize;

    NormKeySize
}

fn getKeySize(ct: &Vec<u8>) -> u64 {
    let mut NormKeyScore: u64 = 100;
    let mut KeyScore: u64 = 0;
    let mut byteArray1 = Vec::<u8>::new();
    let mut byteArray2 = Vec::<u8>::new();

    for i in 2..40 {
        for j in 0..i {
            //println!("ba1 index: {}",j);
            byteArray1.push(ct[j]);
            //println!("ba2 index: {}", j+i);
            byteArray2.push(ct[j+i]);
        }
        
        let nks = getNormKeySize(&byteArray1, &byteArray2);
        
        if (nks as u64) < NormKeyScore {
            println!("New lowest nks: {}\tSet KeyScore: {}",nks,i);
            NormKeyScore = nks as u64;
            KeyScore = i as u64;
        }

        byteArray1.clear();
        byteArray2.clear();
    }

    KeyScore
}

fn read_lines() -> Vec<String> {

    let f = File::open("6.txt").unwrap();
    let f = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();

    for line in f.lines() {
        lines.push(line.unwrap().trim().to_string());
    }

    lines
}

fn main() {

    let mut ct: Vec<u8> = Vec::new();

    for line in read_lines() {
        ct = base64::decode(&line).unwrap()
    }

    println!("CipherText Length: {}", ct.len());

    let keysize = getKeySize(&ct);

    println!("KeySize: {}",keysize);

    let mut BlockArray: Vec<Vec<u8>> = Vec::new();
/*
    for i in 0..keysize {
        BlockArray[i as usize] = Vec::<u8>::new();
    }
*//*
    for i in 0..BlockArray.len()+1 {
        let c = BlockArray.len();
        for j in 0..ct {
            BlockArray[i].push(ct[j*c]);
        }
    }*/

    let mut iter = ct.chunks(5);
    for ch in iter {
        let mut i: u64 = 0;
        let blockline: Vec<u8> = Vec::new();
        for c in ch {
            print!("{:02X}\t",c);

            BlockArray[i as usize].push();
        }
        i = 0;
        println!("");
    }
    for b in BlockArray {
        println!("{:?}",b);
    }
}