use crate::Solve;

pub struct Problem {
    data: Vec<i64>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm_1(self.data.as_slice(), true)
    }
    fn p2(&mut self) -> i64 {
        algorithm_1(self.data.as_slice(), false)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input[0].split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }
}

fn algorithm_1(nums: &[i64], p1: bool) -> i64 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let mut min_diff = i64::MAX;
    for i in min..=max {
        let mut distance: i64 = 0;
        for num in nums.iter() {
            if p1 {
                distance += (num - i).abs();
            } else {
                let mut cost: i64 = 1;
                if i <= *num {
                    for _walker in i..*num {
                        distance += cost;
                        cost += 1;
                    }
                } else {
                    for _walker in *num..i {
                        distance += cost;
                        cost += 1;
                    }
                }
            }
        }
        if distance < min_diff {
            min_diff = distance;
        }
    }

    min_diff
}
