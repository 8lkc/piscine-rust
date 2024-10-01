pub enum Security {Unknown, High, Medium, Low, BlockServer}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {server.expect("")}
        Security::High => {server.unwrap_or("ERROR: program stops".to_string())}
        Security::Medium => {server.unwrap_or("WARNING: check the server".to_string())}
        Security::Low => {server.unwrap_or_else(|err| format!("Not found: {}", err))}
        Security::BlockServer => {
            match server {
                Ok(url) => panic!("{}", url),
                Err(err) => err,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result;
        result = fetch_data(Ok("server1.com".to_string()), Security::Medium); assert_eq!(result, "server1.com");
        result = fetch_data(Err(String::new()), Security::Medium); assert_eq!(result, "WARNING: check the server");
        result = fetch_data(Err("server2.com".to_string()), Security::Low); assert_eq!(result, "Not found: server2.com");
    }
}
