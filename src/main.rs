use dotenv::dotenv;
use reqwest::Response;
use std::env;
use v_name_generator::generate_valid_name;

async fn send_ntfy(name: String) -> Result<Response, reqwest::Error> {
    let ntfy = env::var("NTFY_POST_URL").unwrap_or_else(|_| {
        dotenv().expect("Couldnt load the .env");
        env::var("NTFY_POST_URL").expect("Couldnt find the NTFY_POST_URL anywhere")
    });

    reqwest::Client::new()
        .post(ntfy)
        .body(format!("The new name is: {}", name.to_owned()))
        .header("Title", "New V-Name Generated")
        .header("Tags", "rotating_light")
        .send()
        .await
}

#[tokio::main]
async fn main() {
    let name = generate_valid_name();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.get(1).expect("Error geting the 1 arg") == "ntfy" {
        let response = send_ntfy(name)
            .await
            .expect("Something wrong happened sending the ntfy");
        println!("ntfy sent correctly\nResponse code: {}", response.status());
    } else {
        println!("{}", name);
    }
}
