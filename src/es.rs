use crate::{decomposition, key, slash};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum ReqType {
    Search,
    SearchTemplate,
    MultiSearch,
    MultiSearchTemplate,
    MGet,
    IndexApi,
    IndexBlockApi,
    IndexTemplate,
    DeleteByQuery,
    UpdateByQuery,
    Tasks,
    Bulk,
    Ingest,
    Refresh,
    Reindex,
    Termvectors,
    MTermvectors,
    SearchShards,
    ScriptsTemplate,
    Validate,
    Explain,
    FieldStats,
    FieldCaps,
    Cat,
    Cluster,
    Nodes,
    Remote,
    Analyze,
    Xpark,
    Security,
    Watcher,
    Migration,
    All,
    AllSettings,
    Template,
    Other,
    None
}

fn req_type(input: &str) -> IResult<Vec<String>, ReqType> {
    let (out, func) = decomposition(input);

    let req_type = if let Some(value) = func {
        match value.as_ref() {
            "_search" => ReqType::Search,
            "_msearch" => ReqType::MultiSearch,
            "_template" => ReqType::Template,
            "_tasks" => ReqType::Tasks,
            "_ingest" => ReqType::Ingest,
            "_refresh" => ReqType::Refresh,
            "_reindex" => ReqType::Reindex,
            "_scripts" => ReqType::ScriptsTemplate,
            "_field_stats" => ReqType::FieldStats,
            "_field_caps" => ReqType::FieldCaps,
            "_all" => ReqType::All,
            "_block" => ReqType::IndexBlockApi,
            "_bulk" => ReqType::Bulk,
            "_mget" => ReqType::MGet,
            "_delete_by_query" => ReqType::DeleteByQuery,
            "_update_by_query" => ReqType::UpdateByQuery,
            "_termvectors" => ReqType::Termvectors,
            "_mtermvectors" => ReqType::MTermvectors,
            "_search_shards" => ReqType::SearchShards,
            "_explain" => ReqType::Explain,
            "_cat" => ReqType::Cat,
            "_cluster" => ReqType::Cluster,
            "_nodes" => ReqType::Nodes,
            "_remote" => ReqType::Remote,
            "_analyze" => ReqType::Analyze,
            "_xpack" => ReqType::Xpark,
            _ => ReqType::IndexApi,
        }
    }else {
        ReqType::None
    };

    Ok((out, req_type))
}


// search_url = {("/_search"|(virgule* ~ index* ~ "/_search"))}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_req_type() {
        let x = req_type("/_search").unwrap();
        assert_eq!((vec!["/_search".to_string()], ReqType::Search), x)
    }
}
