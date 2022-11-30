use crate::Solve;

pub struct Problem {
    data: Vec<u32>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm_3(self.data.as_slice(), 80)
    }
    fn p2(&mut self) -> i64 {
        algorithm_3(&self.data, 256)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input[0]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .into_iter()
                .collect(),
        }
    }
}

#[allow(dead_code)]
fn algorithm_1(mut nums: Vec<u32>, max_day: i64) -> i64 {
    for _d in 0..max_day {
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums[i] = 8;
                nums.push(6);
            } else {
                nums[i] -= 1;
            }
        }
    }
    nums.len() as i64
}

#[allow(dead_code)]
fn algorithm_2(nums: Vec<u32>, max_day: u32) -> i64 {
    let mut answers: Vec<i64> = vec![0]; // 0 not used
    for i in 1..=5 {
        answers.push(count_offspring(i, 0, max_day));
    }
    let mut retval: i64 = 0;
    for num in nums {
        retval += answers[num as usize];
    }

    retval
}

#[allow(dead_code)]
fn count_offspring(age: u32, today: u32, max_day: u32) -> i64 {
    let mut retval: i64 = 1;
    let mut day: u32 = today + age + 1;

    while day <= max_day {
        retval += count_offspring(8, day, max_day);
        day += 7;
    }

    retval
}

fn algorithm_3(nums: &[u32], max_day: usize) -> i64 {
    let mut days: Vec<i64> = vec![0; max_day];
    let mut offspring: i64 = 0;

    for num in nums {
        days[*num as usize] += 1;
        offspring += 1;
    }

    for i in 0..max_day {
        offspring += days[i];
        if i + 9 < max_day {
            days[i + 9] += days[i];
        }
        if i + 7 < max_day {
            days[i + 7] += days[i];
        }
    }

    offspring
}
