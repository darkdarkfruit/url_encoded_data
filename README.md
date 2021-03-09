# url_encoded_data

## Url Encoded Data manipulation
Manipulate data of `application/x-www-form-urlencoded` format,
eg:
    * query_string of a url
    * http content-type with: `application/x-www-form-urlencoded`

## Terminology
* Pair: a (key, format) tuple, `(Cow<'a, str>, Cow<'a, str>)`
* url encoded string: a string which is encoded by standards of `application/x-www-form-urlencoded`

## Basic apis

### strigify: Stringify pairs to url encoded String

#### example 1
```rust
use url_encoded_data::*;
use url_encoded_data::stringify;
let encoded = stringify(&[("a", "b"), ("c", "d")]);
assert_eq!(encoded, "a=b&c=d");
```

#### example 2
```rust
use url_encoded_data::*;
use url_encoded_data::stringify;
let encoded = stringify(&[("hello", "你好"), ("world", "世界")]);
assert_eq!(encoded, "hello=%E4%BD%A0%E5%A5%BD&world=%E4%B8%96%E7%95%8C");
```


### UrlEncodedDataPairIterator: **Lazy** iterator yielding pairs
#### example:

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
    let q = UrlEncodedDataPairIterator::from(*s);
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

### UrlEncodedData: parse url_encoded_data to pairs eagerly
#### main methods:
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

assert_eq!(ued.get_multiple_values("a").unwrap(), vec!["1".to_string(), "3".to_string()]);

// get first occurrence value
assert_eq!(ued.get_first_occurrence_value("a").unwrap(), "1".to_string());

// get last occurrence value
assert_eq!(ued.get_last_occurrence_value("a").unwrap(), "3".to_string());

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
        let q = UrlEncodedData::parse_from_str(s);
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
                assert_eq!(v1[i], v2i);
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



