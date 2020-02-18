extern crate crypto;
extern crate base64;

use crypto::aes::{self,KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer,RefWriteBuffer};

use std::fs::File;
use std::io::prelude::*;


fn main() {
    
    let mut decryptor = aes::cbc_decryptor(KeySize::KeySize128, b"YELLOW SUBMARINE", b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00", NoPadding);

    let mut f = File::open("10.txt").unwrap();
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer).unwrap();

    let mut outbuf: [u8; 4096] = [0;4096];
    let mut inbuf = base64::decode(&buffer).unwrap();

    println!("[+] Decrypting message...");
    decryptor.decrypt(&mut RefReadBuffer::new(&mut inbuf), &mut RefWriteBuffer::new(&mut outbuf),true).unwrap();

    println!("[+] Decrypted Message:\n\n{}",String::from_utf8(outbuf.to_vec()).unwrap().trim());
}
