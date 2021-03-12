use url_encoded_data::UrlEncodedData;

fn main() {
    // note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
    let url = "https://google.com/?q=rust&ei=code";
    let q = UrlEncodedData::from(url);
    // q.to_string(), best performance, (key, value) pairs are in un-deterministic order.
    assert_eq!(
        q.to_string_of_original_order(),
        "https://google.com/?q=rust&ei=code"
    );
    assert_eq!(
        q.to_string_of_sorted_order(),
        "https://google.com/?ei=code&q=rust"
    );

    // pairs length
    assert_eq!(q.len(), 2);

    // keys length
    assert_eq!(q.keys_length(), 2);

    // keys
    assert!(q.keys().contains(&"q"));
    assert!(q.keys().contains(&"ei"));

    // let's do some manipulation
    let url = "https://google.com/?q=rust&ei=code";
    let q = UrlEncodedData::parse_str(url)
        .set_one("q", "rust-lang")
        .set("vector", &vec!["1", "2"])
        .set_one("a", "1")
        .set_one("b", "2")
        .set_one("hello", "world")
        .set("whole", &vec!["world", "世界"]) // utf-8, auto encoding and decoding
        .delete("ei") // ei is deleted
        .push("b", "3")
        .done(); // now b is: vec!["1", "2"]
    let q = q; // mut -> const

    // q.keys() // performant
    assert_eq!(q.keys_of_original_order()[0].as_ref(), "q");

    // something like: https://google.com/?b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
    println!("{}", q.to_final_string());

    // https://google.com/?q=rust-lang&b=2&b=3&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
    println!("{}", q.to_string_of_original_order());

    // https://google.com/?a=1&b=2&b=3&hello=world&q=rust-lang&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
    println!("{}", q.to_string_of_sorted_order());
}
