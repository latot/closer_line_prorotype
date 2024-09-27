mod nearest;
mod value;

fn main() {
    let point = geo::Point::new(18.006691126034692, 69.04048768506776);
    let a = geo::LineString::new(vec![
        geo::Point::new(18.00678831099686, 69.0404811833497).into(),
        geo::Point::new(18.006784630996860, 69.04045431334970).into(),
        geo::Point::new(18.00677727099686, 69.0404005833497).into(),
    ]);
    let b = geo::LineString::new(vec![
        geo::Point::new(18.00677727099686, 69.0404005833497).into(),
        geo::Point::new(18.006780950996863, 69.04042744334969).into(),
        geo::Point::new(18.00678831099686, 69.0404811833497).into(),
    ]);
    let lines = vec![a, b];
    let ret = nearest::Nearest::closer_line(point, lines);
    println!("Nearest id: {}", ret.0);
    println!("Nearest distance: {}", (ret.1.num / ret.1.div).sqrt());
    println!("Nearest: {} / {}", ret.1.num.sqrt(), ret.1.div.sqrt());
}
