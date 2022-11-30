use crate::Solve;

pub struct Problem {
    data: Vec<i64>,
}
impl Solve for Problem {
    /// Count where item is greater than last
    fn p1(&mut self) -> i64 {
        count_greater_by_distance(1, &self.data)
    }
    /// Count where sum of 3 is later than last sum of 3
    fn p2(&mut self) -> i64 {
        count_greater_by_distance(3, &self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.iter().flat_map(|s| s.parse()).collect(),
        }
    }
}

fn count_greater_by_distance(dist: usize, data: &[i64]) -> i64 {
    let mut result = 0;
    for i in dist..data.len() {
        if data[i] > data[i - dist] {
            result += 1;
        }
    }
    result
}
