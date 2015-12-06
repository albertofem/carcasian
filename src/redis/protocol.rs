use std::str;

const CLRF: &'static str = "\r\n";
const DOLLAR: &'static str = "$";

pub fn get_redis_command_from_human_command(command: &String) -> String {
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

    redis_command = "*".to_string()
        + &total_commands.to_string()
        + &CLRF
        + &redis_command;

    redis_command
}

pub fn get_human_command_from_redis_command(command: String) -> String {
    let tokens = command.split(CLRF);

    let mut human_command = "".to_string();

    for token in tokens {
        match token.chars().next().unwrap() {
            '*' => continue,
            '$' => continue,
            _ => human_command = human_command + token
        }
    }

    human_command
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_conversion_reciprocity() {
        let human_command = "SET test test";
        let redis_command = "*3\r\n$3\r\nSET\r\n$4\r\ntest\r\n$4\r\ntest\r\n";

        assert_eq!(get_redis_command_from_human_command(human_command.to_string()), redis_command.to_string());
        assert_eq!(get_human_command_from_redis_command(human_command.to_string()), human_command.to_string());
    }
}
