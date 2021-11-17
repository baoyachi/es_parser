use crate::{key, slash};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum ReqType {
    IndexAPI,
    Search,
}

fn req_type(input: &str) -> IResult<&str, ReqType> {
    let (input, (_, out)) = tuple((slash, key))(input)?;
    let req_type = match out {
        //TODO replace with from_str
        "_msearch" => ReqType::Search,
        _ => ReqType::IndexAPI,
    };
    Ok((input, req_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_req_type() {
        let x = req_type("_msearch").unwrap();
        assert_eq!(("", ReqType::Search), x)
    }
}
