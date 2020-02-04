extern crate hex;

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
        if (xstr[i] > 0x2f) && (xstr[i] < 0x3a) {
            chk_bytes_obj.ascii_score += 2;
        }
        if (xstr[i] > 0x40) && (xstr[i] < 0x5b) {
            chk_bytes_obj.ascii_score += 3;
        }
        if (xstr[i] > 0x60) && (xstr[i] < 0x7b) {
            chk_bytes_obj.ascii_score += 3;
        }
    }

    Some(ProbableComputations {
        key_byte: chk_bytes_obj.key_byte,
        ascii_score: chk_bytes_obj.ascii_score,
        decrypted_content: xstr.to_vec(),
    })
}

fn main() {

    let mut prob_comp = ProbableComputations::new();

    let hex_str1: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let hex_bytes: Vec<u8> = hex::decode(hex_str1).unwrap();
    let mut xordbytes: Vec<u8> = Vec::<u8>::new();

    for i in 0..256 {
        for j in 0..hex_bytes.len() {
            xordbytes.push(hex_bytes[j] ^ i as u8);
        }
        match check_ascii_bytes(&xordbytes,i as u8) {
            Some(pc) => {if pc.ascii_score > prob_comp.ascii_score{prob_comp = pc;}},
            None => {}
        }
        xordbytes.clear();
    }

    println!("[+] Key: {:#x}\n[+] Decrypted Message: {}",prob_comp.key_byte,String::from_utf8(prob_comp.decrypted_content).unwrap());
}
