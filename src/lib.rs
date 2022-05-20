mod es;

use nom::error::ParseError;
use nom::{AsChar, IResult, InputTakeAtPosition, InputTake, FindSubstring};
use nom::bytes::complete::{take_till, take_until};

pub fn key<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(
        |item| !matches!(item.as_char(), 'A'..='Z' | 'a'..='z' |'0'..='9'|'-'|'_'),
    )
}

pub fn slash<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| !matches!(item.as_char(), '/'))
}

pub fn decomposition(input: &str) -> (Vec<String>, Option<String>){
    if input.eq("/") {
        return (vec!["/".to_string()], None)
    }

    let mut func = None;
    let result: Vec<&str> = input.split('/').collect();
    let mut values = Vec::new();
    for value in result {
        if value.is_empty() {
            continue;
        }

        if func.is_none() && value.starts_with("_"){
            func = Some(value.to_string());
        }

        values.push(format!("/{}", value));
    }
    (values, func)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        fn key_fn(input: &str) -> IResult<&str, &str> {
            let x = key(input)?;
            Ok(x)
        }
        assert_eq!(Ok(("/tweet/1", "twitter")), key_fn("twitter/tweet/1"))
    }

    #[test]
    fn test_slash() {
        fn slash_fn(input: &str) -> IResult<&str, &str> {
            let x = slash(input)?;
            Ok(x)
        }
        assert_eq!(Ok(("", "/")), slash_fn("/"));
    }

    #[test]
    fn test_decomposition() {
        assert_eq!(vec!["/"], decomposition("/").0);

        assert_eq!(vec!["/_search"], decomposition("/_search").0);

        assert_eq!(vec!["/incident", "/_search"], decomposition("/incident/_search").0);
    }
}
