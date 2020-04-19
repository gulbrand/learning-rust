use std::collections::HashMap;

pub fn main() {
    println!("hash_map_test");
    let mut map: HashMap<i32, i32> = HashMap::new();
    let key = 1;
    let value = 2;
    map.insert(key, value);
    println!("map = {:?}", map);
    if let Some(v) = map.get(&key) {
        if v == &2 {
            println!("v == 2");
        }
        if *v == 2 {
            println!("v == 2");
        }
        println!("v = {}", v);
    } else {
        println!("k = {} not found", key);
    }

    map.entry(key)
        .and_modify(|v| { *v += 1 })
        .or_insert(10);
    map.entry(10)
        .and_modify(|v| { *v += 1 })
        .or_insert(10);
    println!("map = {:?}", map);
}