# url_encoded_data

* crate: https://crates.io/crates/url_encoded_data
* doc: https://crates.io/crates/url_encoded_data
* test coverage: 100% lines covered(2021-03-12)

## Ergonomic, Versatile Url-Encoded-Data Manipulator
Manipulate data of `application/x-www-form-urlencoded` format,
eg:
    * query_string of a url (eg: '?a=1&b=2&c=3&c=3&e=5')
    * http content-type with: `application/x-www-form-urlencoded`

## Features:
* convenient api:
    * as_pairs
    * as_pairs_of_original_order
    * as_paris_of_sorted_order

    * as_map_of_single_key_to_multiple_values
    * as_map_of_single_key_to_first_occurrence_value
    * as_map_of_single_key_to_last_occurrence_value

    * set
    * push
    * clear

    * get
    * get_first
    * get_last

    * keys
    * len // pair length
    * keys_length
    * to_string (to_final_string), same to: `format!("{}", self)`
    * exists

    * // consult doc for more

* Automatic unicode encoding/decoding


## Terminology
* Pair: a (key, format) tuple, `(Cow<'a, str>, Cow<'a, str>)`
* url encoded string: a string which is encoded by standards of `application/x-www-form-urlencoded`

## Notes
* UrlEncodedDataPairScanner: Pairs Iterator, yields pairs only. (high performant)
* UrlEncodedData: eager version

## Sample
### Sample of url query string
```rust
use url_encoded_data::UrlEncodedData;
use std::borrow::Cow;
// note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
let url = "https://google.com/?q=rust&ei=code";
let q = UrlEncodedData::from(url);
// q.to_string(), best performance, (key, value) pairs are in un-deterministic order.
assert_eq!(q.to_string_of_original_order(), "https://google.com/?q=rust&ei=code");
assert_eq!(q.to_string_of_sorted_order(), "https://google.com/?ei=code&q=rust");

// pairs length
assert_eq!(q.len(), 2);

// keys length
assert_eq!(q.keys_length(), 2);

// keys
assert!(q.keys().contains(&"q"));
assert!(q.keys().contains(&"ei"));

// exists
assert!(q.exists("q"));
assert!(q.exists("ei"));


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

// q.keys() // performant
assert_eq!(q.keys_of_original_order()[0].as_ref(), "q");

// something like: https://google.com/?b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
println!("{}", q); // calls q.to_final_string() actually.

// something like: https://google.com/?b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
println!("{}", q.to_final_string());

// https://google.com/?q=rust-lang&b=2&b=3&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
println!("{}", q.to_string_of_original_order());

// https://google.com/?a=1&b=2&b=3&hello=world&q=rust-lang&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
println!("{}", q.to_string_of_sorted_order());

```
### Sample of encoded data in www/x-www-form-urlencoded
```rust
use url_encoded_data::UrlEncodedData;
use std::borrow::Cow;
// note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
let s = "b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C";
let q = UrlEncodedData::parse_str(s);
// q.to_string(), best performance, (key, value) pairs are in un-deterministic order.
assert_eq!(q.to_string_of_original_order(), s);

// [("hello", "world"), ("vector", "1"), ("vector", "2"), ("whole", "world"), ("whole", "世界"), ("b", "2"), ("b", "3"), ("q", "rust-lang"), ("a", "1")]
println!("{:?}", q.as_pairs());

// {"a": ["1"], "hello": ["world"], "b": ["2", "3"], "q": ["rust-lang"], "whole": ["world", "世界"], "vector": ["1", "2"]}
println!("{:?}", q.as_map_of_single_key_to_multiple_values());

// {"b": "2", "a": "1", "q": "rust-lang", "whole": "world", "hello": "world", "vector": "1"}
println!("{:?}", q.as_map_of_single_key_to_first_occurrence_value());

// {"q": "rust-lang", "whole": "世界", "vector": "2", "a": "1", "b": "3", "hello": "world"}
println!("{:?}", q.as_map_of_single_key_to_last_occurrence_value());
// assert!(false);

```

### Sample of performant pairs iterator: UrlEncodedDataPairScanner (Lazy version)
```rust
use url_encoded_data::{UrlEncodedData, UrlEncodedDataPairScanner};
use std::borrow::Cow;
// note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
let s = "b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C";
let q = UrlEncodedDataPairScanner::from(s);
// same:
// let q = UrlEncodedDataPairScanner::parse_from_data_str(s);

for (key, value) in q.iter() {
    // k, v are decoded
    // process the pair: (key, value)
}

```

###  Some apis

#### strigify: Stringify pairs to url encoded String

##### example 1
```rust
use url_encoded_data::stringify;
let encoded = stringify(&[("a", "b"), ("c", "d")]);
assert_eq!(encoded, "a=b&c=d");
```

##### example 2
```rust
use url_encoded_data::stringify;
let encoded = stringify(&[("hello", "你好"), ("world", "世界")]);
assert_eq!(encoded, "hello=%E4%BD%A0%E5%A5%BD&world=%E4%B8%96%E7%95%8C");
```


#### UrlEncodedDataPairScanner: **Lazy** iterator yielding pairs only, performant when you only needs pairs in sequence.
##### example:

```rust
use url_encoded_data::*;
let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
for s in [
    qs.as_str(),
    ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
]
.iter()
{
    let q = UrlEncodedDataPairScanner::from(*s);
    println!("got qs: {}", q);

    let pairs_expected_as_str = [
        ("a", "1"),
        ("b", "2"),
        ("c", "3"),
        ("c", "4"),
        ("key_without_value", ""),
        ("", "value_without_key"),
    ];


    for (i, (k, v)) in q.iter().enumerate() {
        let (k_, v_) = pairs_expected_as_str[i];
        assert_eq!(k.as_ref(), k_);
        assert_eq!(v.as_ref(), v_);
    }
}
```

#### UrlEncodedData: parse url_encoded_data to pairs eagerly
##### some methods:
> for string: "a=1&b=2&a=3"
* as_pairs: ["a", "1"], ["b", "2"], ["c", "3"]
* as_map_of_single_key_to_multiple_values: {"a": ["1", "3"], "b": ["2"]}
* as_map_of_single_key_to_first_occurrence_value: {"a": "1", "b": "2"}
* as_map_of_single_key_to_last_occurrence_value: {"a": "3", "b": "2"}

> get shortcuts
> note: for multiple get, use the result of map mehtods directly
* get_multiple_values: "a" -> vec!["1".to_string(), "3".to_string()]
* get_first_occurrence_value: "a" -> "1".to_string()
* get_last_occurrence_value: "a" -> "3".to_string()


Typical usage might be:

```rust
use url_encoded_data::*;
use std::borrow::Cow;
let s = "a=1&b=2&a=3";
let ued = UrlEncodedData::from(s);

// get pairs
let pairs = ued.as_pairs();

// 1:N
let map_n = ued.as_map_of_single_key_to_multiple_values();
let a = map_n.get(&Cow::from("a")).unwrap();
assert_eq!(a[0].as_ref(), "1");
assert_eq!(a[1].as_ref(), "3");

// 1:first-value-met
let map_f = ued.as_map_of_single_key_to_first_occurrence_value();
let a = map_f.get(&Cow::from("a")).unwrap();
assert_eq!(a.as_ref(), "1");

// 1:last-value-met
let map_l = ued.as_map_of_single_key_to_last_occurrence_value();
let a = map_l.get(&Cow::from("a")).unwrap();
assert_eq!(a.as_ref(), "3");


```

One time get(For best performance of multiple callings, use the result of method calling of as_map_*)
```rust
use url_encoded_data::*;
use std::borrow::Cow;
let s = "a=1&b=2&a=3";
let ued = UrlEncodedData::from(s);

assert_eq!(ued.get_multiple_values("a").unwrap().iter().map(|x| x.as_ref()).collect::<Vec<_>>(), vec!["1", "3"]);

// get first occurrence value
assert_eq!(ued.get_first_occurrence_value("a").unwrap().as_ref(), "1");

// get last occurrence value
assert_eq!(ued.get_last_occurrence_value("a").unwrap().as_ref(), "3");

// no existed key
assert!(ued.get_last_occurrence_value("not-existed-key").is_none());
```

#### full example

```rust
#[macro_use]
extern crate maplit;
use url_encoded_data::*;

fn main() {
    use std::borrow::Cow;
use url_encoded_data::UrlEncodedData;
let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    for s in [
        qs.as_str(),
        ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
        ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    ]
        .iter()
    {
        let q = UrlEncodedData::parse_str(s);
        // let mut q = UrlEncodedData::prepare(url_1);
        // let q = q.parse();
        println!("got qs: {}", q);

        let pairs_expected_as_str = [
            ("a", "1"),
            ("b", "2"),
            ("c", "3"),
            ("c", "4"),
            ("key_without_value", ""),
            ("", "value_without_key"),
        ];

        for (i, (k, v)) in q.as_pairs_of_original_order().iter().enumerate() {
            let (k_, v_) = pairs_expected_as_str[i];
            assert_eq!(k.as_ref(), k_);
            assert_eq!(v.as_ref(), v_);
        }

        //
        let map_of_multiple_values_expected = hashmap! {
            "a"=>vec!("1"),
            "b"=>vec!("2"),
            "c"=>vec!("3", "4"),
            "key_without_value" => vec!(""),
            "" => vec!("value_without_key"),
        };
        dbg!("as_map_of_single_key_to_multiple_values");
        println!("as_map_of_single_key_to_multiple_values");
        let map = q.as_map_of_single_key_to_multiple_values();
        assert_eq!(map.len(), 5);

        for (k1, v1) in map {
            let v2 = map_of_multiple_values_expected.get(k1.as_ref()).unwrap();
            for (i, v2i) in v2.into_iter().enumerate() {
                assert_eq!(v1[i].to_string(), v2i.to_string());
            }
        }

        //
        let map_of_first_occurrence_value_expected = hashmap! {
            "a"=>"1",
            "b"=>"2",
            "c"=>"3",
            "key_without_value" => "",
            "" => "value_without_key",
        };
        dbg!("as_map_of_single_key_to_first_occurrence_value");
        let map = q.as_map_of_single_key_to_first_occurrence_value();
        assert_eq!(map.len(), 5);
        for (k1, v1) in map {
            let v2 = map_of_first_occurrence_value_expected
                .get(k1.as_ref())
                .unwrap();
            // let v3 = &v1;
            assert_eq!(&v1, v2); // ok, signifies comparing with references, it will auto-dereference to compare the value, which is more convenient
            let ptr1 = v1 as *const Cow<'_, str> as *const usize;
            let ptr2 = v2 as *const &str as *const usize;
            let msg = format!("{:p}, {:p}", ptr1, ptr2);
            dbg!(msg);
            println!("{:p}, {:p}", ptr1, ptr2);
            assert!(!std::ptr::eq(ptr1, ptr2));
            assert_eq!(*v1, **v2); // ok, value compare
        }

        //
        let map_of_last_occurrence_value_expected = hashmap! {
            "a"=>"1",
            "b"=>"2",
            "c"=>"4",
            "key_without_value" => "",
            "" => "value_without_key",
        };
        dbg!("as_map_of_single_key_to_last_occurrence_value");
        let map = q.as_map_of_single_key_to_last_occurrence_value();
        assert_eq!(map.len(), 5);

        for (k1, v1) in map {
            let v2 = map_of_last_occurrence_value_expected
                .get(k1.as_ref())
                .unwrap();
            assert_eq!(&v1, v2);
        }
    }
}
```



