use crate::{Coord, Mesh};

pub struct Plane {
    origin: Coord,
    normal: Coord,
    d0: f64,
}

type LocalVertex = u32;

type LocalEdge = [LocalVertex; 2];

#[derive(Debug, PartialEq)]
pub struct EdgeIntersection {
    edge: LocalEdge,
    point: Coord,
}

#[derive(Debug, PartialEq)]
pub enum TriIntersection {
    CoplanarFace,
    CoplanarEdge(LocalEdge),
    CoplanarPoint(LocalVertex),
    EdgeEdge(EdgeIntersection, EdgeIntersection),
    EdgeCoPoint(EdgeIntersection, LocalVertex),
    None,
}

impl Plane {
    pub fn new(origin: Coord, normal: Coord) -> Plane {
        let d0 = normal.dot(origin);
        Plane { origin, normal, d0 }
    }
    /// assumes self.normal is normalized
    pub fn dist(&self, p: Coord) -> f64 {
        self.normal.dot(p) - self.d0
    }

    /// assumes l.dot(self.normal) != 0
    /// where l is a line direcetion vector
    fn happy_line_intersect(&self, l0: Coord, l1: Coord) -> Coord {
        // plane dist = (x-p0).dot(n)
        // points on plane: (x-p0).dot(n)=0
        // points on line = l0 + s*l
        // intersection_points: (l0 + s*l - p0).dot(n) = 0
        // s = (p0 - l0).dot(n) / l.dot(n)
        // intersection = l0 + s*l
        let l = l1 - l0;
        let n = self.normal;
        let s = (self.origin - l0).dot(n) / l.dot(n);
        l0 + l * s
    }

    pub fn tri_intersect(&self, tri: [Coord; 3]) -> TriIntersection {
        let dists = [self.dist(tri[0]), self.dist(tri[1]), self.dist(tri[2])];
        let ge = [dists[0] > 0.0, dists[1] > 0.0, dists[2] > 0.0];
        let eq = [dists[0] == 0.0, dists[1] == 0.0, dists[2] == 0.0];
        let le = [!(ge[0] | eq[0]), !(ge[1] | eq[1]), !(ge[2] | eq[2])];

        let eq_sum = eq.iter().map(|b| if *b { 1 } else { 0 }).sum::<u8>();
        let ge_sum = ge.iter().map(|b| if *b { 1 } else { 0 }).sum::<u8>();

        let mut copoints = eq
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b { Some(i as u32) } else { None });

        let mut gepoints = ge
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b { Some(i as u32) } else { None });

        let mut lepoints = le
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b { Some(i as u32) } else { None });

        match (eq_sum, ge_sum) {
            (3, _) => TriIntersection::CoplanarFace,

            (2, _) => {
                TriIntersection::CoplanarEdge([copoints.next().unwrap(), copoints.next().unwrap()])
            }
            (_, 3) | (0, 0) => TriIntersection::None,
            (1, 0 | 2) => TriIntersection::CoplanarPoint(copoints.next().unwrap()),
            (1, 1) => {
                let ids: [LocalVertex; 2] = [gepoints.next().unwrap(), lepoints.next().unwrap()];
                let coords = [tri[ids[0] as usize], tri[ids[1] as usize]];
                TriIntersection::EdgeCoPoint(
                    EdgeIntersection {
                        edge: ids,
                        point: self.happy_line_intersect(coords[0], coords[1]),
                    },
                    copoints.next().unwrap(),
                )
            }
            (0, 2) => {
                let low_id = lepoints.next().unwrap();
                let low_point = tri[low_id as usize];

                let high_id1 = gepoints.next().unwrap();
                let high_id2 = gepoints.next().unwrap();
                let hight_point1 = tri[high_id1 as usize];
                let hight_point2 = tri[high_id2 as usize];

                let loc_edge1 = [low_id, high_id1];
                let loc_edge2 = [low_id, high_id2];

                let intersect1 = EdgeIntersection {
                    edge: loc_edge1,
                    point: self.happy_line_intersect(low_point, hight_point1),
                };

                let intersect2 = EdgeIntersection {
                    edge: loc_edge2,
                    point: self.happy_line_intersect(low_point, hight_point2),
                };

                TriIntersection::EdgeEdge(intersect1, intersect2)
            }
            (0, 1) => {
                let high_id = gepoints.next().unwrap();
                let high_point = tri[high_id as usize];

                let low_id1 = lepoints.next().unwrap();
                let low_id2 = lepoints.next().unwrap();
                let low_point1 = tri[low_id1 as usize];
                let low_point2 = tri[low_id2 as usize];

                let loc_edge1 = [low_id1, high_id];
                let loc_edge2 = [low_id2, high_id];

                let intersect1 = EdgeIntersection {
                    edge: loc_edge1,
                    point: self.happy_line_intersect(high_point, low_point1),
                };

                let intersect2 = EdgeIntersection {
                    edge: loc_edge2,
                    point: self.happy_line_intersect(high_point, low_point2),
                };

                TriIntersection::EdgeEdge(intersect1, intersect2)
            }
            (4.., _) => panic!("unreachable"),
            (_, 4..) => panic!("unreachable"),
        }
    }
}

impl Mesh {
    pub fn split_plane(&mut self, plane: Plane) {
        for mut coords in self.tri_coords() {
            let tri = [
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            ];
            let res = plane.tri_intersect(tri);
            println!("{:?}", tri);
            println!("{:?}", res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EdgeIntersection, Plane, TriIntersection};

    #[test]
    fn test_dist() {
        let plane_origin = [0.5, 0.5, 0.5];
        let z_axis = [0.0, 0.0, 1.0];
        let plane = Plane::new(plane_origin.into(), z_axis.into());

        let co = [0.0, 0.0, 0.0];
        let res = plane.dist(co.into());
        let expected = -0.5;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tri_intersect() {
        let plane_origin = [0.0, 0.0, 0.0];
        let z_axis = [0.0, 0.0, 1.0];
        let plane = Plane::new(plane_origin.into(), z_axis.into());

        let co_tri = [
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 0.0].into(),
        ];
        let res = plane.tri_intersect(co_tri);
        assert_eq!(res, TriIntersection::CoplanarFace);

        let bisect_tri = [
            [1.0, 0.0, 1.0].into(),
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, -1.0].into(),
        ];
        let res = plane.tri_intersect(bisect_tri);
        assert_eq!(
            res,
            TriIntersection::EdgeCoPoint(
                EdgeIntersection {
                    edge: [0, 2],
                    point: [0.5, 0., 0.].into()
                },
                1
            )
        );
    }
}
