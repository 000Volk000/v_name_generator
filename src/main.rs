mod name_functions;
use dotenv::dotenv;
use rand::Rng;
use reqwest::Response;
use std::env;

use crate::name_functions::{generate_random_name, is_valid_name};

async fn send_ntfy(name: String) -> Result<Response, reqwest::Error> {
    match env::var("NTFY_POST_URL") {
        Ok(_) => println!("The enviroment key is already on the coputer"),
        Err(_) => {
            dotenv().expect("Couldn load the .env");
        }
    }
    let ntfy =
        env::var("NTFY_POST_URL").expect("Couldn't find the NTFY_POST_URL on the enviroment");

    let response = reqwest::Client::new()
        .post(ntfy)
        .body(format!("The new name is: {}", name.to_owned()))
        .header("Title", "New V-Name Generated")
        .header("Tags", "rotating_light")
        .send()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let mut rng = rand::rng();
    let letter_number = 3 + (rng.random::<i8>() % 3).abs();
    let mut name = generate_random_name(letter_number, &mut rng);

    while !is_valid_name(name.clone()) {
        name = generate_random_name(letter_number, &mut rng);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.get(1).expect("Error geting the 1 arg") == "ntfy" {
        match send_ntfy(name).await {
            Ok(response) => println!("ntfy sent correctly\nResponse code: {}", response.status()),
            Err(e) => println!("Something wrong happened sending the ntfy\nError: {e}"),
        }
    } else {
        println!("{}", name);
    }
}
