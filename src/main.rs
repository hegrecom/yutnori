mod libs;
use std::collections::HashMap;

fn main() {
    let yut_set = libs::models::yut_set::YutSet::new();
    let mut result: HashMap<&str, usize> = HashMap::new();

    (0..=1000000).for_each(|_| {
        let throw = yut_set.throw();
        let counter = result.entry(throw).or_insert(0);
        *counter += 1;
    });

    println!("{:?}", result);
}
