extern crate hex;

fn main() {
    
    let hex_str1: &str = "1c0111001f010100061a024b53535009181c";
    let hex_str2: &str = "686974207468652062756c6c277320657965";

    println!("Hex1: {}",hex_str1);
    println!("Hex2: {}\n",hex_str2);

    let hex_bytes1: Vec<u8> = hex::decode(hex_str1).unwrap();
    let hex_bytes2: Vec<u8> = hex::decode(hex_str2).unwrap();
    let mut xor_bytes: Vec<u8> = Vec::<u8>::new();

    for i in 0..hex_bytes1.len() {
        println!("XOR\t{:#x}\t{:#x}\t-> {:#x}",hex_bytes1[i],hex_bytes2[i],(hex_bytes1[i] ^ hex_bytes2[i]) & 0xff);
        xor_bytes.push(hex_bytes1[i] ^ hex_bytes2[i]);
    }
    
    let xor_hex: &str = &hex::encode(xor_bytes);

    println!("\nHex XOR'd -> {}",xor_hex);
}
