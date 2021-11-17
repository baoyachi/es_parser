use nom::error::ParseError;
use nom::{AsChar, IResult, InputTakeAtPosition};

pub fn key<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let x = item.as_char();
        !matches!(x, 'A'..='Z' | 'a'..='z' |'0'..='9'|'-'|'_')
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        fn key_demo(input: &str) -> IResult<&str, &str> {
            let x = key(input)?;
            Ok(x)
        }
        assert_eq!(Ok(("/tweet/1", "twitter")), key_demo("twitter/tweet/1"))
    }
}
