extern crate postgres;
extern crate reqwest;
extern crate scraper;

fn max_pages() -> u64 {
    const URL: &str = "https://dbase.tube/chart/channels/subscribers/all";
    let document = reqwest::get(URL).unwrap().text().unwrap();

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

fn main() {
    /*let params: &str = "postgres://postgres@192.168.1.63:30000/youtube";
    let query: &str = "SELECT id FROM youtube.entities.channels";
    let tls = postgres::TlsMode::None;

    let conn: postgres::Connection = postgres::Connection::connect(params, tls).unwrap();*/

    println!("{}", max_pages());
}
