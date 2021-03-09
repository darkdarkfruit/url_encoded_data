#[macro_use]
extern crate maplit;

use std::borrow::Cow;
use url_encoded_data::UrlEncodedData;

fn simple_example() -> anyhow::Result<()> {
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
    Ok(())
}

fn main() {
    simple_example().unwrap();
}
