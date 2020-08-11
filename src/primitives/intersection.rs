use super::Sphere;
use std::cmp::{Ord, Ordering};

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    t: f64,
    shape: Sphere,
}

impl Intersection {
    pub fn new(t: f64, shape: Sphere) -> Intersection {
        if t == std::f64::NAN {
            panic!("Intersection does not support NaN t values");
        }
        Intersection { t, shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> Sphere {
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

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        use crate::maths::is_same;
        is_same(self.t, other.t) && self.shape == other.shape
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        // This is a bit risky but I don't think we should ever
        // have a t value of NaN
        self.t.partial_cmp(&other.t).unwrap()
    }
}