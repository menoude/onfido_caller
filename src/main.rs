use std::process;

use onfido_caller::*;

fn main() {
    let api_token = env::get_env_var("API_TOKEN").unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        eprintln!("Please set up your api token with \"export API_TOKEN=<your token>\"");
        process::exit(1);
    });
    if let Err(e) = run(&api_token) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
