use url::Url;

pub fn get_rating_key_from_url(url: &str) -> String {
    let parsed_url = Url::parse(url).expect("Unable to parse url");
    let server_fragment = parsed_url.fragment().unwrap();
    let server_url = parsed_url.join(server_fragment).expect("Unable to join url");
    server_url.query_pairs().next().unwrap().1.to_string()
}
