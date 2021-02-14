mod libs;

fn main() {
    let yut_set = libs::models::yut_set::YutSet::new();

    println!("{:?}", yut_set.throw());
}
