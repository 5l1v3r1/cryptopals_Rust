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

struct DistanceAvg {
    keysize: usize,
    avg_dist: usize,
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
    println!("pew1 {:x} pew2 {:x}",ba1,ba2);
    let keysize = ba1.len();
    let hd: u64 = hamming::distance(ba1,ba2);
    let NormKeySize = hd as usize / keysize;

    NormKeySize
}
/*
 * Get list of hamdist / keysize
 * Get average of each Normalization
 * Pop off of vector until size of block is larger than amount of vector elements left
 * Put average and keysize into a DistanceAvg structure
 * The DistanceAvg structure with the lowest avg distance is the correct keysize
 */
fn getKeySize(ct: &Vec<u8>) -> u64 {
    println!("Getting keysize");
    let mut NormKeyScore: u64 = 100;
    let mut KeyScore: u64 = 0;
    let mut byteArray1 = Vec::<u8>::new();
    let mut byteArray2 = Vec::<u8>::new();
    let mut distance_array: Vec<usize> = Vec::new();
    let mut ct_new = ct.clone();
    let mut distance_avgs: Vec<DistanceAvg> = Vec::new();

    println!("Starting iter");
    for i in 2..38 {
        
        let mut ct_iter = ct_new.clone();
        println!("Cloned ct_iter");
        let mut flag = 0;
        let mut ch1: &[u8] = b"A";
        let mut ch2: &[u8] = b"A";
        println!("Starting chunk iter");
        for ch in ct_iter.chunks(i) {
        
            if ch.len() != i {
                println!("Chunk length: {}",ch.len());
                break;
            }
            if flag == 0 {
                println!("setting chunk1");
                ch1 = ch;
                println!("chunk1 set");
                flag = 1;
            } else {
                ch2 = ch;
                flag = 0;
            }
            println!("Attempting normkeysize {} {}",ch1,ch2);
            distance_array.push(getNormKeySize(&ch1.to_vec(), &ch2.to_vec()));
            println!("normkeysize complete");
        }

        println!("Setting object params");
        let mut sum = 0;
        for d in &distance_array {
            sum += d;
        }

        let avg = sum as u64 / distance_array.len() as u64;
        
        distance_avgs.push(DistanceAvg{
            keysize: i,
            avg_dist: avg as usize,
        });
    }
    println!("Sorting..");
    distance_avgs.sort_by(|a,b| b.avg_dist.cmp(&a.avg_dist));

    distance_avgs[0].keysize as u64
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

    let keysize = getKeySize(&ct); //<- Cant get working

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