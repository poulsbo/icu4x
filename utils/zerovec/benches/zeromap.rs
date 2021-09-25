// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};

use zerovec::ZeroMap;

const DATA: [(&str, &str); 16] = [
    ("ar", "Arabic"),
    ("bn", "Bangla"),
    ("ccp", "Chakma"),
    ("chr", "Cherokee"),
    ("el", "Greek"),
    ("en", "English"),
    ("eo", "Esperanto"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("iu", "Inuktitut"),
    ("ja", "Japanese"),
    ("ru", "Russian"),
    ("sr", "Serbian"),
    ("th", "Thai"),
    ("tr", "Turkish"),
    ("zh", "Chinese"),
];

const POSTCARD: [u8; 282] = [
    102, 16, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 7, 0, 0, 0, 10, 0, 0, 0, 12, 0, 0, 0, 14,
    0, 0, 0, 16, 0, 0, 0, 18, 0, 0, 0, 20, 0, 0, 0, 22, 0, 0, 0, 24, 0, 0, 0, 26, 0, 0, 0, 28, 0,
    0, 0, 30, 0, 0, 0, 32, 0, 0, 0, 97, 114, 98, 110, 99, 99, 112, 99, 104, 114, 101, 108, 101,
    110, 101, 111, 101, 115, 102, 114, 105, 117, 106, 97, 114, 117, 115, 114, 116, 104, 116, 114,
    122, 104, 177, 1, 16, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 12, 0, 0, 0, 18, 0, 0, 0, 26, 0, 0, 0,
    31, 0, 0, 0, 38, 0, 0, 0, 47, 0, 0, 0, 54, 0, 0, 0, 60, 0, 0, 0, 69, 0, 0, 0, 77, 0, 0, 0, 84,
    0, 0, 0, 91, 0, 0, 0, 95, 0, 0, 0, 102, 0, 0, 0, 65, 114, 97, 98, 105, 99, 66, 97, 110, 103,
    108, 97, 67, 104, 97, 107, 109, 97, 67, 104, 101, 114, 111, 107, 101, 101, 71, 114, 101, 101,
    107, 69, 110, 103, 108, 105, 115, 104, 69, 115, 112, 101, 114, 97, 110, 116, 111, 83, 112, 97,
    110, 105, 115, 104, 70, 114, 101, 110, 99, 104, 73, 110, 117, 107, 116, 105, 116, 117, 116, 74,
    97, 112, 97, 110, 101, 115, 101, 82, 117, 115, 115, 105, 97, 110, 83, 101, 114, 98, 105, 97,
    110, 84, 104, 97, 105, 84, 117, 114, 107, 105, 115, 104, 67, 104, 105, 110, 101, 115, 101,
];

const POSTCARD_HASHMAP: [u8; 176] = [
    16, 2, 114, 117, 7, 82, 117, 115, 115, 105, 97, 110, 3, 99, 99, 112, 6, 67, 104, 97, 107, 109,
    97, 3, 99, 104, 114, 8, 67, 104, 101, 114, 111, 107, 101, 101, 2, 116, 114, 7, 84, 117, 114,
    107, 105, 115, 104, 2, 116, 104, 4, 84, 104, 97, 105, 2, 106, 97, 8, 74, 97, 112, 97, 110, 101,
    115, 101, 2, 101, 115, 7, 83, 112, 97, 110, 105, 115, 104, 2, 101, 111, 9, 69, 115, 112, 101,
    114, 97, 110, 116, 111, 2, 122, 104, 7, 67, 104, 105, 110, 101, 115, 101, 2, 115, 114, 7, 83,
    101, 114, 98, 105, 97, 110, 2, 101, 110, 7, 69, 110, 103, 108, 105, 115, 104, 2, 105, 117, 9,
    73, 110, 117, 107, 116, 105, 116, 117, 116, 2, 102, 114, 6, 70, 114, 101, 110, 99, 104, 2, 98,
    110, 6, 66, 97, 110, 103, 108, 97, 2, 101, 108, 5, 71, 114, 101, 101, 107, 2, 97, 114, 6, 65,
    114, 97, 98, 105, 99,
];

/// Run this function to print new data to the console. Requires the optional `serde` feature.
#[allow(dead_code)]
fn generate(map: &ZeroMap<String, String>) {
    let buf = postcard::to_stdvec(&map).unwrap();
    println!("{:?}", buf);
}

/// Run this function to print new data to the console. Requires the optional `serde` feature.
#[allow(dead_code)]
fn generate_hashmap(map: &HashMap<String, String>) {
    let buf = postcard::to_stdvec(&map).unwrap();
    println!("{:?}", buf);
}

fn overview_bench(c: &mut Criterion) {
    let map = build_zeromap(false);
    c.bench_function("zeromap/read", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu"), Some("Inuktitut"));
            assert_eq!(map.get("zz"), None);
        });
    });

    // Uncomment the following line to re-generate the binary data.
    // generate(&map);

    bench_serialization(c);
    bench_deserialization(c);

    bench_large(c);
    bench_large_deserialized(c);

    bench_hashmap(c);
}

fn build_zeromap(large: bool) -> ZeroMap<'static, String, String> {
    let mut map: ZeroMap<String, String> = ZeroMap::new();
    for (key, value) in DATA.iter() {
        if large {
            for n in 0..65536 {
                map.insert(format!("{}{}", key, n), value.to_string());
            }
        } else {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

fn bench_large(c: &mut Criterion) {
    let map = build_zeromap(true);
    c.bench_function("zeromap/read/1m", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some("Inuktitut"));
            assert_eq!(map.get("zz"), None);
        });
    });
}

fn bench_serialization(c: &mut Criterion) {
    let map = build_zeromap(false);
    c.bench_function("zeromap/serialize", |b| {
        b.iter(|| {
            let buf = postcard::to_stdvec(&map).unwrap();
            assert_eq!(buf.len(), POSTCARD.len());
        })
    });
}

fn bench_deserialization(c: &mut Criterion) {
    c.bench_function("zeromap/deserialize", |b| {
        b.iter(|| {
            let map: ZeroMap<String, String> = postcard::from_bytes(&POSTCARD).unwrap();
            assert_eq!(map.get("iu"), Some("Inuktitut"));
        })
    });
}

fn bench_large_deserialized(c: &mut Criterion) {
    let original_map = build_zeromap(true);
    let buf = postcard::to_stdvec(&original_map).unwrap();
    let map: ZeroMap<String, String> = postcard::from_bytes(&buf).unwrap();
    c.bench_function("zeromap/read/1m/deseralized", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some("Inuktitut"));
            assert_eq!(map.get("zz"), None);
        });
    });
}

fn bench_hashmap(c: &mut Criterion) {
    let map = build_hashmap(false);
    c.bench_function("zeromap/read/hashmap", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu"), Some(&"Inuktitut".to_string()));
            assert_eq!(map.get("zz"), None);
        });
    });

    // Uncomment the following line to re-generate the binary data.
    // generate_hashmap(&map);
    bench_serialization_hashmap(c);
    bench_deserialization_hashmap(c);

    bench_large_hashmap(c);
    bench_large_deserialized_hashmap(c);
}

fn build_hashmap(large: bool) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (key, value) in DATA.iter() {
        if large {
            for n in 0..65536 {
                map.insert(format!("{}{}", key, n), value.to_string());
            }
        } else {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

fn bench_large_hashmap(c: &mut Criterion) {
    let map = build_hashmap(true);
    c.bench_function("zeromap/read/1m/hashmap", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some(&"Inuktitut".to_string()));
            assert_eq!(map.get("zz"), None);
        });
    });
}

fn bench_serialization_hashmap(c: &mut Criterion) {
    let map = build_hashmap(false);
    c.bench_function("zeromap/serialize/hashmap", |b| {
        b.iter(|| {
            let buf = postcard::to_stdvec(&map).unwrap();
            assert_eq!(buf.len(), POSTCARD_HASHMAP.len());
        })
    });
}

fn bench_deserialization_hashmap(c: &mut Criterion) {
    c.bench_function("zeromap/deserialize/hashmap", |b| {
        b.iter(|| {
            let map: HashMap<String, String> = postcard::from_bytes(&POSTCARD_HASHMAP).unwrap();
            assert_eq!(map.get("iu"), Some(&"Inuktitut".to_string()));
        })
    });
}

fn bench_large_deserialized_hashmap(c: &mut Criterion) {
    let original_map = build_hashmap(true);
    let buf = postcard::to_stdvec(&original_map).unwrap();
    let map: HashMap<String, String> = postcard::from_bytes(&buf).unwrap();
    c.bench_function("zeromap/read/1m/deseralized/hashmap", |b| {
        b.iter(|| {
            assert_eq!(map.get("iu33333"), Some(&"Inuktitut".to_string()));
            assert_eq!(map.get("zz"), None);
        });
    });
}

criterion_group!(benches, overview_bench);
criterion_main!(benches);
