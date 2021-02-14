use rand::Rng;

pub struct Yut {}

impl Yut {
    pub fn new() -> Self {
        Yut {}
    }

    pub fn throw(&self) -> bool {
        let mut rng = rand::thread_rng();
        let random_value  = rng.gen_range(0..5);

        if random_value < 3 { true } else { false }
    }
}
