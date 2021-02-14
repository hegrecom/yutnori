use rand::Rng;

#[allow(dead_code)]
pub struct Yut {
    is_backdo: bool
}

impl Yut {
    pub fn new(is_backdo: bool) -> Self {
        Yut { is_backdo }
    }

    pub fn throw(self) -> bool {
        let mut rng = rand::thread_rng();
        let random_value  = rng.gen_range(0..5);

        if random_value < 3 { true } else { false }
    }
}
