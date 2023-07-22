use crate::Point;

struct Plane {
    origin: Point,
    normal: Point,
}

impl Plane {
    /// assums self.normal is normalized
    pub fn dist(&self, p: Point) -> f64 {
        let prod = self.normal * p;
        let diff = prod - self.origin;
        diff.iter().sum()
    }
}
