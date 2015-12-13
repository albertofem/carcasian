use database::storage::Storage;
use redis::protocol;
use std::sync::{Arc, Mutex};

pub fn handle_command(tcp_message: String, data: Arc<Mutex<Storage>>) -> String {
    let command = protocol::get_human_command_from_redis_command(tcp_message);

    println!("{}", command);

    let words: Vec<&str> = command.split(" ").collect();

    let mut data = data.lock().unwrap();

    let command = words[0];

    match command {
        "SET" => {
            let response = data.set(words[1].to_string(), words[2].to_string());
            match response {
                _ => protocol::get_ok_response()
            }
        },

        "GET" => {
            let data = data.get(words[1].to_string());

            match data {
                Ok(value) => protocol::get_bulk_string_response(value),
                Err(_) => protocol::get_nil_response(),
            }
        },

        "DEL" => {
            let response = match data.del(words[1].to_string()) {
                Ok(true) => 1,
                _ => 0
            };

            protocol::get_int_response(response)
        },

        "SADD" => {
            let response = data.sadd(words[1].to_string(), words[2].to_string());

            match response {
                Ok(d) => protocol::get_int_response(d as i32),
                Err(_) => protocol::get_int_response(0)
            }
        },

        "SREM" => {
            let response = data.srem(words[1].to_string(), words[2].to_string());

            match response {
                Ok(d) => protocol::get_int_response(d as i32),
                Err(_) => protocol::get_err_response("Nope")
            }
        },

        "SISMEMBER" => {
            let response = data.sismember(words[1].to_string(), words[2].to_string());

            match response {
                Ok(d) => protocol::get_int_response(d as i32),
                Err(_) => protocol::get_err_response("Nope")
            }
        },

        "SMEMBERS" => {
            let response = data.smembers(words[1].to_string());

            match response {
                Ok(ref set) => protocol::get_array_response(set),
                Err(_) => protocol::get_err_response("Nope")
            }
        },

        "QUIT" => {
            protocol::get_ok_response()
        }

        _ => {
            protocol::get_err_response("Invalid command")
        }
    }
}