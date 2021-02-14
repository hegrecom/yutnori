use super::yut::Yut;
use std::collections::HashMap;

pub struct YutSet<'a> {
    back_do: Yut,
    normal_yut_set: Vec<Yut>,
    rank: HashMap<(usize, bool), &'a str>
}

impl<'a> YutSet<'a> {
    pub fn new() -> Self {
        let back_do = Yut::new();
        let normal_yut_set = (0..3).map(|_| Yut::new()).collect();
        let rank = [
            ((1, false), "도"),
            ((1, true), "빽도"),
            ((2, false), "개"),
            ((2, true), "개"),
            ((3, false), "걸"),
            ((3, true), "걸"),
            ((4, false), "윷"),
            ((4, true), "윷"),
            ((0, false), "모"),
            ((0, true), "모"),
        ].iter().cloned().collect();


        YutSet {
            back_do,
            normal_yut_set,
            rank
        }
    }

    pub fn throw(&self) -> &str {
        let throwed_backdo: bool = self.back_do.throw();
        let mut throwed_result: Vec<bool> = self.normal_yut_set.iter().map(|yut| yut.throw()).collect();

        throwed_result.push(throwed_backdo);
        let flipped_count = throwed_result.iter().filter(|&result| *result == true).count();

        self.rank.get(&(flipped_count, throwed_backdo)).unwrap()
    }
}
