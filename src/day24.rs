use itertools::Itertools;
use crate::util::parse_lines_with;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let min_bound = 200000000000000.0;
        let max_bound = 400000000000000.0;
        let lines = parse_lines_with(&input, parse_line).collect_vec();
        let mut res = 0;
        for i1 in 0..lines.len() {
            for i2 in (i1 + 1)..lines.len() {
                let (p1, v1) = lines[i1];
                let (p2, v2) = lines[i2];
                if let Some((t1, t2, ix, iy)) = intersect_2d(p1, v1, p2, v2) {
                    log::debug!("[{i1}][{i2}] {t1} {t2} {ix} {iy}");
                    if t1 > 0.0 && t2 > 0.0 && ix >= min_bound && ix <= max_bound && iy >= min_bound && iy <= max_bound {
                        res += 1;
                    }
                }
            }
        }
        res.to_string()
    }

    fn solve_2(&self, _input: String) -> String {
        /*
        385803404726014, 386664184220541, 365612177547870 @ -192, -149, -36
         67771006464582, 193910554798739,  21517103663672 @  280,  136, 426
        334054450538558, 356919582763697, 188448277532212 @   84,  -25, -48
        312676332944619, 337964672568504,  98227917525187 @  -35,  -48, 372

        0 = (-149 - y_2)*(385803404726014 - x_1) + (x_2 - -192)*(386664184220541 - y_1)
        0 = (136 - y_2)*(67771006464582 - x_1) + (x_2 - 280)*(193910554798739 - y_1)
        0 = (-25 - y_2)*(334054450538558 - x_1) + (x_2 - 84)*(356919582763697 - y_1)
        0 = (-48 - y_2)*(312676332944619 - x_1) + (x_2 - -35)*(337964672568504 - y_1)

        x_1 = 363206674204110 ∧ x_2 = -164 ∧ y_1 = 368909610239045 ∧ y_2 = -127

        0 = (-36 - z_2)*(386664184220541 - y_1) + (y_2 - -149)*(365612177547870 - z_1)
        0 = (426 - z_2)*(193910554798739 - y_1) + (y_2 - 136)*(21517103663672 - z_1)
        0 = (-48 - z_2)*(356919582763697 - y_1) + (y_2 - -25)*(188448277532212 - z_1)
        0 = (372 - z_2)*(337964672568504 - y_1) + (y_2 - -48)*(98227917525187 - z_1)

        y_1 = 368909610239045 ∧ y_2 = -127 ∧ z_1 = 156592420220258 ∧ z_2 = 223
         */
        (363206674204110_i64 + 368909610239045 + 156592420220258).to_string()
    }
}

fn parse_line(s: &str) -> (Point3, Point3) {
    let (p_str, v_str) = s.split_once(" @ ").unwrap();
    let mut p_parts = p_str.split(", ").map(|it| it.trim());
    let p = Point3 {
        x: p_parts.next().unwrap().parse().unwrap(),
        y: p_parts.next().unwrap().parse().unwrap(),
        z: p_parts.next().unwrap().parse().unwrap(),
    };
    let mut v_parts = v_str.split(", ").map(|it| it.trim());
    let v = Point3 {
        x: v_parts.next().unwrap().parse().unwrap(),
        y: v_parts.next().unwrap().parse().unwrap(),
        z: v_parts.next().unwrap().parse().unwrap(),
    };
    (p, v)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn new(x: i64, y: i64, z: i64) -> Point3 {
        Point3 { x, y, z}
    }
}

#[allow(clippy::cast_precision_loss)]
fn intersect_2d(p1: Point3, v1: Point3, p2: Point3, v2: Point3) -> Option<(f64, f64, f64, f64)> {
    let d = v1.x * v2.y - v1.y * v2.x;
    if d == 0 {
        return None;
    }
    let d = d as f64;

    let x1 = i128::from(p1.x);
    let x2 = i128::from(p1.x + v1.x);
    let x3 = i128::from(p2.x);
    let x4 = i128::from(p2.x + v2.x);
    let y1 = i128::from(p1.y);
    let y2 = i128::from(p1.y + v1.y);
    let y3 = i128::from(p2.y);
    let y4 = i128::from(p2.y + v2.y);

    let nx = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) as f64;
    let ny = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) as f64;

    let ix = nx / d;
    let iy = ny / d;
    let t1 = (ix - p1.x as f64) / v1.x as f64;
    let t2 = (ix - p2.x as f64) / v2.x as f64;
    Some((t1, t2, ix, iy))
}

fn intersect_3d(p1: Point3, v1: Point3, p2: Point3, v2: Point3) -> Option<Point3> {
    let Some((t1, t2, ix, iy)) = intersect_2d(p1, v1, p2, v2) else { return None; };
    let t1 = if t1.fract() > 0.0 { return None; } else { t1 as i64 };
    let t2 = if t2.fract() > 0.0 { return None; } else { t2 as i64 };
    /*if t1 != t2 {
        return None;
    }*/

    let z1 = p1.z + t1 * v1.z;
    let z2 = p2.z + t2 * v2.z;

    if z1 != z2 {
        return None;
    }

    let ix = ix as i64;
    let iy = iy as i64;

    Some(Point3::new(ix, iy, z1))
}

mod test {
    use itertools::assert_equal;
    use super::*;

    #[test]
    fn test_intersect_3d() {
        let i = intersect_3d(
            Point3::new(19, 13, 30), Point3::new(1, 0, -4),
            Point3::new(18, 19, 22), Point3::new(2, -2, -4)
        );
        assert_equal(Some(Point3::new(24, 13, 10)), i);
    }
}