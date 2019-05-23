use crate::*;

/// Gets an environment variable, asks for it if it didn't find it,
/// then returns a Box<io::Error> if the user input fails to be read.
pub fn get_env_var(var_name: &str) -> Result<String> {
    Ok(std::env::var(var_name)?)
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
