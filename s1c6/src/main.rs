use std::io::{BufReader,prelude::*};
use std::fs::File;

extern crate hamming;
extern crate base64;

struct ProbableComputations {
    key_byte: u8,
    ascii_score: u64,
    decrypted_content: Vec<u8>
}

impl ProbableComputations {
    fn new() -> ProbableComputations {
        ProbableComputations {
            key_byte: 0x00,
            ascii_score: 0,
            decrypted_content: Vec::<u8>::new(),
        }
    }
}

fn check_ascii_bytes(xstr: &Vec<u8>, kb: u8) -> Option<ProbableComputations> {
    let mut chk_bytes_obj = ProbableComputations::new();
    chk_bytes_obj.key_byte = kb;

    for i in 0..xstr.len() {
        if (xstr[i] > 0x1f) && (xstr[i] < 0x7e) {
            chk_bytes_obj.ascii_score += 1;
        }
        if (xstr[i] >= 0x00) && (xstr[i] < 0x20) {
            if chk_bytes_obj.ascii_score > 1 {
                chk_bytes_obj.ascii_score -= 1
            }
        }
        if (xstr[i] > 0x21) && (xstr[i] < 0x30) {
            if chk_bytes_obj.ascii_score > 1 {
                chk_bytes_obj.ascii_score -= 1;
            }
        }
        if (xstr[i] > 0x39) && (xstr[i] < 0x41) {
            if chk_bytes_obj.ascii_score > 1 {
                chk_bytes_obj.ascii_score -= 1;
            }
        }
        if xstr[i] == 0x20 {
            chk_bytes_obj.ascii_score += 3;
        }
        if (xstr[i] > 0x2f) && (xstr[i] < 0x3a) {
            chk_bytes_obj.ascii_score += 2;
        }
        if (xstr[i] > 0x40) && (xstr[i] < 0x5b) {
            chk_bytes_obj.ascii_score += 4;
        }
        if (xstr[i] > 0x60) && (xstr[i] < 0x7b) {
            chk_bytes_obj.ascii_score += 5;
        }
    }

    Some(ProbableComputations {
        key_byte: chk_bytes_obj.key_byte,
        ascii_score: chk_bytes_obj.ascii_score,
        decrypted_content: xstr.to_vec(),
    })
}

fn getNormKeySize(ba1: &Vec<u8>, ba2: &Vec<u8>) -> usize {

    let keysize = ba1.len();
    let hd: u64 = hamming::distance(ba1,ba2);
    //let NormKeySize = hd as usize / keysize;

    //NormKeySize
    hd as usize
}

fn getKeySize(ct: &Vec<u8>) -> u64 {
    let mut NormKeyScore: u64 = 100;
    let mut KeyScore: u64 = 0;
    let mut byteArray1 = Vec::<u8>::new();
    let mut byteArray2 = Vec::<u8>::new();
    let mut byteArray3 = Vec::<u8>::new();
    let mut byteArray4 = Vec::<u8>::new();
    let mut KSArray = Vec::<usize>::new();

    for i in 2..38 {
        for j in 0..i {
            //println!("ba1 index: {}",j);
            byteArray1.push(ct[j]);
            //println!("ba2 index: {}", j+i);
            byteArray2.push(ct[j+i]);

            byteArray3.push(ct[j+(i*2)]);
            byteArray4.push(ct[j+(i*3)]);
        }
        
        KSArray.push(getNormKeySize(&byteArray1, &byteArray2));
        
        /*if (nks as u64) < NormKeyScore {
            println!("New lowest nks: {}\tSet KeyScore: {}",nks,i);
            NormKeyScore = nks as u64;
            KeyScore = i as u64;
        }*/

        byteArray1.clear();
        byteArray2.clear();
    }
    let KS_sum: usize = KSArray.iter().sum();
    KeyScore = KS_sum as u64 / KSArray.len() as u64;
    println!("Normalized Bit Distance: {}",KeyScore);
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

    let keysize = 29;//getKeySize(&ct); //<- Cant get working

    //println!("KeySize: {}",keysize);

    let mut BlockArray: Vec<Vec<u8>> = Vec::new();

    for i in 0..keysize {
        BlockArray.push(Vec::new());
    }



    let ct_chunks = ct.chunks(keysize as usize);

    let chunk_amount = ct_chunks.len();
    
    let mut end_count = ct.len();


    for ct_chunk in ct_chunks {
        for i in 0..keysize {
            if end_count == 0 {
                break;
            } else {
                BlockArray[i as usize].push(ct_chunk[i as usize]);
                end_count -= 1;
            }
        }
    }

    let mut encryption_key: Vec<u8> = Vec::<u8>::new();
    let mut prob_comp = ProbableComputations::new();

    println!("Decrypting {} blocks with keysize {}\n",chunk_amount,keysize);

    for line in BlockArray {

        let mut xordbytes: Vec<u8> = Vec::<u8>::new();

        for i in 0..256 {
            for j in 0..line.len() {
                xordbytes.push(line[j] ^ i as u8);
            }
            match check_ascii_bytes(&xordbytes,i as u8) {
                Some(pc) => {if pc.ascii_score > prob_comp.ascii_score{prob_comp = pc;}},
                None => {}
            }
            xordbytes.clear();
        }
        println!("Key Byte Score: {}\tKey Byte: {:02X}",prob_comp.ascii_score,prob_comp.key_byte);
        encryption_key.push(prob_comp.key_byte);
        prob_comp = ProbableComputations::new();
    }
    
    let mut kb_count = encryption_key.len();
    print!("\nKey Found: [");
    for kb in &encryption_key {
        if kb_count != 1 {
            print!("{:02X}:",kb);
            kb_count -= 1;
        } else {
            print!("{:02X}",kb);
        }
    }
    print!("]\n");
    println!("Plaintext Key: [{}]",String::from_utf8(encryption_key).unwrap());
}