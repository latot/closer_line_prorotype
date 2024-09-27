#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub num: f64,
    pub div: f64,
}

impl Value {
    pub fn default() -> Self {
        Value { num: 1.0, div: 1.0 }
    }
    pub fn new(num: f64, div: f64) -> Self {
        Value { num: num, div: div }
    }
    pub fn sum(&self, b: &Self) -> Self {
        Value::new(self.num * b.div + b.num * self.div, self.div * b.div)
    }
    pub fn mul(&self, b: &Self) -> Self {
        Value::new(self.num * b.num, self.div * b.div)
    }
    pub fn sub(&self, b: &Self) -> Self {
        self.sum(&Value::new(b.num * -1.0, b.div))
    }
    pub fn less_than(&self, b: &Self) -> bool {
        (b.num * self.div - self.num * b.div) >= 0.0
    }
    pub fn bigger_than(&self, b: &Self) -> bool {
        (self.num * b.div - b.num * self.div) >= 0.0
    }
    pub fn powi(&self, s: i32) -> Value {
        Value::new(self.num.powi(s), self.div.powi(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        assert_eq!(
            Value::new(1.0, 1.0).sum(&Value::new(1.0, 1.0)),
            Value::new(2.0, 1.0)
        );
        assert_eq!(
            Value::new(1.0, 1.0).sum(&Value::new(1.0, 2.0)),
            Value::new(3.0, 2.0)
        );
        assert_eq!(
            Value::new(-1.0, 1.0).sum(&Value::new(1.0, 2.0)),
            Value::new(-1.0, 2.0)
        );
    }
    #[test]
    fn mul() {
        assert_eq!(
            Value::new(1.0, 1.0).mul(&Value::new(1.0, 1.0)),
            Value::new(1.0, 1.0)
        );
        assert_eq!(
            Value::new(3.0, 4.0).mul(&Value::new(1.0, 2.0)),
            Value::new(3.0, 8.0)
        );
        assert_eq!(
            Value::new(-3.0, 4.0).mul(&Value::new(1.0, 2.0)),
            Value::new(-3.0, 8.0)
        );
    }
    #[test]
    fn sub() {
        assert_eq!(
            Value::new(3.0, 2.0).sub(&Value::new(1.0, 2.0)),
            Value::new(4.0, 4.0)
        );
        assert_eq!(
            Value::new(1.0, 4.0).sub(&Value::new(1.0, 2.0)),
            Value::new(-2.0, 8.0)
        );
    }
    #[test]
    fn less_than() {
        assert_eq!(Value::new(3.0, 2.0).less_than(&Value::new(1.0, 1.0)), false);
        assert_eq!(Value::new(1.0, 8.0).less_than(&Value::new(1.0, 4.0)), true);
        assert_eq!(Value::new(-3.0, 2.0).less_than(&Value::new(1.0, 1.0)), true);
        assert_eq!(
            Value::new(1.0, 4.0).less_than(&Value::new(-1.0, 8.0)),
            false
        );
    }
    #[test]
    fn bigger_than() {
        assert_eq!(
            Value::new(1.0, 1.0).bigger_than(&Value::new(3.0, 2.0)),
            false
        );
        assert_eq!(
            Value::new(1.0, 4.0).bigger_than(&Value::new(1.0, 8.0)),
            true
        );
        assert_eq!(
            Value::new(1.0, 1.0).bigger_than(&Value::new(-3.0, 2.0)),
            true
        );
        assert_eq!(
            Value::new(-1.0, 8.0).bigger_than(&Value::new(1.0, 4.0)),
            false
        );
    }
    #[test]
    fn powi() {
        assert_eq!(Value::new(3.0, 2.0).powi(2), Value::new(9.0, 4.0));
        assert_eq!(Value::new(1.0, 4.0).powi(1), Value::new(1.0, 4.0));
    }
}
