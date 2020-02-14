fn pad_buffer(bytes: &mut Vec<u8>, block_size: usize) {

    let mut pad_byte: u8 = 0x00;
    let padding_count = block_size - ( bytes.len() % block_size as usize );

    for _i in 0..padding_count {
        pad_byte += 0x01;
    }
    for _ii in 0..padding_count {
        bytes.push(pad_byte);
    }
}

fn main() {
    let msg = "YELLOW SUBMARINE";
    let mut msg_bytes = msg.as_bytes().to_vec();
    pad_buffer(&mut msg_bytes, 4);

    print!("[+] Padded Input: ");
    for byte in msg_bytes {
        print!("{:02X} ",byte);
    }
    println!("");
}
