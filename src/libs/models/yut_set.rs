use super::yut::Yut;

pub struct YutSet {
    back_do: Yut,
    normal_yut_set: Vec<Yut>
}

impl YutSet {
    pub fn new() -> Self {
        let back_do = Yut::new();
        let normal_yut_set = (0..3).map(|_| Yut::new()).collect();

        YutSet {
            back_do,
            normal_yut_set
        }
    }

    pub fn throw(&self) -> (usize, bool) {
        let throwed_backdo: bool = self.back_do.throw();
        let mut throwed_result: Vec<bool> = self.normal_yut_set.iter().map(|yut| yut.throw()).collect();

        throwed_result.push(throwed_backdo);
        let flipped_count = throwed_result.iter().filter(|&result| *result == true).count();

        (flipped_count, throwed_backdo)
    }
}
