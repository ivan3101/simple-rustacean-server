use super::http_verbs::HttpVerbs;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpVerbs,
}