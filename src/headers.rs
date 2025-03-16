use reqwest::header::{HeaderMap, HeaderValue};

pub fn get_headers() -> Result<HeaderMap, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:136.0) Gecko/20100101 Firefox/136.0",
        ),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://www.google.com/"),
    );
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("cross-site"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    Ok(headers)
}
