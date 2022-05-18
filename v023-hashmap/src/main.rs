use std::collections::HashMap;

fn main() {
    let mut h: HashMap<u32, String> = HashMap::new();
    h.insert(1, "value1".to_string());
    h.insert(2, "value2".to_string());
    h.insert(3, "value3".to_string());
    h.insert(4, "value4".to_string());
    h.insert(5, "value5".to_string());

    h.remove(&3);
    let m: Option<&String> = h.get(&4);
    println!("{}", m.unwrap());
    println!("len:{} cap:{}", h.len(), h.capacity());
    for (k, v) in &h {
        println!("{}:{}", k, v);
    }

    let solar_distance: HashMap<&str, f64> = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
}
