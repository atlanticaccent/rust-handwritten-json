use core::{iter::Peekable, str::Chars};

use crate::{Error, Result};

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while chars
        .peek()
        .map(|ch| ch.is_ascii_whitespace())
        .unwrap_or(false)
    {
        chars.next();
    }
}

fn next_should_be(output: &mut String, chars: &mut Peekable<Chars>, expected: char) -> Result<()> {
    if let Some(ch) = chars.next() {
        if ch != expected {
            return Err(Error::ShouldBe {
                actual: ch,
                expected,
            });
        }
        output.push(ch);
    }
    Ok(())
}

fn parse_string(output: &mut String, chars: &mut Peekable<Chars>, delimiter: char) -> Result<()> {
    next_should_be(output, chars, delimiter)?;
    let mut escape = false;
    let mut matched = false;
    for ch in chars {
        output.push(ch);
        if escape {
            escape = false;
        } else {
            match ch {
                '\\' => {
                    escape = true;
                }
                _ if ch == delimiter => {
                    matched = true;
                    break;
                }
                _ => {}
            }
        }
    }
    if matched {
        Ok(())
    } else {
        Err(Error::MissingDoubleQuote)
    }
}

fn parse_key(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    let mut is_empty = true;
    output.push('"');
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_alphanumeric() || *ch == '_' {
            output.push(*ch);
            chars.next();
            if is_empty {
                is_empty = false;
            }
        } else {
            break;
        }
    }
    if is_empty {
        Err(Error::MissingKey)
    } else {
        output.push('"');
        Ok(())
    }
}

fn parse_nonstring(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    let mut is_empty = true;
    let mut numeric = true;
    let mut temp_out = String::new();
    while let Some(ch) = chars.peek() {
        numeric = numeric && (ch.is_numeric() || *ch == '.');
        if ch.is_ascii_alphanumeric() || *ch == '+' || *ch == '-' || *ch == '.' {
            temp_out.push(*ch);
            chars.next();
            if is_empty {
                is_empty = false;
            }
        } else {
            break;
        }
    }
    if is_empty {
        Err(Error::MissingNonString)
    } else {
        if !numeric {
            output.push('"');
            output.push_str(temp_out.trim_end_matches(" "));
            output.push('"');
        } else {
            output.push_str(&temp_out);
        }
        Ok(())
    }
}

fn parse_key_value(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    if let Some(ch) = chars.peek().filter(|ch| **ch == '"' || **ch == '\'').cloned() {
        parse_string(output, chars, ch)?;
    } else {
        parse_key(output, chars)?;
    }
    skip_whitespace(chars);
    if chars.next().map(|ch| ch == ':').unwrap_or(false) {
        output.push(':');
        skip_whitespace(chars);
        parse_value(output, chars)
    } else {
        Err(Error::MissingColon)
    }
}

fn parse_value(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    if let Some(ch) = chars.peek().cloned() {
        match ch {
            '{' => parse_object(output, chars),
            '[' => parse_array(output, chars),
            '"' | '\'' => parse_string(output, chars, ch),
            _ => parse_nonstring(output, chars),
        }
    } else {
        Err(Error::MissingValue)
    }
}

pub(crate) fn parse_object(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    next_should_be(output, chars, '{')?;
    skip_whitespace(chars);
    if chars.peek().map(|ch| *ch == '}').unwrap_or(false) {
        output.push('}');
        chars.next();
        return Ok(());
    }
    parse_key_value(output, chars)?;
    skip_whitespace(chars);
    let mut matched = false;
    while let Some(ch) = chars.peek() {
        if *ch == ',' {
            chars.next();
            skip_whitespace(chars);
        }
        if chars.peek().map(|ch| *ch == '}').unwrap_or(false) {
            output.push('}');
            chars.next();
            matched = true;
            break;
        }
        output.push(',');
        skip_whitespace(chars);
        parse_key_value(output, chars)?;
        skip_whitespace(chars);
    }
    if matched {
        Ok(())
    } else {
        Err(Error::MissingRightBrace)
    }
}

pub(crate) fn parse_array(output: &mut String, chars: &mut Peekable<Chars>) -> Result<()> {
    next_should_be(output, chars, '[')?;
    skip_whitespace(chars);
    if chars.peek().map(|ch| *ch == ']').unwrap_or(false) {
        output.push(']');
        chars.next();
        return Ok(());
    }
    parse_value(output, chars)?;
    skip_whitespace(chars);
    let mut matched = false;
    while let Some(ch) = chars.peek() {
        if *ch == ',' {
            chars.next();
            skip_whitespace(chars);
        }
        if chars.peek().map(|ch| *ch == ']').unwrap_or(false) {
            output.push(']');
            chars.next();
            matched = true;
            break;
        }
        output.push(',');
        skip_whitespace(chars);
        parse_value(output, chars)?;
        skip_whitespace(chars);
    }
    if matched {
        Ok(())
    } else {
        Err(Error::MissingRightBracket)
    }
}
