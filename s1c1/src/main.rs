extern crate hex;
extern crate base64;

fn main() {

    let hex_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_bytes = hex::decode(hex_string).unwrap();
    let b64_string = base64::encode(&hex_bytes);

    println!("-> encode_b64(decode_hex({}))",hex_string);
    println!("-> {}",b64_string);
}
