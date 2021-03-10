//! # Ergonomic, Versatile Url-Encoded-Data Manipulator
//! Manipulate data of `application/x-www-form-urlencoded` format,
//! eg:
//!     * query_string of a url (eg: '?a=1&b=2&c=3&c=3&e=5')
//!     * http content-type with: `application/x-www-form-urlencoded`
//!
//! # Features:
//! * convenient api:
//!     * as_pairs
//!     * as_pairs_of_original_order
//!     * as_paris_of_sorted_order
//!
//!     * as_map_of_single_key_to_multiple_values
//!     * as_map_of_single_key_to_first_occurrence_value
//!     * as_map_of_single_key_to_last_occurrence_value
//!
//!     * set
//!     * push
//!     * clear
//!
//!     * get
//!     * get_first
//!     * get_last
//!
//!     * keys
//!     * len // pair length
//!     * keys_length
//!     * to_string
//!
//!     * // consult doc for more
//!
//! * Automatic unicode encoding/decoding
//!
//!
//! # Terminology
//! * Pair: a (key, format) tuple, `(Cow<'a, str>, Cow<'a, str>)`
//! * url encoded string: a string which is encoded by standards of `application/x-www-form-urlencoded`
//!
//! # Notes
//! * UrlEncodedDataPairScanner: Pairs Iterator, yields pairs only. (high performant)
//! * UrlEncodedData: eager version
//!
//! # Sample
//! ## Sample of url query string
//! ```rust
//! use url_encoded_data::UrlEncodedData;
//! use std::borrow::Cow;
//! // note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
//! let url = "https://google.com/?q=rust&ei=code";
//! let q = UrlEncodedData::parse_from_data_str(url);
//! // q.to_string(), best performance, (key, value) pairs are in un-deterministic order.
//! assert_eq!(q.to_string_of_original_order(), "https://google.com/?q=rust&ei=code");
//! assert_eq!(q.to_string_of_sorted_order(), "https://google.com/?ei=code&q=rust");
//!
//! // pairs length
//! assert_eq!(q.len(), 2);
//!
//! // keys length
//! assert_eq!(q.keys_length(), 2);
//!
//! // keys
//! assert!(q.keys().contains(&"q"));
//! assert!(q.keys().contains(&"ei"));
//!
//!
//! // let's do some manipulation
//! let url = "https://google.com/?q=rust&ei=code";
//! let q = UrlEncodedData::parse_from_data_str(url).set_one("q", "rust-lang");
//! let mut q = q; // const -> mut
//!
//! // set ->
//! let q = q.set("vector", &vec!["1", "2"])
//!          .set_one("a", "1")
//!          .set_one("b", "2")
//!          .set_one("hello", "world")
//!          .set("whole", &vec!["world", "世界"]) // utf-8, auto encoding and decoding
//!          .delete("ei") // ei is deleted
//!          .push("b", "3"); // now b is: vec!["1", "2"]
//! let q = q; // mut -> const
//!
//! // q.keys() // performant
//! assert_eq!(q.keys_of_original_order()[0].as_ref(), "q");
//!
//! // something like: https://google.com/?b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
//! println!("{}", q.to_string());
//!
//! // https://google.com/?q=rust-lang&b=2&b=3&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
//! println!("{}", q.to_string_of_original_order());
//!
//! // https://google.com/?a=1&b=2&b=3&hello=world&q=rust-lang&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C
//! println!("{}", q.to_string_of_sorted_order());
//!
//! ```
//! ## Sample of encoded data in www/x-www-form-urlencoded
//! ```rust
//! use url_encoded_data::UrlEncodedData;
//! use std::borrow::Cow;
//! // note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
//! let s = "b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C";
//! let q = UrlEncodedData::parse_from_data_str(s);
//! // q.to_string(), best performance, (key, value) pairs are in un-deterministic order.
//! assert_eq!(q.to_string_of_original_order(), s);
//!
//! // [("hello", "world"), ("vector", "1"), ("vector", "2"), ("whole", "world"), ("whole", "世界"), ("b", "2"), ("b", "3"), ("q", "rust-lang"), ("a", "1")]
//! println!("{:?}", q.as_pairs());
//!
//! // {"a": ["1"], "hello": ["world"], "b": ["2", "3"], "q": ["rust-lang"], "whole": ["world", "世界"], "vector": ["1", "2"]}
//! println!("{:?}", q.as_map_of_single_key_to_multiple_values());
//!
//! // {"b": "2", "a": "1", "q": "rust-lang", "whole": "world", "hello": "world", "vector": "1"}
//! println!("{:?}", q.as_map_of_single_key_to_first_occurrence_value());
//!
//! // {"q": "rust-lang", "whole": "世界", "vector": "2", "a": "1", "b": "3", "hello": "world"}
//! println!("{:?}", q.as_map_of_single_key_to_last_occurrence_value());
//! // assert!(false);
//!
//! ```
//!
//! ## Sample of performant pairs iterator: UrlEncodedDataPairScanner (Lazy version)
//! ```rust
//! use url_encoded_data::{UrlEncodedData, UrlEncodedDataPairScanner};
//! use std::borrow::Cow;
//! // note: the library will not check the validity of the url, it just searchs for url-encoded-data, eg: string after first '?' and then s.trim_start('?')
//! let s = "b=2&b=3&q=rust-lang&a=1&hello=world&vector=1&vector=2&whole=world&whole=%E4%B8%96%E7%95%8C";
//! let q = UrlEncodedDataPairScanner::from(s);
//! // same:
//! // let q = UrlEncodedDataPairScanner::parse_from_data_str(s);
//!
//! for (key, value) in q.iter() {
//!     // k, v are decoded
//!     // process the pair: (key, value)
//! }
//!
//! ```
//! ##
//! # Basic apis
//!
//! ## strigify: Stringify pairs to url encoded String
//!
//! ### example 1
//! ```rust
//! use url_encoded_data::*;
//! use url_encoded_data::stringify;
//! let encoded = stringify(&[("a", "b"), ("c", "d")]);
//! assert_eq!(encoded, "a=b&c=d");
//! ```
//!
//! ### example 2
//! ```rust
//! use url_encoded_data::*;
//! use url_encoded_data::stringify;
//! let encoded = stringify(&[("hello", "你好"), ("world", "世界")]);
//! assert_eq!(encoded, "hello=%E4%BD%A0%E5%A5%BD&world=%E4%B8%96%E7%95%8C");
//! ```
//!
//!
//! ## UrlEncodedDataPairIterator: **Lazy** iterator yielding pairs
//! ### example:
//!
//! ```rust
//! use url_encoded_data::*;
//! let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
//! for s in [
//!     qs.as_str(),
//!     ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
//!     ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
//! ]
//! .iter()
//! {
//!     let q = UrlEncodedDataPairScanner::from(*s);
//!     println!("got qs: {}", q);
//!
//!     let pairs_expected_as_str = [
//!         ("a", "1"),
//!         ("b", "2"),
//!         ("c", "3"),
//!         ("c", "4"),
//!         ("key_without_value", ""),
//!         ("", "value_without_key"),
//!     ];
//!
//!
//!     for (i, (k, v)) in q.iter().enumerate() {
//!         let (k_, v_) = pairs_expected_as_str[i];
//!         assert_eq!(k.as_ref(), k_);
//!         assert_eq!(v.as_ref(), v_);
//!     }
//! }
//! ```
//!
//! ## UrlEncodedData: parse url_encoded_data to pairs eagerly
//! ### main methods:
//! > for string: "a=1&b=2&a=3"
//! * as_pairs: ["a", "1"], ["b", "2"], ["c", "3"]
//! * as_map_of_single_key_to_multiple_values: {"a": ["1", "3"], "b": ["2"]}
//! * as_map_of_single_key_to_first_occurrence_value: {"a": "1", "b": "2"}
//! * as_map_of_single_key_to_last_occurrence_value: {"a": "3", "b": "2"}
//!
//! > get shortcuts
//! > note: for multiple get, use the result of map mehtods directly
//! * get_multiple_values: "a" -> vec!["1".to_string(), "3".to_string()]
//! * get_first_occurrence_value: "a" -> "1".to_string()
//! * get_last_occurrence_value: "a" -> "3".to_string()
//!
//!
//! Typical usage might be:
//!
//! ```rust
//! use url_encoded_data::*;
//! use std::borrow::Cow;
//! let s = "a=1&b=2&a=3";
//! let ued = UrlEncodedData::from(s);
//!
//! // get pairs
//! let pairs = ued.as_pairs();
//!
//! // 1:N
//! let map_n = ued.as_map_of_single_key_to_multiple_values();
//! let a = map_n.get(&Cow::from("a")).unwrap();
//! assert_eq!(a[0].as_ref(), "1");
//! assert_eq!(a[1].as_ref(), "3");
//!
//! // 1:first-value-met
//! let map_f = ued.as_map_of_single_key_to_first_occurrence_value();
//! let a = map_f.get(&Cow::from("a")).unwrap();
//! assert_eq!(a.as_ref(), "1");
//!
//! // 1:last-value-met
//! let map_l = ued.as_map_of_single_key_to_last_occurrence_value();
//! let a = map_l.get(&Cow::from("a")).unwrap();
//! assert_eq!(a.as_ref(), "3");
//!
//!
//! ```
//!
//! One time get(For best performance of multiple callings, use the result of method calling of as_map_*)
//! ```rust
//! use url_encoded_data::*;
//! use std::borrow::Cow;
//! let s = "a=1&b=2&a=3";
//! let ued = UrlEncodedData::from(s);
//!
//! assert_eq!(ued.get_multiple_values("a").unwrap().iter().map(|x| x.as_ref()).collect::<Vec<_>>(), vec!["1", "3"]);
//!
//! // get first occurrence value
//! assert_eq!(ued.get_first_occurrence_value("a").unwrap().as_ref(), "1");
//!
//! // get last occurrence value
//! assert_eq!(ued.get_last_occurrence_value("a").unwrap().as_ref(), "3");
//!
//! // no existed key
//! assert!(ued.get_last_occurrence_value("not-existed-key").is_none());
//! ```
//!
//! ### full example
//!
//! ```rust
//! #[macro_use]
//! extern crate maplit;
//! use url_encoded_data::*;
//!
//! fn main() {
//!     use std::borrow::Cow;
//! use url_encoded_data::UrlEncodedData;
//! let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
//!     for s in [
//!         qs.as_str(),
//!         ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
//!         ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
//!     ]
//!         .iter()
//!     {
//!         let q = UrlEncodedData::parse_from_data_str(s);
//!         // let mut q = UrlEncodedData::prepare(url_1);
//!         // let q = q.parse();
//!         println!("got qs: {}", q);
//!
//!         let pairs_expected_as_str = [
//!             ("a", "1"),
//!             ("b", "2"),
//!             ("c", "3"),
//!             ("c", "4"),
//!             ("key_without_value", ""),
//!             ("", "value_without_key"),
//!         ];
//!
//!         for (i, (k, v)) in q.as_pairs_of_original_order().iter().enumerate() {
//!             let (k_, v_) = pairs_expected_as_str[i];
//!             assert_eq!(k.as_ref(), k_);
//!             assert_eq!(v.as_ref(), v_);
//!         }
//!
//!         //
//!         let map_of_multiple_values_expected = hashmap! {
//!             "a"=>vec!("1"),
//!             "b"=>vec!("2"),
//!             "c"=>vec!("3", "4"),
//!             "key_without_value" => vec!(""),
//!             "" => vec!("value_without_key"),
//!         };
//!         dbg!("as_map_of_single_key_to_multiple_values");
//!         println!("as_map_of_single_key_to_multiple_values");
//!         let map = q.as_map_of_single_key_to_multiple_values();
//!         assert_eq!(map.len(), 5);
//!
//!         for (k1, v1) in map {
//!             let v2 = map_of_multiple_values_expected.get(k1.as_ref()).unwrap();
//!             for (i, v2i) in v2.into_iter().enumerate() {
//!                 assert_eq!(v1[i].to_string(), v2i.to_string());
//!             }
//!         }
//!
//!         //
//!         let map_of_first_occurrence_value_expected = hashmap! {
//!             "a"=>"1",
//!             "b"=>"2",
//!             "c"=>"3",
//!             "key_without_value" => "",
//!             "" => "value_without_key",
//!         };
//!         dbg!("as_map_of_single_key_to_first_occurrence_value");
//!         let map = q.as_map_of_single_key_to_first_occurrence_value();
//!         assert_eq!(map.len(), 5);
//!         for (k1, v1) in map {
//!             let v2 = map_of_first_occurrence_value_expected
//!                 .get(k1.as_ref())
//!                 .unwrap();
//!             // let v3 = &v1;
//!             assert_eq!(&v1, v2); // ok, signifies comparing with references, it will auto-dereference to compare the value, which is more convenient
//!             let ptr1 = v1 as *const Cow<'_, str> as *const usize;
//!             let ptr2 = v2 as *const &str as *const usize;
//!             let msg = format!("{:p}, {:p}", ptr1, ptr2);
//!             dbg!(msg);
//!             println!("{:p}, {:p}", ptr1, ptr2);
//!             assert!(!std::ptr::eq(ptr1, ptr2));
//!             assert_eq!(*v1, **v2); // ok, value compare
//!         }
//!
//!         //
//!         let map_of_last_occurrence_value_expected = hashmap! {
//!             "a"=>"1",
//!             "b"=>"2",
//!             "c"=>"4",
//!             "key_without_value" => "",
//!             "" => "value_without_key",
//!         };
//!         dbg!("as_map_of_single_key_to_last_occurrence_value");
//!         let map = q.as_map_of_single_key_to_last_occurrence_value();
//!         assert_eq!(map.len(), 5);
//!
//!         for (k1, v1) in map {
//!             let v2 = map_of_last_occurrence_value_expected
//!                 .get(k1.as_ref())
//!                 .unwrap();
//!             assert_eq!(&v1, v2);
//!         }
//!     }
//! }
//! ```
//!
//!
//!

#[cfg(test)]
#[macro_use]
extern crate maplit;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use url;
use url as url_lib;
use url::form_urlencoded::Parse;
use std::collections::hash_map::Entry;

pub type Pair<'a> = (Cow<'a, str>, Cow<'a, str>);
// type StringPair = (String, String);
type StrPair<'a> = (&'a str, &'a str);
type RefPair<'a> = (&'a Cow<'a, str>, &'a Cow<'a, str>);

/// # Algorithm
/// 1. If param: `str` contains '?', then url_encoded_string = <there_after>.trim_start('?')
/// 2. Else, url_encoded_string = param: `str`
///
/// # Example
/// ```rust
/// use url_encoded_data::*;
/// use url_encoded_data::extract_url_encoded_string;
/// let s = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
/// assert_eq!(extract_url_encoded_string(s), s);
///
/// let s_prefixed_with_question_mark = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
/// assert_eq!(extract_url_encoded_string(s_prefixed_with_question_mark), s);
///
/// let url = "http://abc.com/?".to_string() + s;
/// assert_eq!(extract_url_encoded_string(&url), s);
///
/// let url = "http://abc.com/?".to_string() + s;
/// assert_eq!(extract_url_encoded_string(&url), s);
/// ```
pub fn extract_url_encoded_string(s: &str) -> &str {
    let found = s.find('?');
    match found {
        None => s,
        Some(idx) => {
            let left = s[idx..s.len()].trim_start_matches('?'); // use the str behind
            left
        }
    }
}

/// split to 'prefix' + 'data_str'
/// ```rust
/// use url_encoded_data::split_url_encoded_string;
/// let url = "https://google.com/?q=rust";
/// let (prefix, data_str) = split_url_encoded_string(url);
/// println!("prefix: {:?}, data_str: {:?}", prefix, data_str);
/// assert_eq!(prefix, "https://google.com/?");
/// assert_eq!(data_str, "q=rust");
///
/// let s = "";
/// let (prefix, data_str) = split_url_encoded_string(s);
/// println!("prefix: {:?}, data_str: {:?}", prefix, data_str);
/// assert_eq!(prefix, "");
/// assert_eq!(data_str, "");
///
/// let s = "?";
/// let (prefix, data_str) = split_url_encoded_string(s);
/// println!("prefix: {:?}, data_str: {:?}", prefix, data_str);
/// assert_eq!(prefix, "?");
/// assert_eq!(data_str, "");
///
/// let s = "q=rust";
/// let (prefix, data_str) = split_url_encoded_string(s);
/// println!("prefix: {:?}, data_str: {:?}", prefix, data_str);
/// assert_eq!(prefix, "");
/// assert_eq!(data_str, "q=rust");
///
/// ```
pub fn split_url_encoded_string(s: &str) -> (&str, &str) {
    let found = s.find('?');
    match found {
        None => ("", s),
        Some(idx) => {
            let prefix = &s[0..idx + 1];
            let left = s[idx + 1..s.len()].trim_start_matches('?'); // use the str behind
            (prefix, left)
        }
    }
}

/// # Stringify pairs to url encoded String
///
/// ## example 1
/// ```rust
/// use url_encoded_data::*;
/// use url_encoded_data::stringify;
/// let encoded = stringify(&[("a", "b"), ("c", "d")]);
/// assert_eq!(encoded, "a=b&c=d");
/// ```
///
/// ## example 2
/// ```rust
/// use url_encoded_data::*;
/// use url_encoded_data::stringify;
/// let encoded = stringify(&[("hello", "你好"), ("world", "世界")]);
/// assert_eq!(encoded, "hello=%E4%BD%A0%E5%A5%BD&world=%E4%B8%96%E7%95%8C");
/// ```
///
/// ## example 3
/// ```rust
/// use url_encoded_data::*;
/// use url_encoded_data::stringify;
/// let encoded = stringify(&[("foo", "bar & baz"), ("saison", "Été+hiver")]);
/// assert_eq!(encoded, "foo=bar+%26+baz&saison=%C3%89t%C3%A9%2Bhiver");
/// ```
///
/// Panics if called more than once.
pub fn stringify<'a>(pairs: &'a [StrPair<'a>]) -> String {
    let mut s = url_lib::form_urlencoded::Serializer::new(String::new());
    for &(k, v) in pairs.into_iter() {
        s.append_pair(k, v);
    }
    s.finish()
}

// A scanner iterating (decoded_key, decoded_value) pair in order.
#[derive(Clone)]
pub struct UrlEncodedDataPairScanner<'a> {
    prefix: &'a str,
    pub original_data_str: &'a str,
    pairs_iterator: Parse<'a>,
}

impl<'a> Display for UrlEncodedDataPairScanner<'a> {
    /// ```rust
    /// use url_encoded_data::*;
    /// let q = UrlEncodedDataPairScanner::from("abcd=efg");
    /// let display = format!("got qs: {}", q);
    /// assert!(display.len() > 3)
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}{}", self.prefix, self.original_data_str)
    }
}

/// # Lazy iterator yielding pairs
impl<'a> UrlEncodedDataPairScanner<'a> {
    /// # Iterator of pairs
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    /// for s in [
    ///     qs.as_str(),
    ///     ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///     ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    /// ]
    /// .iter()
    /// {
    ///     let q = UrlEncodedDataPairScanner::from(*s);
    ///     println!("got qs: {}", q);
    ///
    ///     let pairs_expected_as_str = [
    ///         ("a", "1"),
    ///         ("b", "2"),
    ///         ("c", "3"),
    ///         ("c", "4"),
    ///         ("key_without_value", ""),
    ///         ("", "value_without_key"),
    ///     ];
    ///
    ///
    ///     for (i, (k, v)) in q.iter().enumerate() {
    ///         let (k_, v_) = pairs_expected_as_str[i];
    ///         assert_eq!(k.as_ref(), k_);
    ///         assert_eq!(v.as_ref(), v_);
    ///     }
    /// }
    /// ```
    pub fn iter(&'a self) -> impl Iterator<Item=Pair<'a>> {
        self.pairs_iterator.into_iter()
    }

    /// # Iterator of pairs
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedDataPairScanner;
    /// let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    /// for s in [
    ///     qs.as_str(),
    ///     ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///     ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    /// ]
    /// .iter()
    /// {
    ///     let q = UrlEncodedDataPairScanner::parse_from_str(s);
    ///     println!("got qs: {}", q);
    ///
    ///     let pairs_expected_as_str = [
    ///         ("a", "1"),
    ///         ("b", "2"),
    ///         ("c", "3"),
    ///         ("c", "4"),
    ///         ("key_without_value", ""),
    ///         ("", "value_without_key"),
    ///     ];
    ///
    ///
    ///     for (i, (k, v)) in q.iter().enumerate() {
    ///         let (k_, v_) = pairs_expected_as_str[i];
    ///         assert_eq!(k.as_ref(), k_);
    ///         assert_eq!(v.as_ref(), v_);
    ///     }
    /// }
    /// ```
    pub fn parse_from_str(s: &'a str) -> Self {
        let (prefix, original_data_str) = split_url_encoded_string(s);
        let pairs_iterator = url_lib::form_urlencoded::parse(original_data_str.as_bytes());
        Self {
            prefix,
            original_data_str,
            pairs_iterator,
        }
    }
}

impl<'a> From<&'a str> for UrlEncodedDataPairScanner<'a> {
    /// # UrlEncodedDataPairIterator from &str
    /// ```rust
    /// use url_encoded_data::*;
    /// let q = UrlEncodedDataPairScanner::from("abcd=efg");
    /// let first_pair = q.iter().next().unwrap();
    /// let (k, v) = first_pair;
    /// assert_eq!(k.as_ref(), "abcd");
    /// assert_eq!(v.as_ref(), "efg");
    /// ```
    fn from(s: &'a str) -> Self {
        Self::parse_from_str(s)
    }
}

/// Represents the form-urlencoded data: eg: url query string, or application/x-www-form-urlencoded of the body.
#[derive(Clone)]
pub struct UrlEncodedData<'a> {
    // original prefix of the input string before query_string.
    prefix: &'a str,

    // un-escaped raw data string extracted from input
    pub original_data_str: &'a str,

    // more consistent displaying to old str, if selected
    original_keys_in_order: Vec<Cow<'a, str>>,

    // map: 1 -> many, one key to multiple values.
    map: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
    // pairs: Vec<Pair<'a>>,
}

// /// Yields an iterator with Item = (key, value) pair
// ///  Item = (Cow<'a, str>, Cow<'a, str>)
// impl<'a> Iterator for UrlEncodedData<'a> {
//     type Item = &'a RefPair<'a> ;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.map.iter().flat_map(|(k, v)| {
//             for i in v {
//                 return Some((k, i));
//             }
//         });
//         None
//
//     }
// }

impl<'a> From<&'a str> for UrlEncodedData<'a> {
    /// # UrlEncodedData from &str
    /// ```rust
    /// use url_encoded_data::*;
    /// let q = UrlEncodedData::from("abcd=efg");
    /// let first_pair = q.iter().next().unwrap();
    /// let (k, v) = first_pair;
    /// assert_eq!(k.as_ref(), "abcd");
    /// assert_eq!(v.as_ref(), "efg");
    /// ```
    fn from(s: &'a str) -> Self {
        // Self::prepare(s).parse()
        // unimplemented!()
        Self::parse_from_data_str(s)
    }
}

impl<'a> Display for UrlEncodedData<'a> {
    /// ```rust
    /// use url_encoded_data::*;
    /// let q = UrlEncodedData::from("abcd=efg");
    /// let display = format!("got qs: {}", q);
    /// assert!(display.len() > 3)
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "prefix: {:?}, old str: {:?}; old keys: {:?}; current map: {:?}; current str: {:?}",
            self.prefix, self.original_data_str, self.original_keys_in_order, self.map, self.to_string_of_original_order()
        )
    }
}

/// # UrlEncodedData: parse url_encoded_data to pairs eagerly
impl<'a> UrlEncodedData<'a> {
    // fn prepare(input: &'a str) -> UrlEncodedData<'a> {
    //     // let parse = url_lib::form_urlencoded::parse(raw.as_bytes()).clone(); // error: returns a value referencing data owned by the current function
    //     let raw = retrieve_url_encoded_data_to_string(input);
    //     let parse = url_lib::form_urlencoded::parse("".as_bytes()); // error: returns a value referencing data owned by the current function
    //     UrlEncodedData {
    //         // input,
    //         parse: parse,
    //         pairs: Vec::new(),
    //         raw: raw,
    //         is_parsed: false,
    //         // parse: url_lib::form_urlencoded::parse("".as_bytes()),
    //         // parse: parse,
    //     }
    // }

    // /// # from str
    // /// 1. If param: `str` contains '?', then url_encoded_string = <there_after>.trim_start('?')
    // /// 2. Else, url_encoded_string = param: `str`
    // fn parse(&'a mut self) -> Self {
    //     // self.raw = retrieve_url_encoded_string(self.input);
    //     let parse = url_lib::form_urlencoded::parse(self.raw.as_bytes());
    //     self.pairs = parse.into_iter().collect();
    //     self.is_parsed = true;
    //     self.clone()
    // }

    /// # UrlEncodedData from &str
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedData;
    /// let q = UrlEncodedData::parse_from_data_str("abcd=efg");
    /// let first_pair = q.iter().next().unwrap();
    /// let (k, v) = first_pair;
    /// assert_eq!(k.as_ref(), "abcd");
    /// assert_eq!(v.as_ref(), "efg");
    /// ```
    pub fn parse_from_data_str(s: &'a str) -> Self {
        let (prefix, data_str) = split_url_encoded_string(s);
        let parse = url_lib::form_urlencoded::parse(data_str.as_bytes());
        let pairs: Vec<Pair> = parse.into_iter().collect();
        let mut map: HashMap<Cow<'_, str>, Vec<Cow<'_, str>>> = HashMap::new();
        let mut original_keys_in_order: Vec<Cow<str>> = vec![];
        for (k, v) in pairs {
            map.entry(k.clone()).or_default().push(v);
            if !original_keys_in_order.contains(&k) {
                original_keys_in_order.push(k);
            }
        }

        Self {
            prefix,
            original_data_str: data_str,
            map,
            original_keys_in_order,
        }
    }

    /// # As pairs slice in random order, better performance than `as_pairs_of_original_order` and `as_pairs_of_sorted_order`
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedData;
    /// let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    /// for s in [
    ///     qs.as_str(),
    ///     ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///     ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    /// ]
    /// .iter()
    /// {
    ///     let q = UrlEncodedData::parse_from_data_str(s);
    ///     // let mut q = UrlEncodedData::prepare(url_1);
    ///     // let q = q.parse();
    ///     println!("got qs: {}", q);
    ///
    ///     let pairs_expected_as_str = [
    ///         ("a", "1"),
    ///         ("b", "2"),
    ///         ("c", "3"),
    ///         ("c", "4"),
    ///         ("key_without_value", ""),
    ///         ("", "value_without_key"),
    ///     ];
    ///
    ///    for (k, v) in q.as_pairs().iter().map(|(k,v)|(k.as_ref(), v.as_ref())) {
    ///        assert!(pairs_expected_as_str.contains(&(k, v)));
    ///    }
    /// }
    /// ```
    ///
    /// ## An example of decoding:
    /// ```rust
    /// use url_encoded_data::*;
    /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
    /// let qs = UrlEncodedData::from(s);
    /// let str_vec: Vec<_> = qs.as_pairs().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
    /// // random order
    /// assert!(str_vec[0] == ("hello", "你好") || str_vec[0] == ("world", "世界"));
    /// assert!(str_vec[1] == ("hello", "你好") || str_vec[1] == ("world", "世界"));
    /// ```
    pub fn as_pairs(&'a self) -> Vec<RefPair<'a>> {
        let mut vector = vec![];
        for (k, v) in self.map.iter() {
            for i in v.iter() {
                vector.push((k, i));
            }
        }
        vector
    }

    /// # As pairs slice
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "c=3&a=1&b=2&c=4&key_without_value&=value_without_key";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// // let mut q = UrlEncodedData::prepare(url_1);
    /// // let q = q.parse();
    /// println!("got qs: {}", q);
    ///
    /// let pairs_expected_as_str = [
    ///     ("c", "3"),
    ///     ("c", "4"),
    ///     ("a", "1"),
    ///     ("b", "2"),
    ///     ("key_without_value", ""),
    ///     ("", "value_without_key"),
    /// ];
    ///
    ///
    /// for (i, (k, v)) in q.as_pairs_of_original_order().iter().enumerate() {
    ///     let (k_, v_) = pairs_expected_as_str[i];
    ///     println!("{}, ({}, {}), ({}, {})", i, k, v, k_, v_);
    ///     assert_eq!(k.as_ref(), k_);
    ///     assert_eq!(v.as_ref(), v_);
    /// }
    ///
    /// ```
    ///
    /// ## An example of decoding:
    /// ```rust
    /// use url_encoded_data::*;
    /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
    /// let qs = UrlEncodedData::from(s);
    /// let str_vec: Vec<_> = qs.as_pairs_of_original_order().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
    /// assert_eq!(str_vec[0], ("hello", "你好"));
    /// assert_eq!(str_vec[1], ("world", "世界"));
    /// ```
    // donetodo: found a better a way to return Vec<RefPair>, but i have no time to do it now.
    pub fn as_pairs_of_original_order(&'a self) -> Vec<RefPair<'a>> {
        let mut vector = vec![];
        for k_old in self.original_keys_in_order.iter() {
            if self.map.contains_key(k_old) {
                for element in self.map.get(k_old).unwrap() {
                    vector.push((k_old, element))
                }
            }
        }

        // now add left
        for (k, v) in self.map.iter() {
            if !self.original_keys_in_order.contains(k) {
                for element in v {
                    vector.push((k, element))
                }
            }
        }

        vector
    }

    /// # As pairs slice
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "c=3&a=1&b=2&c=4&key_without_value&=value_without_key";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// // let mut q = UrlEncodedData::prepare(url_1);
    /// // let q = q.parse();
    /// println!("got qs: {}", q);
    ///
    /// let pairs_expected_as_str = [
    ///     ("", "value_without_key"),
    ///     ("a", "1"),
    ///     ("b", "2"),
    ///     ("c", "3"),
    ///     ("c", "4"),
    ///     ("key_without_value", ""),
    /// ];
    ///
    /// dbg!(q.as_pairs_of_sorted_order());
    /// for (i, (k, v)) in q.as_pairs_of_sorted_order().iter().enumerate() {
    ///     let (k_, v_) = pairs_expected_as_str[i];
    ///     println!("{}, ({}, {}), ({}, {})", i, k, v, k_, v_);
    ///     assert_eq!(k.as_ref(), k_);
    ///     assert_eq!(v.as_ref(), v_);
    /// }
    /// ```
    ///
    /// ## An example of decoding:
    /// ```rust
    /// use url_encoded_data::*;
    /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
    /// let qs = UrlEncodedData::from(s);
    /// let str_vec: Vec<_> = qs.as_pairs_of_sorted_order().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
    /// assert_eq!(str_vec[0], ("hello", "你好"));
    /// assert_eq!(str_vec[1], ("world", "世界"));
    /// ```
    pub fn as_pairs_of_sorted_order(&'a self) -> Vec<RefPair<'a>> {
        let mut vector = vec![];

        let mut keys_in_sorted_order: Vec<_> = self.map.keys().collect();
        keys_in_sorted_order.sort_unstable();
        dbg!(&keys_in_sorted_order);

        for key in keys_in_sorted_order {
            for element in self.map.get(key).unwrap() {
                vector.push((key, element))
            }
        }
        vector
    }

    /// Yields an iterator with Item = (key, value) pair
    ///  Item = (Cow<'a, str>, Cow<'a, str>)
    pub fn iter(&'a self) -> impl Iterator<Item=RefPair<'a>> {
        self.as_pairs().into_iter()
        // // let c = self.as_pairs().iter();
        // self.map.iter().flat_map(|(k, v)| {
        //     let mut vector = vec![];
        //     for i in v {
        //         vector.push((k, i));
        //     }
        //     vector.iter()
        // })
    }

    // pub fn as_str_pairs(&'a self) -> Vec<(&'a str, &'a str)> {
    //     self.iter().collect()
    //     // self.p.clone().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()
    //     // let mut vector = vec![];
    //     // for (k, v) in self.p.clone() {
    //     //     vector.push((k.borrow(), v.borrow()));
    //     // }
    //     // vector
    // }

    // pub fn as_str_pairs(&'a mut self) -> Vec<(&'a str, &'a str)> {
    //     self.pairs().iter()
    //         .map(|(k, v)| (k.as_ref(), v.as_ref()))
    //         .collect()
    // }

    /// # As_String_pairs
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let qs = "a=1&b=2";
    /// let q = UrlEncodedData::parse_from_data_str(qs);
    /// // let mut q = UrlEncodedData::prepare(url_1);
    /// // let q = q.parse();
    /// println!("got qs: {}", q);
    ///
    /// let pairs_expected_as_str = [
    ///     ("a", "1"),
    ///     ("b", "2"),
    /// ];
    ///
    ///
    /// for (k, v) in q.as_string_pairs().iter() {
    ///     if k.as_str() == "a" {
    ///         assert_eq!(v.as_str(), "1")
    ///     } else {
    ///         assert_eq!(v.as_str(), "2")
    ///     }
    /// }
    /// ```
    pub fn as_string_pairs(&'a self) -> Vec<(String, String)> {
        self.as_pairs()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    pub fn stringify(pairs: &Vec<RefPair>) -> String {
        let mut s = url_lib::form_urlencoded::Serializer::new(String::new());
        for (k, v) in pairs.into_iter() {
            s.append_pair(k.as_ref(), v.as_ref());
        }
        s.finish()
    }

    /// to string
    /// ``` rust
    /// use url_encoded_data::UrlEncodedData;
    /// let url = "https://google.com/?q=rust";
    /// let q = UrlEncodedData::parse_from_data_str(url).set_one("q", "rust-lang");
    /// assert_eq!(q.to_string(), "https://google.com/?q=rust-lang")
    /// ```
    pub fn to_string(&self) -> String {
        self.prefix.to_owned() + &Self::stringify(&self.as_pairs())
    }

    /// to_string_of_original_order
    /// ``` rust
    /// use url_encoded_data::UrlEncodedData;
    /// let url = "https://google.com/?q=rust&ei=code";
    /// let q = UrlEncodedData::parse_from_data_str(url).set_one("q", "rust-lang");
    /// assert_eq!(q.to_string_of_original_order(), "https://google.com/?q=rust-lang&ei=code")
    /// ```
    pub fn to_string_of_original_order(&self) -> String {
        self.prefix.to_owned() + &Self::stringify(&self.as_pairs_of_original_order())
    }
    /// to_string_of_sorted_order
    /// ``` rust
    /// use url_encoded_data::UrlEncodedData;
    /// let url = "https://google.com/?q=rust&ei=code";
    /// let q = UrlEncodedData::parse_from_data_str(url).set_one("q", "rust-lang");
    /// assert_eq!(q.to_string_of_sorted_order(), "https://google.com/?ei=code&q=rust-lang")
    /// ```
    pub fn to_string_of_sorted_order(&self) -> String {
        self.prefix.to_owned() + &Self::stringify(&self.as_pairs_of_sorted_order())
    }

    /// # As Map of Single-key to Multiple-values
    ///
    /// # example:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate maplit;
    ///
    /// fn main() {
    ///     use url_encoded_data::*;
    ///     use url_encoded_data::*;
    /// use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    ///     for s in [
    ///         qs.as_str(),
    ///         ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///         ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    ///     ]
    ///     .iter()
    ///     {
    ///         let q = UrlEncodedData::parse_from_data_str(s);
    ///         println!("got qs: {}", q);
    ///
    ///         //
    ///         let map_of_multiple_values_expected = hashmap! {
    ///             "a"=>vec!("1"),
    ///             "b"=>vec!("2"),
    ///             "c"=>vec!("3", "4"),
    ///             "key_without_value" => vec!(""),
    ///             "" => vec!("value_without_key"),
    ///         };
    ///         dbg!("as_map_of_single_key_to_multiple_values");
    ///         println!("as_map_of_single_key_to_multiple_values");
    ///         let map = q.as_map_of_single_key_to_multiple_values();
    ///
    ///         assert_eq!(map.len(), 5);
    ///         for (k1, v1) in map {
    ///             let v2 = map_of_multiple_values_expected.get(k1.as_ref()).unwrap();
    ///             for (i, v2i) in v2.into_iter().enumerate() {
    ///                 assert_eq!(v1[i].to_string(), v2i.to_string());
    ///             }
    ///         }
    ///
    ///     }
    /// }
    /// ```
    /// eg: "a=b&a=c&d=&e" => {"a" : ["b", "c"], "d: [""], "": ["e"]}
    pub fn as_map_of_single_key_to_multiple_values(
        &'a self,
    ) -> &'a HashMap<Cow<'a, str>, Vec<Cow<'a, str>>> {
        &self.map
    }

    /// # As Map of Single-key to First Occurrence Value
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::*;
    /// #[macro_use]
    /// extern crate maplit;
    ///
    /// fn main() {
    ///     use std::borrow::Cow;
    /// use url_encoded_data::UrlEncodedData;
    /// let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    ///     for s in [
    ///         qs.as_str(),
    ///         ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///         ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    ///     ]
    ///     .iter()
    ///     {
    ///         let q = UrlEncodedData::parse_from_data_str(s);
    ///         println!("got qs: {}", q);
    ///
    ///         //
    ///         let map_of_multiple_values_expected = hashmap! {
    ///             "a"=>vec!("1"),
    ///             "b"=>vec!("2"),
    ///             "c"=>vec!("3", "4"),
    ///             "key_without_value" => vec!(""),
    ///             "" => vec!("value_without_key"),
    ///         };
    ///         let map_of_first_occurrence_value_expected = hashmap! {
    ///             "a"=>"1",
    ///             "b"=>"2",
    ///             "c"=>"3",
    ///             "key_without_value" => "",
    ///             "" => "value_without_key",
    ///         };
    ///         dbg!("as_map_of_single_key_to_first_occurrence_value");
    ///         let mut q_clone = q.clone();
    ///         let map = q_clone.as_map_of_single_key_to_first_occurrence_value();
    ///         assert_eq!(map.len(), 5);
    ///         for (k1, v1) in map {
    ///             let v2 = map_of_first_occurrence_value_expected
    ///                 .get(k1.as_ref())
    ///                 .unwrap();
    ///             // let v3 = &v1;
    ///             assert_eq!(&v1, v2); // ok, signifies comparing with each reference, it will  auto-dereference to compare the value, which is more convenient
    ///             let ptr1 = v1 as *const Cow<'_, str> as *const usize;
    ///             let ptr2 = v2 as *const &str as *const usize;
    ///             let msg = format!("{:p}, {:p}", ptr1, ptr2);
    ///             dbg!(msg);
    ///             println!("{:p}, {:p}", ptr1, ptr2);
    ///             assert!(!std::ptr::eq(ptr1, ptr2));
    ///             assert_eq!(*v1, **v2); // ok, value compare
    ///         }
    ///     }
    /// }
    /// ```
    /// eg: "a=b&a=c" => {"a" : "b"}
    pub fn as_map_of_single_key_to_first_occurrence_value(
        &'a self,
    ) -> HashMap<&'a Cow<str>, &'a Cow<str>> {
        let mut m = HashMap::new();
        for (k, v) in self.as_pairs() {
            m.entry(k).or_insert(v);
        }
        m
    }

    /// # As Map of Single-key to Last Occurrence Value
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::*;
    /// use url_encoded_data::*;
    /// #[macro_use]
    /// extern crate maplit;
    ///
    /// fn main() {
    ///     use url_encoded_data::UrlEncodedData;
    /// let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
    ///     for s in [
    ///         qs.as_str(),
    ///         ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
    ///         ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
    ///     ]
    ///     .iter()
    ///     {
    ///         let q = UrlEncodedData::parse_from_data_str(s);
    ///         println!("got qs: {}", q);
    ///
    ///         let map_of_last_occurrence_value_expected = hashmap! {
    ///             "a"=>"1",
    ///             "b"=>"2",
    ///             "c"=>"4",
    ///             "key_without_value" => "",
    ///             "" => "value_without_key",
    ///         };
    ///         dbg!("as_map_of_single_key_to_last_occurrence_value");
    ///         let map = q.as_map_of_single_key_to_last_occurrence_value();
    ///         assert_eq!(map.len(), 5);
    ///
    ///         for (k1, v1) in map {
    ///             let v2 = map_of_last_occurrence_value_expected
    ///                 .get(k1.as_ref())
    ///                 .unwrap();
    ///             assert_eq!(&v1, v2);
    ///         }
    ///     }
    /// }
    /// ```
    /// eg: "a=b&a=c" => {"a" : "b"}
    pub fn as_map_of_single_key_to_last_occurrence_value(
        &'a self,
    ) -> HashMap<&'a Cow<str>, &'a Cow<str>> {
        let mut m = HashMap::new();
        for (k, v) in self.as_pairs() {
            m.insert(k, v);
        }
        m
    }

    /// # Get multiple values by key
    ///
    /// # example:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate maplit;
    ///
    /// fn main() {
    ///     use url_encoded_data::*;
    ///     use url_encoded_data::*;
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs);
    ///     println!("got qs: {}", q);
    ///
    ///     assert_eq!(q.get_multiple_values("a").unwrap()[0].as_ref(), "1");
    ///     assert_eq!(q.get_multiple_values("c").unwrap().iter().map(|x| x.as_ref()).collect::<Vec<_>>(), vec!["3", "4"]);
    ///     assert_eq!(q.get_multiple_values("non-exist"), None);
    /// }
    /// ```
    pub fn get_multiple_values<'b>(&'a self, key: &'b str) -> Option<&Vec<Cow<str>>> {
        self.map.get(key)

        // Some(
        //     self.as_map_of_single_key_to_multiple_values()
        //         .get(&Cow::from(key))?
        //         .iter()
        //         .fold(Vec::new(), |mut acc, v| {
        //             acc.push(v.to_string());
        //             acc
        //         }),
        // )
    }

    /// # Get first occurrence value by key
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// println!("got qs: {}", q);
    ///
    /// assert_eq!(q.get_first_occurrence_value("a").unwrap().as_ref(), "1");
    /// assert_eq!(q.get_first_occurrence_value("c").unwrap().as_ref(), "3");
    /// assert_eq!(q.get_first_occurrence_value("non-exist"), None);
    /// ```
    pub fn get_first_occurrence_value<'b>(&'a self, key: &'b str) -> Option<&'a Cow<str>> {
        self.as_map_of_single_key_to_first_occurrence_value()
            .get(&Cow::from(key))
            .map(|&v| v)
        // .map(|v| v.to_string())
    }

    /// # Get last occurrence value by key
    ///
    /// # example:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate maplit;
    ///
    /// fn main() {
    ///     use url_encoded_data::*;
    ///     use url_encoded_data::*;
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs);
    ///     println!("got qs: {}", q);
    ///
    ///     assert_eq!(q.get_last_occurrence_value("a").unwrap(), "1");
    ///     assert_eq!(q.get_last_occurrence_value("c").unwrap(), "4");
    ///     assert_eq!(q.get_last_occurrence_value("non-exist"), None);
    /// }
    /// ```
    pub fn get_last_occurrence_value<'b>(&'a self, key: &'b str) -> Option<&'a Cow<str>> {
        self.as_map_of_single_key_to_last_occurrence_value()
            .get(&Cow::from(key))
            .map(|&v| v)
    }

    /// # set a key with value slice
    ///
    /// # example:
    ///
    /// ```rust
    /// fn main() {
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs).set("a", &["100", "200"]);
    ///
    ///     assert_eq!(q.get("a").unwrap(), vec!["100", "200"]);
    /// }
    /// ```
    pub fn set(mut self, key: &'a str, value: &[&'a str]) -> Self {
        self.map.insert(
            Cow::from(key),
            value.iter().map(|x| Cow::from(*x)).collect::<Vec<_>>(),
        );
        self
    }

    /// # set a key with exactly one value
    ///
    /// # example:
    ///
    /// ```rust
    /// fn main() {
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs).set_one("a", "100");
    ///
    ///     assert_eq!(q.get_first("a").unwrap(), "100");
    /// }
    /// ```
    pub fn set_one(mut self, key: &'a str, value: &'a str) -> Self {
        self.map.insert(Cow::from(key), vec![Cow::from(value)]);
        self
    }

    /// # Push(aka, append) a value to the key
    ///
    /// # example:
    ///
    /// ```rust
    /// fn main() {
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs).push("a", "100").push("hello", "world");
    ///
    ///     assert_eq!(q.get("a").unwrap(), vec!["1", "100"]);
    ///     assert_eq!(q.get("hello").unwrap(), vec!["world"]);
    ///     assert_eq!(q.get_first("hello").unwrap(), "world");
    /// }
    /// ```
    pub fn push(mut self, key: &'a str, value: &'a str) -> Self {
        match self.map.entry(Cow::from(key)) {
            Entry::Occupied(mut entry) => entry.get_mut().push(Cow::from(value)),
            Entry::Vacant(entry) => { entry.insert(vec![Cow::from(value)]); }
        }
        self
    }

    // pub fn set_one_by_ref_mut(&'a mut self, key: &'a str, value: &'a str) ->  &'a mut Self {
    //     self.map.insert(Cow::from(key), vec![Cow::from(value)]);
    //     self
    // }
    //
    // /// # Done setting
    // ///
    // /// # example:
    // ///
    // /// ```rust
    // /// fn main() {
    // ///     use url_encoded_data::UrlEncodedData;
    // ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    // ///     let mut q = UrlEncodedData::parse_from_data_str(qs).set_one("a", "100").set_one("b", "200").done();
    // ///
    // ///     assert_eq!(q.get_first("a").unwrap(), "100");
    // ///     assert_eq!(q.get_first("b").unwrap(), "200");
    // /// }
    // /// ```
    // pub fn done(&'a mut self) -> &'a Self {
    //     self
    // }
    //
    // pub fn done2(&'a mut self) -> Self {
    //     self.clone()
    // }

    pub fn mut_done(&'a mut self) -> &'a Self {
        self
    }


    /// # Get multiple values, same as method: get_multiple_values(key) but return `Option<Vec<&'a str>>` instead
    ///
    /// # example:
    ///
    /// ```rust
    /// fn main() {
    ///     use url_encoded_data::UrlEncodedData;
    ///     let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
    ///     let q = UrlEncodedData::parse_from_data_str(qs);
    ///     assert_eq!(q.get("c").unwrap(), vec!["3", "4"]);
    /// }
    /// ```
    pub fn get<'b>(&'a self, key: &'b str) -> Option<Vec<&'a str>> {
        Some(
            self.get_multiple_values(key)?
                .iter()
                .fold(Vec::new(), |mut acc, v| {
                    acc.push(v.as_ref());
                    acc
                }),
        )
    }

    /// # Get first occurrence value by key, similar to self.get_first_occurrence_value(key), but returns `Option<&'a str>` instead.
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// println!("got qs: {}", q);
    ///
    /// assert_eq!(q.get_first("c").unwrap(), "3");
    /// assert_eq!(q.get_first("a").unwrap(), "1");
    /// assert_eq!(q.get_first("b").unwrap(), "2");
    /// assert_eq!(q.get_first("non-exist"), None);
    /// ```
    /// similar to self.get_first_occurrence_value(key), but returns `Option<&'a str>` instead.
    pub fn get_first<'b>(&'a self, key: &'b str) -> Option<&'a str> {
        self.get_first_occurrence_value(key).map(|x| x.as_ref())
    }

    /// # Get last occurrence value by key, similar to self.get_last_occurrence_value(key), but returns `Option<&'a str>` instead.
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// println!("got qs: {}", q);
    ///
    /// assert_eq!(q.get_last("c").unwrap(), "4");
    /// assert_eq!(q.get_last("a").unwrap(), "1");
    /// assert_eq!(q.get_last("b").unwrap(), "2");
    /// assert_eq!(q.get_last("non-exist"), None);
    /// ```
    /// similar to self.get_last_occurrence_value(key), but returns `Option<&'a str>` instead.
    pub fn get_last<'b>(&'a self, key: &'b str) -> Option<&'a str> {
        self.get_last_occurrence_value(key).map(|x| x.as_ref())
    }

    /// # Delete (k, v) pairs of key
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4&d=5";
    /// let mut  q = UrlEncodedData::parse_from_data_str(s);
    /// println!("got qs: {}", q);
    ///
    /// assert_eq!(q.get_first("c").unwrap(), "3");
    /// assert_eq!(q.get_last("c").unwrap(), "4");
    /// q = q.delete("c").delete("b").delete("a");
    /// assert_eq!(q.get("c"), None);
    /// assert_eq!(q.get_first("c"), None);
    /// assert_eq!(q.get_last("c"), None);
    ///
    /// assert_eq!(q.get_first("a"), None);
    /// assert_eq!(q.get_first("b"), None);
    /// assert_eq!(q.get_first("d").unwrap(), "5");
    /// ```
    ///
    // pub fn delete<'b>(&'a mut self, key: &'a str) -> Option<Vec<Cow<'a, str>>> {
    pub fn delete<'b>(mut self, key: &'a str) -> Self {
        self.map.remove(&Cow::from(key));
        self
    }

    /// # Clear all (k, v) pairs
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// assert_eq!(q.len(), 5);
    /// assert_eq!(q.keys_length(), 4);
    /// let q = q.clear();
    /// assert_eq!(q.len(), 0);
    /// assert_eq!(q.get("c"), None);
    /// assert_eq!(q.get_first("c"), None);
    /// assert_eq!(q.get_last("c"), None);
    ///
    /// assert_eq!(q.get_first("a"), None);
    /// assert_eq!(q.get_first("b"), None);
    /// assert_eq!(q.get_first("d"), None);
    /// ```
    ///
    pub fn clear(mut self) -> Self {
        self.map.clear();
        self
    }

    /// # len of pairs
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// assert_eq!(q.len(), 5);
    /// assert_eq!(q.keys_length(), 4);
    /// let q = q.clear();
    /// assert_eq!(q.len(), 0);
    /// assert_eq!(q.keys_length(), 0);
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.map.values()
            .into_iter()
            .map(|v| v.len())
            .sum()
    }

    /// # length of keys
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// let s = "a=1&b=2&c=3&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// assert_eq!(q.len(), 5);
    /// assert_eq!(q.keys_length(), 4);
    /// let q = q.clear();
    /// assert_eq!(q.len(), 0);
    /// assert_eq!(q.keys_length(), 0);
    /// ```
    ///
    pub fn keys_length(&self) -> usize {
        self.map.len()
    }

    /// # length of keys
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// use std::borrow::Cow;
    /// let s = "c=3&b=2&a=1&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// let keys = q.keys();
    /// assert!(keys.contains(&"a"));
    /// assert!(keys.contains(&"b"));
    /// assert!(keys.contains(&"c"));
    /// assert!(keys.contains(&"d"));
    /// assert!(!keys.contains(&"e"));
    /// assert_eq!(q.keys_length(), 4);
    /// ```
    ///
    pub fn keys(&self) -> Vec<&str> {
        self.map.keys().map(|x| x.as_ref()).collect()
    }


    /// # keys_of_original_order
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// use std::borrow::Cow;
    /// let s = "c=3&b=2&a=1&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// let keys = q.keys();
    /// assert_eq!(q.keys_of_original_order(), vec!["c", "b", "a", "d"]);
    /// ```
    ///
    pub fn keys_of_original_order(&self) -> Vec<Cow<str>> {
        let mut ks: Vec<_> = self.map.keys().map(|x| x.clone()).collect();
        let mut elements_need_to_insert_at_front = vec![];
        for i in self.original_keys_in_order.iter() {
            if let Some(pos) = ks.iter().position(|x| x == i) {
                let item = ks.remove(pos);
                elements_need_to_insert_at_front.push(item);
            }
        }

        elements_need_to_insert_at_front.extend_from_slice(&ks);
        elements_need_to_insert_at_front
    }

    /// # keys_of_original_order
    ///
    /// # example:
    ///
    /// ```rust
    /// use url_encoded_data::UrlEncodedData;
    /// use std::borrow::Cow;
    /// let s = "c=3&b=2&a=1&c=4&d=5";
    /// let q = UrlEncodedData::parse_from_data_str(s);
    /// let keys = q.keys();
    /// assert_eq!(q.keys_of_sorted_order(), vec!["a", "b", "c", "d"]);
    /// ```
    ///
    pub fn keys_of_sorted_order(&self) -> Vec<Cow<str>> {
        let mut ks: Vec<_> = self.map.keys().map(|x|x.clone()).collect();
        ks.sort_unstable();
        ks
        // ks.iter().map(|x| x.as_ref()).collect()
    }
}

#[cfg(test)]
mod test_qs {
    use super::*;

    #[test]
    fn test_can_construct_instance() -> anyhow::Result<()> {
        let url_1 = "https://abc.com/?a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
        // let q = UrlEncodedData::try_from_full_url(url_1)?;
        // let url = url_lib::Url::parse(url_1)?;
        // let q = UrlEncodedData::from_url(&url);

        // let q = UrlEncodedData::try_from_full_url(url_1)?;
        // println!("got qs: {}", q);

        // let mut q = UrlEncodedData::from(url_1);
        let q = UrlEncodedData::parse_from_data_str(url_1);
        // let mut q = UrlEncodedData::prepare(url_1);
        // let q = q.parse();
        println!("got qs: {}", q);

        Ok(())
    }

    #[test]
    fn test_qs() -> anyhow::Result<()> {
        let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
        for s in [
            qs.as_str(),
            ("https://abc.com/?".to_string() + qs.as_str()).as_str(),
            ("https://abc.com/?????".to_string() + qs.as_str()).as_str(),
        ]
            .iter()
        {
            let q = UrlEncodedData::parse_from_data_str(s);
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

            for (k, v) in q.as_pairs().iter().map(|(k, v)| (k.as_ref(), v.as_ref())) {
                assert!(pairs_expected_as_str.contains(&(k, v)));
            }

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
                    assert_eq!(v1[i].as_ref(), *v2i);
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
        Ok(())
    }

    #[test]
    fn test_done() {
        let qs = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key";
        // let q = UrlEncodedData::parse_from_data_str(qs)
        //     .set_one("a", "100")
        //     .set_one("b", "200")
        //     .done();

        let q = UrlEncodedData::parse_from_data_str(qs)
            .set_one("a", "100")
            .set_one("b", "200");
        // let q = UrlEncodedData::parse_from_data_str(qs).set_one_by_ref_mut("a", "20").done2();
        // let q = UrlEncodedData::parse_from_data_str(qs).set_one("a", "20").done2();

        assert_eq!(q.get_first("a").unwrap(), "100");
        assert_eq!(q.get_first("b").unwrap(), "200");
    }

    #[test]
    fn test_delete() {
        let s = "a=1&b=2&c=3&c=4";
        let mut q = UrlEncodedData::parse_from_data_str(s);
        println!("got qs: {}", q);

        assert_eq!(q.get_first("c").unwrap(), "3");
        assert_eq!(q.get_last("c").unwrap(), "4");
        q = q.delete("c");
        println!("got qs: {}", q);
        assert_eq!(q.get("c"), None);
        assert_eq!(q.get_first("c"), None);
        assert_eq!(q.get_last("c"), None);
        //
        assert_eq!(q.get_first("a").unwrap(), "1");
        assert_eq!(q.get_first("b").unwrap(), "2");
        assert_eq!(q.get_first("non-exist"), None);
    }
}
