use crate::Solve;

pub struct Problem {
    data: Target,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm1(&self.data)
    }
    fn p2(&mut self) -> i64 {
        algorithm2(&self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: parse_input(&input[0]),
        }
    }
}

#[derive(Debug)]
struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

#[allow(dead_code)]
impl Target {
    fn in_target(&self, x: i64, y: i64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    fn past_target(&self, x: i64, y: i64) -> bool {
        x >= self.x_max || y <= self.y_min
    }
}

fn parse_input(input: &str) -> Target {
    let mut tmp: Vec<String> = input.split("x=").map(str::to_string).collect();
    tmp = tmp[1].split("..").map(str::to_string).collect();
    let x1: i64 = tmp[0].parse().unwrap();
    let y2: i64 = tmp[2].parse().unwrap();
    tmp = tmp[1].split(',').map(str::to_string).collect();
    let x2: i64 = tmp[0].parse().unwrap();
    tmp = tmp[1].split("y=").map(str::to_string).collect();
    let y1: i64 = tmp[1].parse().unwrap();

    Target {
        x_min: std::cmp::min(x1, x2),
        x_max: std::cmp::max(x1, x2),
        y_min: std::cmp::min(y1, y2),
        y_max: std::cmp::max(y1, y2),
    }
}

fn shoot(x_speed: i64, y_speed: i64, t: &Target) -> bool {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut hit;
    let mut cur_x_speed = x_speed;
    let mut y_speed_cur = y_speed;

    loop {
        x += cur_x_speed;
        y += y_speed_cur;
        hit = t.in_target(x, y);
        if t.past_target(x, y) || hit {
            break;
        }

        match cur_x_speed.cmp(&0) {
            std::cmp::Ordering::Less => cur_x_speed += 1,
            std::cmp::Ordering::Greater => cur_x_speed -= 1,
            std::cmp::Ordering::Equal => (),
        }
        y_speed_cur -= 1;
    }
    hit
}

fn algorithm1(t: &Target) -> i64 {
    let mut max_steps: i64 = 0;
    let mut min_steps: i64 = 0;
    let mut x_inc = 1;
    let mut height = 0;
    if i64::abs(t.x_max) > i64::abs(t.x_min) {
        let mut x_pos = 0;
        loop {
            x_pos += x_inc;
            if x_pos > t.x_max {
                break;
            }
            max_steps += 1;
            x_inc += 1;
        }

        x_pos = 0;
        x_inc = 1;
        loop {
            x_pos += x_inc;
            min_steps += 1;
            x_inc += 1;
            if x_pos >= t.x_min {
                break;
            }
        }

        for x in min_steps..=max_steps {
            if x % 2 == 0 {
                for y in (t.y_min + 1)..0 {
                    height += -y;
                }
            }
        }
    }

    height
}

fn algorithm2(t: &Target) -> i64 {
    let mut retval = 0;

    // Direct shots
    retval += (t.x_max - t.x_min + 1) * -(t.y_min - t.y_max - 1);

    // Min x
    let mut min_steps: i64 = 0;
    let mut x_pos = 0;
    let mut x_inc = 1;
    loop {
        x_pos += x_inc;
        min_steps += 1;
        x_inc += 1;
        if x_pos >= t.x_min {
            break;
        }
    }

    // Max X
    let max_steps: i64 = if t.x_max % 2 == 0 {
        t.x_max / 2
    } else {
        t.x_max / 2 + 1
    };

    for x in min_steps..=max_steps {
        for y in (t.y_min / 2)..100 {
            if shoot(x, y, t) {
                retval += 1;
            }
        }
    }

    retval
}
