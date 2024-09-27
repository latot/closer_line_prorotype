use std::ops::Mul;

use crate::value::Value;

trait Distance<A, B> {
    fn distance(a: A, b: B) -> Value;
}

pub struct Nearest {}

impl Distance<geo::Point, geo::Line> for Nearest {
    fn distance(a: geo::Point, b: geo::Line) -> Value {
        let b1 = b.start_point();
        let b2 = b.end_point();
        let vpx = b2.0.x - b1.0.x;
        let vpy = b2.0.y - b1.0.y;
        let t = Value::new(
            vpx * (a.0.x - b1.0.x) + vpy * (a.0.y - b1.0.y),
            vpx.powi(2) + vpy.powi(2),
        );
        let x = Value::new(vpx, 1.0).mul(&t).sum(&Value::new(b1.0.x, 1.0));
        let y = Value::new(vpy, 1.0).mul(&t).sum(&Value::new(b1.0.y, 1.0));
        let valid_x = (
            Value::new(b1.0.x.min(b2.0.x), 1.0),
            Value::new(b1.0.x.max(b2.0.x), 1.0),
        );
        let valid_y = (
            Value::new(b1.0.y.min(b2.0.y), 1.0),
            Value::new(b1.0.y.max(b2.0.y), 1.0),
        );
        if x.less_than(&valid_x.1)
            && x.bigger_than(&valid_x.0)
            && y.less_than(&valid_y.1)
            && y.bigger_than(&valid_y.0)
        {
            return (x.sub(&Value::new(a.0.x, 1.0)))
                .powi(2)
                .sum(&y.sub(&Value::new(a.0.y, 1.0)).powi(2));
        }
        let d1 = (a.0.x - b1.0.x).powi(2) + (a.0.y - b1.0.y).powi(2);
        let d2 = (a.0.x - b2.0.x).powi(2) + (a.0.y - b2.0.y).powi(2);
        if d1 > d2 {
            return Value::new(d2, 1.0);
        }
        if d2 > d1 {
            return Value::new(d1, 1.0);
        }
        panic!("They are equal!");
    }
}

impl Distance<geo::Line, geo::Point> for Nearest {
    fn distance(a: geo::Line, b: geo::Point) -> Value {
        Nearest::distance(b, a)
    }
}

impl Distance<geo::Point, geo::Point> for Nearest {
    fn distance(a: geo::Point, b: geo::Point) -> Value {
        Value::new((a.0.x - b.0.x).powi(2) + (a.0.y - b.0.y).powi(2), 1.0)
    }
}

impl Nearest {
    pub fn closer_line(a: geo::Point, b: Vec<geo::LineString>) -> (usize, Value) {
        let mut distance: Vec<Value> = vec![];
        for linestring in b {
            let mut min_d: Option<Value> = None;
            for line in linestring.lines() {
                let d = Nearest::distance(a, line);
                println!("{} / {}", d.num, d.div);
                min_d = match min_d {
                    Some(x) => {
                        if d.less_than(&x) {
                            Some(d)
                        } else {
                            Some(x)
                        }
                    }
                    None => Some(d),
                }
            }
            distance.push(min_d.unwrap());
        }
        let mut id = 0;
        let mut d = &distance[0];
        for i in 1..distance.len() {
            if distance[i].less_than(d) {
                id = i;
                d = &distance[i];
            }
        }
        return (id, d.clone());
    }
}
