extern crate hex;

fn encrypt(pt: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let k: Vec<u8> = key.to_vec();
    let klen = k.len();
    let mut kindex = 0;
    let mut crypted = Vec::<u8>::new();

    for i in 0..pt.len() {
        crypted.push(pt[i] ^ k[kindex]);
        println!("Plaintext Pos:\t{} - Key Pos:\t{}",i,kindex);
        kindex += 1;
        kindex = kindex % klen;
    }
    crypted
}

fn main() {

    let str1: Vec<u8> = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".bytes().collect();

    let key: Vec<u8> = "ICE".bytes().collect();

    let crypted: String = hex::encode(&encrypt(&str1, &key));

    println!("-> {}",crypted);
}
