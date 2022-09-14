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

    let new_msg = models::Message {text: "this is the second new message".into()};
    let new_msg = reqwest::Client::new()
        .post(url)
        .json(&new_msg)
        .send()
        .await?;

    let res = reqwest::get(url).await?;
    let res_text = res.text().await?;

    dbg!(res_text);
    
    Ok(())
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





