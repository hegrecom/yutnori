mod libs;

fn main() {
    let yut = libs::models::yut::Yut::new(false);

    println!("{}", yut.throw());
}
