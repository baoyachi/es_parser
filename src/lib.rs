mod es;

use nom::error::ParseError;
use nom::{AsChar, IResult, InputTakeAtPosition};

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
        assert_eq!(Ok(("", "/")), slash_fn("/"))
    }
}
