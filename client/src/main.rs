use clap::Parser;
 
#[derive(Parser)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();
    let base_url = args.url;
    let url = base_url + "/messages";

    let msg = models::Message {text: "this is a new message".into()};
    let res = handlers::post_msg(&url, &msg).await?;
    let res_status = res.status();
    dbg!(res_status);
   
    let res = handlers::get_msg_list(&url).await?;
    let res_text = res.text().await?;
    let msg_list = models::MessageList::from_string(&res_text);
    dbg!(msg_list);
    
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

    pub fn to_vec_vec_u128(data: &String) -> Vec<Vec<u128>> {
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
    use rsa_rs::encryption::encrypt::encrypt_string;
    use rsa_rs::encryption::decrypt::decrypt_string;
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
        pub fn encrypt(&self, public_key: &PublicKey) -> EncryptedMessage {
            EncryptedMessage { message: encrypt_string(&self.text, public_key) }
        }
    }

    impl EncryptedMessage {
        pub fn decrypt(&self, private_key: &PrivateKey) -> Message {
            Message { text: decrypt_string(&self.message, private_key) }
        }
        pub fn to_string(&self) -> String {
            convert::vec_u128_to_string(&self.message)
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
                        let msg = Message {text: msg_json_string.clone()};
                        msg_list.push(msg);
                        msg_json_string.clear();
                    },
                    _ => msg_json_string.push(c),
                }
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





