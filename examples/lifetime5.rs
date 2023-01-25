use std::collections::HashMap;
use std::hash::Hash;

fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    map.entry(key).or_insert(V::default())
}

fn main() {
    let mut map = HashMap::new();
    let key = "hello";
    let value: &mut i32 = get_default(&mut map, key);
    println!("{:?}", value);
}
