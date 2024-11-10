use std::collections::BTreeSet;
use regex::Regex;

fn clean_url(mut url: &str) -> &str {
    if url.contains('?') {
        url = &url[..url.find('?').unwrap()];
    }

    url
}

pub fn link_finder(body: &str) -> BTreeSet<String> {
    let regex = Regex::new(r"(http|ftp|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-])").unwrap();
    let mut link_array: BTreeSet<String> = BTreeSet::new();

    // Iterate over all matches and get the start and end positions for each
    for mat in regex.find_iter(body) {
        let (start, end) = (mat.start(), mat.end());

        let url: String = body[start..end].to_string();

        let clean = clean_url(&url);

        link_array.insert(clean.to_string());
    }

    link_array
}

