use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

pub fn a(input: &str) {
    let target = parse_input(input);

    // x dimension can be ignored for now
    // whatever the initial positive y vel is, it will be equal, but negative, when
    // the probe crosses the initial y pos on the way down. Therefore, the highest y
    // vel is equal to the furthest y point on the target, minus one.
    let max_y_vel = -min(target.y0, target.y1) - 1;

    // see how high it gets
    let mut apex = 0;
    let mut curr = 0;
    let mut vel = max_y_vel;
    while curr >= apex {
        apex = curr;
        curr += vel;
        vel -= 1;
    }
    println!("{}", apex);
}
pub fn b(input: &str) {
    let target = parse_input(input);
    // given start position 0 and target position S
    // we can always reach S in one leap with x_vel = S
    // lower velocities could reach S in n steps if the
    // sum of de-/increasing velocities for n steps is
    // equal to S.
    //
    // Given a starting position s and a starting speed v0, each step will
    // substract 1 from v. Therefore, the position after n steps can
    // be expressed as
    // S = s0 + v0 + v0 - 1 + v0 - 2 + ... + v0 - n
    // S = s0 + n * v0 - (1 + 2 + ... + n)
    // S = s0 + n * v0 - n(n+1) / 2 (triangle number)

    // one dimension at a time, find initial velocities that reach the target area
    // given the parameters and store their values along with the number of steps it took.
    fn step_to(t_min: i32, t_max: i32, v_min: i32) -> HashSet<(i32, i32)> {
        let mut set = HashSet::new();
        let mut s = 0;
        let mut n = 0;
        let mut v0 = v_min;
        let mut v = v0;
        let v_max = t_min.abs().max(t_max.abs());
        while v0 <= v_max {
            s += v;
            n += 1;
            v -= 1;
            if s >= t_min && s <= t_max {
                set.insert((v0, n));
            }
            if v <= v_min {
                s = 0;
                n = 0;
                v0 += 1;
                v = v0;
            }
        }

        set
    }
    let mut x_vels = step_to(target.x0, target.x1, 0);
    let mut y_vels = step_to(target.y0, target.y1, target.y0);

    // println!("x_vel_map: \n{:?}\n", x_vels);
    // println!("y_vel_map: \n{:?}\n", y_vels);

    let mut velocities = HashSet::new();
    for (x_vel, x_steps) in &x_vels {
        y_vels
            .iter()
            .filter(|(_, y_steps)| {
                if x_vel == x_steps {
                    // x_vel reached 0 at this position and therefore
                    // any y_vel with >= steps can reach this point.
                    y_steps >= x_steps
                } else {
                    y_steps == x_steps
                }
            })
            .for_each(|(y_vel, _)| {
                velocities.insert((x_vel, y_vel));
            });
    }

    println!("{}", velocities.len());
}

fn velocities_for_pos(s: i32) -> Vec<i32> {
    // given a target position s, returns the number of velocities that
    // reach it exactly.
    todo!()
}

fn vel_for_pos(s: i32, steps: i32) -> i32 {
    (s - (-steps * steps) / 2) * steps
}

fn vel_at_step(v: i32, step: i32) -> i32 {
    // v0 + at
    // with a = -1
    v + -step
}

fn pos_at_step(v0: i32, step: i32) -> i32 {
    // s = s0 + v0t + (at^2)/2
    // with s0 = 0; a = -1
    v0 * step + (-step * step) / 2
}

fn parse_input(input: &str) -> Target {
    let xy_strings: Vec<_> = input
        .trim_end()
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .collect();
    let x_range = parse_range(xy_strings[0], "x=");
    let y_range = parse_range(xy_strings[1], "y=");

    Target {
        x0: x_range[0],
        y0: y_range[0],
        x1: x_range[1],
        y1: y_range[1],
    }
}

fn parse_range(s: &str, prefix: &str) -> Vec<i32> {
    s.strip_prefix(prefix)
        .unwrap()
        .split("..")
        .map(|a| {
            if let Some(neg) = a.strip_prefix('-') {
                -neg.parse::<i32>().unwrap()
            } else {
                a.parse::<i32>().unwrap()
            }
        })
        .collect()
}

struct Target {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
