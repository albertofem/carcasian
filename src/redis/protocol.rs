use std::collections::HashSet;

const CLRF: &'static str = "\r\n";
const DOLLAR: &'static str = "$";
const ASTERISK: &'static str = "*";

/// This function will convert from a Redis protocol
/// formatted command to a human-friendly representation
pub fn get_redis_command_from_human_command(command: String) -> String {
    let words = command.split(" ");

    let mut total_commands = 0;
    let mut redis_command = "".to_string();

    for token in words {
        total_commands += 1;
        let token_length = token.trim().len();

        redis_command = redis_command
            + &DOLLAR
            + &token_length.to_string()
            + &CLRF
            + token
            + &CLRF;
    }

    redis_command = "".to_string()
        + &ASTERISK
        + &total_commands.to_string()
        + &CLRF
        + &redis_command;

    redis_command
}

/// This will convert from a human-friendly representation
/// of a Redis command to the Redis protocol specification (so
/// it can be read by the server)
pub fn get_human_command_from_redis_command(command: String) -> String {
    let tokens = command.split(CLRF);

    let mut human_command = "".to_string();

    for token in tokens {
        match token.chars().next().unwrap() {
            '*' => continue,
            '$' => continue,
            _ => human_command = human_command + token + " "
        }
    }

    human_command.trim().to_string()
}

pub fn get_bulk_string_response(response: &String) -> String {
    let response = "$".to_string()
        + &response.len().to_string()
        + &CLRF
        + response
        + &CLRF;

    response
}

pub fn get_array_response(set: &HashSet<String>) -> String {
    let mut response = "*".to_string() + &set.len().to_string() + &CLRF;

    for element in set {
        response = response
            + &DOLLAR
            + &element.len().to_string()
            + &CLRF
            + element
            + &CLRF;
    }

    response
}

pub fn get_ok_response() -> String {
    "+OK".to_string() + &CLRF
}

pub fn get_nil_response() -> String {
    "$-1".to_string() + &CLRF
}

pub fn get_int_response(int: i32) -> String {
    ":".to_string() + &int.to_string() + &CLRF
}

pub fn get_err_response(message: &str) -> String {
    "-".to_string() + message + &CLRF
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_conversion_reciprocity() {
        let human_command = "LLEN mylist";
        let redis_command = "*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n";

        assert_eq!(get_redis_command_from_human_command(human_command.to_string()), redis_command.to_string());
        assert_eq!(get_human_command_from_redis_command(human_command.to_string()), human_command.to_string());
    }

    #[test]
    fn test_get_bulk_string_response() {
        let data = "test_data";
        let expected_response = "$9\r\ntest_data\r\n";

        assert_eq!(get_bulk_string_response(&data.to_string()), expected_response.to_string());
    }

    #[test]
    fn test_get_ok_response() {
        let expected_response = "+OK\r\n";

        assert_eq!(get_ok_response(), expected_response.to_string());
    }

    #[test]
    fn test_get_array_response() {
        let mut set = HashSet::new();

        set.insert("test".to_string());

        let expected_response = "*1\r\n$4\r\ntest\r\n";

        assert_eq!(get_array_response(&set), expected_response.to_string());
    }

    #[test]
    fn test_get_nil_response() {
        let expected_response = "$-1\r\n";

        assert_eq!(get_nil_response(), expected_response.to_string());
    }

    #[test]
    fn test_get_int_response() {
        let expected_response = ":123\r\n";

        assert_eq!(get_int_response(123), expected_response.to_string());
    }

    #[test]
    fn test_get_err_response() {
        let expected_response = "-ERR Whatever error!\r\n";

        assert_eq!(get_err_response("ERR Whatever error!"), expected_response.to_string());
    }
}
