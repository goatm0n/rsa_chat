use std::collections::HashMap;
use std::error::Error;
use clap::Parser; 
use rsa_rs::encryption::encrypt::encrypt_string;
use rsa_rs::keys::keypair::*;
use rsa_rs::encryption::decrypt::decrypt_string;



#[derive(Parser)]
struct Cli {
    url: String,
}

struct Message {
    text: String,
}

impl Message {
    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn text(&self) -> String {
        self.text.clone()
    }

    fn encrypt(&self, public_key: &PublicKey) -> EncryptedMessage {
        let enc_vec = encrypt_string(&self.text, public_key);
        let e = public_key.public_exponent_clone();
        let n = public_key.modulus_clone();
        let key = PublicKey { public_exponent: e, modulus: n };
        EncryptedMessage { message: enc_vec, public_key :key }
    }
}

struct EncryptedMessage {
    message: Vec<u128>,
    public_key: PublicKey,
}

impl EncryptedMessage {
    fn from(message: Message, public_key: &PublicKey) -> EncryptedMessage {
        message.encrypt(public_key)
    }

    fn to_string(&self) -> String {
        vec_u128_to_string(&self.message)
    }
}

/// clear the terminal screen
fn cls() {
    print!("{}[2J", 27 as char);
}

fn display_tui(msg_list: &Vec<Message>) {
    //cls();
    for message in msg_list {
        let text = message.as_str();
        println!("{text}");
    }
}

fn read_input() -> String {
    let mut buf = String::new();
    println!("Enter to refresh: ");
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf;
}

fn write_encrypted_message_to_file(message: Message, public_key: &PublicKey, url: &String, path: &String) {
    let encrypted_message = message.encrypt(public_key);
    std::fs::write(path.as_str(), encrypted_message.to_string()).expect("Error writing to outfile.txt");
}

fn vec_u128_to_string(data: &Vec<u128>) -> String {
    let mut s = String::new();
    for num in data {
        let num_string = num.to_string();
        s += num_string.as_str();
        s += "\t";
    }
    return s;
}

fn post_encrypted_message(url: &String) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("outfile.txt").expect("could not open outfile.txt");

    let body = reqwest::blocking::Body::new(file);

    let client = reqwest::blocking::Client::new();
    
    let res = client.post(url).body(body).send()?; 
    dbg!(&res);
    Ok(())
}

fn get_data(url: &String) -> Vec<Vec<u128>> {
    let body = reqwest::blocking::get(url).expect("Error making get request")
        .text().expect("Error getting text");
    to_vec_vec_u128(&body)

}

fn to_vec_vec_u128(data: &String) -> Vec<Vec<u128>> {
    let mut vec_vec_u128: Vec<Vec<u128>> = Vec::new();
    let mut vec_string = String::new();
    for c in data.chars() {
        if c != '\n' {
            vec_string.push(c);
        } else {
            let vec_u128 = to_vec_u128(&vec_string);
            vec_vec_u128.push(vec_u128);
            vec_string.clear();

        }
    }
    return vec_vec_u128;
}

fn to_vec_u128(data: &String) -> Vec<u128> {
    let mut vec_u128: Vec<u128> = Vec::new();
    let mut num_string: String = String::new();
    for c in data.chars() {
        if c != '\t' {
            num_string.push(c);
        } else {
            let num: u128 = num_string.parse().expect("error parsing string to u128");
            vec_u128.push(num);
            num_string.clear();

        }
    }
    return vec_u128;
}

fn decrypt_data(data: &Vec<Vec<u128>>, private_key: &PrivateKey) -> Vec<String> {
    let mut decrypted_string_vec: Vec<String> = Vec::new();
    for enc_vec in data {
        let decrypted_string = decrypt_string(enc_vec, private_key);
        decrypted_string_vec.push(decrypted_string);
    }
    return decrypted_string_vec;
}

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let args = Cli::parse();
    let base_url = args.url;
    let post_url = base_url.clone() + "/post";
    let get_url = base_url.clone() + "/get";

    let outfile_path = String::from("outfile.txt");

    let key_pair = KeyPair::generate_key_pair(65537);
    
    let mut msg_list: Vec<Message> = Vec::new();
    
    let mut i = 0;
    while i < 5 {
        display_tui(&mut msg_list);
        let input_string = read_input();
        let message = Message { text: input_string };
        match message.as_str() {
            "\r\n" => continue,
            _ => write_encrypted_message_to_file(message, &key_pair.public_key(), &post_url, &outfile_path)
        }
        post_encrypted_message(&post_url).expect("Errpr posting encrypted message");

        //let incoming_enc_msg_list = get_data(&get_url);
        //msg_list = decrypt_data(&incoming_enc_msg_list, &key_pair.private_key());
        
        i += 1;
    }

    Ok(())
}


