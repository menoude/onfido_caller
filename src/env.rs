use std::error::Error;
use std::io;
use std::io::prelude::*;

/// Gets an environment variable, asks for it if it didn't find it,
/// then returns a Box<io::Error> if the user input fails to be read.
pub fn get_env_var(var_name: &str) -> Result<String, Box<Error>> {
    match std::env::var(var_name) {
        Ok(token) => Ok(token),
        Err(e) => {
            println!("{}", e);
            print!("Please enter {}: ", var_name);
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input)?;
            Ok(user_input.trim_end().to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_api_token() {
        let test_var_name = "TEST_VAR";
        let test_var_value = "testvalue";
        std::env::set_var(test_var_name, test_var_value);
        assert_eq!(get_env_var(test_var_name).unwrap(), test_var_value);
    }
}
