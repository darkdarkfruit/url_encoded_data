//! # Ergonomic, Versatile Url Encoded Data Manipulator
//! Manipulate data of `application/x-www-form-urlencoded` format,
//! eg:
//!     * query_string of a url
//!     * http content-type with: `application/x-www-form-urlencoded`
//!
//! # Terminology
//! * Pair: a (key, format) tuple, `(Cow<'a, str>, Cow<'a, str>)`
//! * url encoded string: a string which is encoded by standards of `application/x-www-form-urlencoded`
//!
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
//! assert_eq!(ued.get_multiple_values("a").unwrap(), vec!["1".to_string(), "3".to_string()]);
//!
//! // get first occurrence value
//! assert_eq!(ued.get_first_occurrence_value("a").unwrap(), "1".to_string());
//!
//! // get last occurrence value
//! assert_eq!(ued.get_last_occurrence_value("a").unwrap(), "3".to_string());
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
//!         for (i, (k, v)) in q.as_pairs().iter().enumerate() {
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
//!                 assert_eq!(v1[i], v2i);
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
    pub raw: &'a str,
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
        write!(f, "{}", self.raw)
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
        let raw = extract_url_encoded_string(s);
        let pairs_iterator = url_lib::form_urlencoded::parse(raw.as_bytes());
        Self {
            raw,
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
    // un-escaped raw data string extracted from input
    pub data_str: &'a str,

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
        write!(f, "{}, {:?}", self.data_str, self.map)
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
        let data_str = extract_url_encoded_string(s);
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

        Self { data_str, map, original_keys_in_order }
    }

    /// # As pairs slice
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
    ///
    ///     for (i, (k, v)) in q.as_pairs().iter().enumerate() {
    ///         let (k_, v_) = pairs_expected_as_str[i];
    ///         assert_eq!(k.as_ref(), k_);
    ///         assert_eq!(v.as_ref(), v_);
    ///     }
    /// }
    /// ```
    ///
    /// ## An example of decoding:
    /// ```rust
    /// use url_encoded_data::*;
    /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
    /// let qs = UrlEncodedData::from(s);
    /// let str_vec: Vec<_> = qs.as_pairs().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
    /// assert_eq!(str_vec[0], ("hello", "你好"));
    /// assert_eq!(str_vec[1], ("world", "世界"));
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
       /// let s = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
       /// let q = UrlEncodedData::parse_from_data_str(s);
       /// // let mut q = UrlEncodedData::prepare(url_1);
       /// // let q = q.parse();
       /// println!("got qs: {}", q);
       ///
       /// let pairs_expected_as_str = [
       ///     ("a", "1"),
       ///     ("b", "2"),
       ///     ("c", "3"),
       ///     ("c", "4"),
       ///     ("key_without_value", ""),
       ///     ("", "value_without_key"),
       /// ];
       ///
       ///
       /// for (i, (k, v)) in q.as_pairs().iter().enumerate() {
       ///     let (k_, v_) = pairs_expected_as_str[i];
       ///     assert_eq!(k.as_ref(), k_);
       ///     assert_eq!(v.as_ref(), v_);
       /// }
       /// }
       /// ```
       ///
       /// ## An example of decoding:
       /// ```rust
       /// use url_encoded_data::*;
       /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
       /// let qs = UrlEncodedData::from(s);
       /// let str_vec: Vec<_> = qs.as_pairs().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
       /// assert_eq!(str_vec[0], ("hello", "你好"));
       /// assert_eq!(str_vec[1], ("world", "世界"));
       /// ```
       // todo: found a better a way to return Vec<RefPair>, but i have no time to do it now.
    pub fn as_pairs_of_original_order(&'a self) -> Vec<Pair<'a>> {
        let mut vector = vec![];
        let mut clone = self.map.clone();
        for key in self.original_keys_in_order.iter() {
            if let Some(v) = clone.remove(key) {
                for i in v {
                    vector.push((key.clone(), i));
                }
            }
        }

        // push left pairs
        for (k, v) in clone.iter() {
            for i in v.iter() {
                vector.push((k.clone(), i.clone()));
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
   /// let s = "a=1&b=2&c=3&c=4&key_without_value&=value_without_key".to_string();
   /// let q = UrlEncodedData::parse_from_data_str(s);
   /// // let mut q = UrlEncodedData::prepare(url_1);
   /// // let q = q.parse();
   /// println!("got qs: {}", q);
   ///
   /// let pairs_expected_as_str = [
   ///     ("a", "1"),
   ///     ("b", "2"),
   ///     ("c", "3"),
   ///     ("c", "4"),
   ///     ("key_without_value", ""),
   ///     ("", "value_without_key"),
   /// ];
   ///
   ///
   /// for (i, (k, v)) in q.as_pairs().iter().enumerate() {
   ///     let (k_, v_) = pairs_expected_as_str[i];
   ///     assert_eq!(k.as_ref(), k_);
   ///     assert_eq!(v.as_ref(), v_);
   /// }
   /// }
   /// ```
   ///
   /// ## An example of decoding:
   /// ```rust
   /// use url_encoded_data::*;
   /// let s = "hello=%e4%bd%a0%e5%a5%bd&world=%e4%b8%96%e7%95%8c";
   /// let qs = UrlEncodedData::from(s);
   /// let str_vec: Vec<_> = qs.as_pairs().into_iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
   /// assert_eq!(str_vec[0], ("hello", "你好"));
   /// assert_eq!(str_vec[1], ("world", "世界"));
   /// ```
    pub fn as_pairs_of_sorted_order(&'a self) -> Vec<RefPair<'a>> {
        let mut vector = vec![];

        let mut keys_in_sorted_order: Vec<_> = self.map.keys().collect();
        keys_in_sorted_order.sort_unstable();

        for (k, v) in self.map.iter() {
            for i in v.iter() {
                vector.push((k, i));
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
    ///
    ///     for (i, (k, v)) in q.as_string_pairs().iter().enumerate() {
    ///         let (k_, v_) = pairs_expected_as_str[i];
    ///         assert_eq!(k.as_str(), k_);
    ///         assert_eq!(v.as_str(), v_);
    ///     }
    /// }
    /// ```
    pub fn as_string_pairs(&'a self) -> Vec<(String, String)> {
        self.as_pairs()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
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
    ///                 assert_eq!(v1[i], v2i);
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
    ///         assert_eq!(q.get_multiple_values("a").unwrap(), vec!["1"]);
    ///         assert_eq!(q.get_multiple_values("c").unwrap(), vec!["3", "4"]);
    ///         assert_eq!(q.get_multiple_values("non-exist"), None);
    ///     }
    /// }
    /// ```
    pub fn get_multiple_values<'b>(&'a self, key: &'b str) -> Option<Vec<String>> {
        Some(
            self.as_map_of_single_key_to_multiple_values()
                .get(&Cow::from(key))?
                .iter()
                .fold(Vec::new(), |mut acc, v| {
                    acc.push(v.to_string());
                    acc
                }),
        )
    }

    /// # Get first occurrence value by key
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
    ///         assert_eq!(q.get_first_occurrence_value("a").unwrap(), "1");
    ///         assert_eq!(q.get_first_occurrence_value("c").unwrap(), "3");
    ///         assert_eq!(q.get_first_occurrence_value("non-exist"), None);
    ///     }
    /// }
    /// ```
    pub fn get_first_occurrence_value<'b>(&'a self, key: &'b str) -> Option<String> {
        self.as_map_of_single_key_to_first_occurrence_value()
            .get(&Cow::from(key))
            .map(|v| v.to_string())
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
    ///         assert_eq!(q.get_last_occurrence_value("a").unwrap(), "1");
    ///         assert_eq!(q.get_last_occurrence_value("c").unwrap(), "4");
    ///         assert_eq!(q.get_last_occurrence_value("non-exist"), None);
    ///     }
    /// }
    /// ```
    pub fn get_last_occurrence_value<'b>(&'a self, key: &'b str) -> Option<String> {
        self.as_map_of_single_key_to_last_occurrence_value()
            .get(&Cow::from(key))
            .map(|v| v.to_string())
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

            for (i, (k, v)) in q.as_pairs().iter().enumerate() {
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
}
