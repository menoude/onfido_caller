use std::process;

use onfido_caller::*;

fn main() {
    let api_token = env::get_env_var("API_TOKEN").unwrap_or_else(|e| {
        eprintln!("couldn't find the API token: {}", e);
        process::exit(1);
    });
    if let Err(_) = run(&api_token) {
        process::exit(1);
    }
}
