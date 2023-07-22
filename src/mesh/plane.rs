use core::panic;

use crate::Point;

struct Plane {
    origin: Point,
    normal: Point,
}

enum TriIntersection {
    CoplanarFace,
    CoplanarEdge(Point, Point),
    CoplanarPoint(Point),
    EdgeEdge(Point, Point),
    EdgePoint(Point, Point),
    None,
}

impl Plane {
    pub fn new(origin: Point, normal: Point) -> Plane {
        Plane { origin, normal }
    }
    /// assums self.normal is normalized
    pub fn dist(&self, p: Point) -> f64 {
        let prod = self.normal * p;
        let diff = prod - self.origin;
        diff.iter().sum()
    }

    pub fn tri_intersect(&self, tri: [Point; 3]) -> TriIntersection {
        let dists = [self.dist(tri[0]), self.dist(tri[1]), self.dist(tri[2])];
        let ge = [dists[0] > 0.0, dists[1] > 0.0, dists[2] > 0.0];
        let eq = [dists[0] == 0.0, dists[1] == 0.0, dists[2] == 0.0];
        let le = [!(ge[0] | eq[0]), !(ge[1] | eq[1]), !(ge[2] | eq[2])];

        let eq_sum = eq.iter().map(|b| if *b { 1 } else { 0 }).sum::<u8>();
        let ge_sum = ge.iter().map(|b| if *b { 1 } else { 0 }).sum::<u8>();
        // let le_sum = le.iter().map(|b| if *b { 1 } else { 0 }).sum::<u8>();

        let mut copoints = eq
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b { Some(tri[i]) } else { None });

        match (eq_sum, ge_sum) {
            (3, _) => TriIntersection::CoplanarFace,

            (2, _) => {
                TriIntersection::CoplanarEdge(copoints.next().unwrap(), copoints.next().unwrap())
            }
            (_, 3) => TriIntersection::None,
            (0, 0) => TriIntersection::None,
            (1, 0 | 2) => TriIntersection::CoplanarPoint(copoints.next().unwrap()),
            (1, 1) => TriIntersection::CoplanarEdge(todo!(), todo!()),
            (0, 2) => TriIntersection::EdgeEdge(todo!(), todo!()),
            (0, 1) => TriIntersection::EdgeEdge(todo!(), todo!()),

            (4.., _) => panic!("unreachable"),
            (_, 4..) => panic!("unreachable"),
        }
    }
}
