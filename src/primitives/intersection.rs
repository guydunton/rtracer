use super::Shape;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    t: f64,
    shape: Shape,
}

impl Intersection {
    pub fn new(t: f64, shape: Shape) -> Intersection {
        Intersection { t, shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> Shape {
        self.shape
    }

    pub fn hit(mut intersections: Vec<Intersection>) -> Option<Intersection> {
        // Sort by t value
        intersections.sort_unstable_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        // Exclude all the t values < 0
        // return the first value left, if any
        intersections.into_iter().filter(|a| a.t() >= 0.0).nth(0)
    }
}
