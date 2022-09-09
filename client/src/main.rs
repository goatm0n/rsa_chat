use clap::Parser; 
use rsa_rs::encryption::encrypt::encrypt_string;
use rsa_rs::keys::keypair::*;
use rsa_rs::encryption::decrypt::decrypt_string;



#[derive(Parser)]
struct Cli {
    url: String,
}

/// clear the terminal screen
fn cls() {
    print!("{}[2J", 27 as char);
}

fn display_tui(msg_list: &mut Vec<String>) {
    cls();
    for message in msg_list {
        dbg!(message);
    }
}

fn read_input(buf: &mut String) {
    println!("Enter to refresh: ");
    std::io::stdin().read_line(buf).unwrap();
}

fn handle_message(message: &String, public_key: &PublicKey, enc_msg_list: &mut Vec<Vec<u128>>) {
    let enc_vec = encrypt_string(&message, public_key);
    enc_msg_list.push(enc_vec);
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

fn post_data(url: &String, data: &Vec<Vec<u128>>) {
    let data = data.last().expect("Error extracting data");
    let data = vec_u128_to_string(data);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .body(data)
        .send().expect("Failed to post data");
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

fn main() {
    let args = Cli::parse();
    let base_url = args.url;
    let post_url = base_url.clone() + "/post";
    let get_url = base_url.clone() + "/get";

    let key_pair = KeyPair::generate_key_pair(65537);
    
    let mut message = String::new();
    let mut msg_list: Vec<String> = Vec::new();
    let mut enc_msg_list: Vec<Vec<u128>> = Vec::new();

    loop {
        display_tui(&mut msg_list);
        read_input(&mut message);
        match message.as_str() {
            "\r\n" => continue,
            _ => handle_message(&message, &key_pair.public_key(), &mut enc_msg_list)
        }
        post_data(&post_url, &enc_msg_list);
        let incoming_enc_msg_list = get_data(&get_url);
        msg_list = decrypt_data(&incoming_enc_msg_list, &key_pair.private_key());
    }
}


