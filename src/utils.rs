use log::debug;
use reqwest::blocking::get;
use reqwest::Error;

pub fn url_to_string(url: &str) -> Result<String, Error> {
    debug!("URL is {url}");
    let response = get(url)?;
    let status = response.status();
    debug!("Status is {status}");

    if response.status().is_success() {
        let body = response.text()?;
        debug!("Body is {body}");
        Ok(body)
    } else {
        Ok(String::new())
    }
}

pub fn string_to_2d_array(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_url_to_string() {
        let _m = mock("GET", "/")
            .with_status(200)
            .with_body("Hello, world!")
            .create();

        let result = url_to_string(&mockito::server_url()).unwrap();

        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn string_conversion() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let result = string_to_2d_array(input);
        let mut char_2d_array: Vec<Vec<char>> = Vec::new();
        char_2d_array.push(vec!['1', 'a', 'b', 'c', '2']);
        char_2d_array.push(vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x']);
        char_2d_array.push(vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f']);
        char_2d_array.push(vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't']);
        assert_eq!(result, char_2d_array);
    }
}
