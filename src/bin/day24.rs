struct Coord { x: f64, y: f64, z: f64 }
impl Coord {
    fn in_bounds_xy(&self, min_xy: f64, max_xy: f64) -> bool {
        self.x >= min_xy && self.x <= max_xy &&
        self.y >= min_xy && self.y <= max_xy
    }

    fn get(&self, dim: &Dimension) -> f64 {
        match dim {
            Dimension::X => self.x,
            Dimension::Y => self.y,
            Dimension::Z => self.z,
        }
    }
}

enum Dimension { X, Y, Z }

struct Hailstone {
    pos: Coord,
    vel: Coord,
}
impl Hailstone {
    fn parse(line: &str) -> Hailstone {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let pos = pos.split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let pos = Coord { x: pos[0], y: pos[1], z: pos[2] };
        let vel = vel.split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let vel = Coord { x: vel[0], y: vel[1], z: vel[2] };
        Hailstone { pos, vel }
    }

    fn point_on_path_is_future(&self, point: &Coord) -> bool {
        (point.x - self.pos.x) / self.vel.x > 0.0
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.lines().map(Hailstone::parse).collect()
}

fn num_collisions_xy(hailstones: &[Hailstone], min_xy: f64, max_xy: f64) -> usize {
    let mut count = 0;
    for i in 0..(hailstones.len() - 1) {
        for j in (i+1)..hailstones.len() {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            let collision_point = collision_xy(h1, h2);
            if let Some(collision_point) = collision_point {
                if collision_point.in_bounds_xy(min_xy, max_xy) &&
                    h1.point_on_path_is_future(&collision_point) &&
                    h2.point_on_path_is_future(&collision_point) {
                      count += 1;
                }
            }
        }
    }
    count
}

// Determine whether the paths of the two hailstones (ignoring z) intersect.
fn collision_xy(h1: &Hailstone, h2: &Hailstone) -> Option<Coord> {
    let m1 = h1.vel.y / h1.vel.x;
    let m2 = h2.vel.y / h2.vel.x;

    if (m1 - m2).abs() < f64::EPSILON {
        return None;
    }

    let x = (h2.pos.y - h1.pos.y + h1.pos.x*m1 - h2.pos.x*m2) / (m1 - m2);
    let y = m1 * (x - h1.pos.x) + h1.pos.y;

    Some(Coord { x, y, z: 0.0 })
}

fn solve_for(hailstones: &[Hailstone], dim_a: &Dimension, dim_b: &Dimension) -> Vec<Vec<f64>> {
    let mut equations = hailstones.iter().map(|hailstone| {
        vec![
            1.0,
            -1.0 * hailstone.pos.get(dim_a),
            hailstone.vel.get(dim_a),
            hailstone.pos.get(dim_b),
            -1.0 * hailstone.vel.get(dim_b),
            hailstone.pos.get(dim_b) * hailstone.vel.get(dim_a) - hailstone.pos.get(dim_a) * hailstone.vel.get(dim_b)
        ]
    }).collect::<Vec<_>>();

    for i in 0..(equations.len().min(equations[0].len() - 1)) {
        let to_unit = equations[i][i];
        for j in 0..equations[i].len() {
            equations[i][j] /= to_unit;
        }

        for j in (i+1)..equations.len() {
            let b = &equations[j];
            let factor = b[i] / equations[i][i];
            for k in 0..b.len() {
                equations[j][k] -= equations[i][k] * factor;
            }
        }
    }

    equations
}

fn part2(hailstones: &[Hailstone]) -> f64 {
    let equations_xy = solve_for(hailstones, &Dimension::Y, &Dimension::X);
    let rock_y = equations_xy[4].iter().last().unwrap().round();

    let equations_yx = solve_for(hailstones, &Dimension::X, &Dimension::Y);
    let rock_x = equations_yx[4].iter().last().unwrap().round();

    let equations_yz = solve_for(hailstones, &Dimension::Z, &Dimension::Y);
    let rock_z = equations_yz[4].iter().last().unwrap().round();

    rock_x + rock_y + rock_z
}

fn main() {
    let input = include_str!("../../input/day24");
    let hailstones = parse(input);
    println!("Part 1: {}", num_collisions_xy(&hailstones, 200000000000000_f64, 400000000000000_f64));
    println!("Part 2: {}", part2(&hailstones));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part_1() {
        let hailstones = parse(EXAMPLE);
        assert_eq!(num_collisions_xy(&hailstones, 7f64, 27f64), 2);
    }
}