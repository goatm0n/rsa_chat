use clap::Parser;
use std::path::PathBuf;
use rsa_utils::io::{get_full_path, parse_key_file};

#[derive(Parser)]
struct Cli {
    url: String,
    key_path: PathBuf, 
}

impl Cli {
    fn key_path(&self) -> PathBuf {
        get_full_path(&self.key_path)
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // args 
    let args = Cli::parse();
    let key_path = args.key_path();
    let key_pair = parse_key_file(key_path);
    let base_url = args.url;
    let url = base_url + "/messages";
    
    loop {
        // io
        println!("/q to quit");
        let mut input = tui::read_input();
        input.pop();
        input.pop();
        if input == "/q" {
            break; 
        }
        let msg = models::Message {text: input};
        //
        
        if !msg.text.is_empty() {
            // encrypt
            let public_key = key_pair.public_key();
            let msg = msg.encrypt(public_key);
            //

            // post 
            let res = handlers::post_msg(&url, &msg).await?;
            let _res_status = res.status(); // some error handling to be done
            //
        }

        // get 
        let res = handlers::get_msg_list(&url).await?;
        let res_text = res.text().await?;
        let msg_list = models::MessageList::from_string(&res_text);
        //
    
        // decrypt
        let private_key = key_pair.private_key();
        let msg_list = msg_list.decrypt(private_key);
        //
        
        // display
        tui::cls();
        for msg in msg_list.items.iter() {
            println!("message: {}", &msg.text);
        }
        //
    }
    Ok(())
}

mod handlers {
    use reqwest::Response;
    use super::models::Message;
    
    pub async fn get_msg_list(url: &String) -> Result<Response, reqwest::Error> {
        let res = reqwest::get(url)
            .await?;
        Ok(res)
    }

    pub async fn post_msg(url: &String, msg: &Message) -> Result<Response, reqwest::Error> {
        let new_msg = reqwest::Client::new()
            .post(url)
            .json(msg)
            .send()
            .await?;
        Ok(new_msg)
    }
}

mod convert {
    
    pub fn vec_u128_to_string(data: &Vec<u128>) -> String {
        let mut s = String::new();
        for num in data {
            let num_string = num.to_string();
            s += num_string.as_str();
            s += "\t";
        }
        return s;
    }

    pub fn to_vec_u128(data: &String) -> Vec<u128> {
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
}


mod models {
    use rsa_rs::encryption::encrypt;
    use rsa_rs::encryption::decrypt;
    use rsa_rs::keys::keypair::{PublicKey, PrivateKey};
    use serde::{Deserialize, Serialize};
    use super::convert;


    #[derive(Debug, Deserialize, Serialize)]
    pub struct Message {
        pub text: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct EncryptedMessage {
        pub message: Vec<u128>,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    pub struct MessageList {
        pub items: Vec<Message>,
    }

    impl Message {
        pub fn encrypted_message(&self, public_key: &PublicKey) -> EncryptedMessage {
            EncryptedMessage { message: encrypt::encrypt_string(&self.text, public_key) }
        }
        pub fn encrypt(&self, public_key: &PublicKey) -> Message {
            let enc_msg = self.encrypted_message(public_key);
            enc_msg.to_message()
        }
        pub fn decrypt(&self, private_key: &PrivateKey) -> Message {
            let enc_vec = convert::to_vec_u128(&self.text); 
            let decrypted_string = decrypt::decrypt_string(&enc_vec, private_key);
            Message { text: decrypted_string }
        }
    }

    impl EncryptedMessage {
        pub fn to_string(&self) -> String {
            convert::vec_u128_to_string(&self.message)
        }
        pub fn to_message(&self) -> Message {
            let message_string = self.to_string();
            Message { text: message_string }
        }
    }

    impl MessageList {
        pub fn from_string(s: &String) -> MessageList {
            let mut msg_json_string = String::new();
            let mut msg_list: Vec<Message> = Vec::new();
            for c in s.chars() {
                match c {
                    '[' => continue,
                    ']' => continue,
                    ',' => continue,
                    '}' => {
                        msg_json_string.push(c);
                        //let msg = Message {text: msg_json_string.clone()};
                        let msg_json_string_clone = msg_json_string.clone();
                        let msg_json_str = msg_json_string_clone.as_str();
                        let msg: Message = serde_json::from_str(msg_json_str).unwrap();
                        msg_list.push(msg);
                        msg_json_string.clear();
                    },
                    _ => msg_json_string.push(c),
                }
            }
            MessageList {items: msg_list}
        }

        pub fn decrypt(&self, private_key: &PrivateKey) -> MessageList {
            let mut msg_list: Vec<Message> = Vec::new();
            for msg in self.items.iter() {
                let msg = msg.decrypt(private_key);
                msg_list.push(msg);
            }
            MessageList {items: msg_list}
        }
    }
}


mod tui {

    /// clear the terminal screen
    pub fn cls() {
        print!("{}[2J", 27 as char);
    }

    pub fn read_input() -> String {
        let mut buf = String::new();
        println!("Enter to refresh: ");
        std::io::stdin().read_line(&mut buf).unwrap();
        return buf;
    }
}





