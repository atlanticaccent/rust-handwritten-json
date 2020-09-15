mod error;
pub(crate) mod utilities;

pub use error::{Error, Result};

pub fn normalize(raw_input: &str) -> Result<String> {
    let input = raw_input.trim();
    if input.is_empty() {
        Ok(String::with_capacity(0))
    } else {
        let mut chars = input.chars().peekable();
        match chars.peek() {
            Some('{') => {
                let mut output = String::new();
                utilities::parse_object(&mut output, &mut chars)?;
                Ok(output)
            }
            Some('[') => {
                let mut output = String::new();
                utilities::parse_array(&mut output, &mut chars)?;
                Ok(output)
            }
            _ => Ok(input.to_owned()),
        }
    }
}
