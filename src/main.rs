extern crate postgres;
extern crate rand;
extern crate reqwest;
extern crate scraper;

fn max_pages() -> u64 {
    let url: &str = "https://dbase.tube/chart/channels/subscribers/all";
    let document = reqwest::get(url).unwrap().text().unwrap();

    let doc = scraper::Html::parse_document(document.as_ref());
    let select_str = "a[href^=\"/chart/channels/subscribers/all?page=\"]";

    let selector = scraper::Selector::parse(select_str).unwrap();
    let mut buttons = doc.select(&selector);
    buttons.next();
    let frag = buttons.next().unwrap().value().attr("href").unwrap();

    let idx = frag.rfind("=").unwrap();
    let raw_str: String = frag.chars().skip(idx + 1).collect();
    return raw_str.parse::<u64>().unwrap();
}

fn channel(page_idx: u64) -> Vec<String> {
    let url: String = if page_idx == 0 || page_idx == 1 {
        format!("https://dbase.tube/chart/channels/subscribers/all")
    } else {
        format!("https://dbase.tube/chart/channels/subscribers/all?page={}", page_idx)
    };

    let url_ref = url.as_str();
    let document = reqwest::get(url_ref).unwrap().text().unwrap();
    let mut vec = Vec::new();

    let doc = scraper::Html::parse_document(document.as_ref());
    let select_str = "a[href^=\"/c/\"]";
    let selector = scraper::Selector::parse(select_str).unwrap();

    for element in doc.select(&selector) {
        let href = element.value().attr("href").unwrap();
        let chan = href.chars().skip(3).take(24).collect();

        vec.push(chan);
    }
    return vec;
}

fn main() {
    loop {
        let params: &str = "postgres://postgres@localhost:30000/youtube";
        let query: &str = "SELECT id FROM youtube.entities.channels";
        let tls = postgres::TlsMode::None;
        let conn: postgres::Connection = postgres::Connection::connect(params, tls).unwrap();

        let max = max_pages();
        let mut nums: Vec<u64> = (0..max).collect();
        let slice: &mut [u64] = &mut nums;
        use rand::Rng;
        rand::thread_rng().shuffle(slice);

        for i in nums {
            println!("On page {}", i);
            for c in channel(i) {
                let query = "INSERT INTO youtube.entities.channels (serial) VALUES ($1) ON CONFLICT (serial) DO NOTHING";

                let trans = conn.transaction().unwrap();
                let stmt = trans.prepare(query).unwrap();
                stmt.execute(&[&c]);
            }
        }
    }
}
