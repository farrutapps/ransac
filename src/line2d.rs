use crate::model;
pub struct Point2d {
    pub x: f64,
    pub y: f64
}

impl Point2d {
    fn distance_to(&self, ln: &Line2d) -> f64 {
        let x = ((self.x + ln.k*self.y) - ln.k*ln.d) / (ln.k*ln.k + 1.);
        let y = (ln.k*(self.x + ln.k*self.y) + ln.d) / (ln.k*ln.k + 1.);
        distance(self,&Point2d{x, y})
    }

    fn subtr(&self, rhs: &Point2d) -> Point2d {
        Point2d{x: self.x - rhs.x, y: self.y - rhs.y}
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y)
    }
}

fn distance(pt1: &Point2d, pt2: &Point2d) -> f64 {
    pt1.subtr(pt2).norm()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2d {
    pub k: f64,
    pub d: f64
}

impl Line2d {
    fn from_points(pt1: &Point2d, pt2: &Point2d) -> Self {
        let k = (pt2.y - pt1.y) / (pt2.x - pt1.x);
        let d = pt1.y - k*pt1.x;
        return Line2d{k,d};
    }
}

impl model::Model for Line2d {
    type DataPoint = Point2d;
    type Error = f64;

    fn hypothesis(data: &Vec<&Self::DataPoint>) -> Self {
        Line2d::from_points(&data[0], &data[1])
    }

    fn is_inlier(&self, data_point: &Self::DataPoint, max_inlier_error: &Self::Error) -> bool {
        data_point.distance_to(&self) < *max_inlier_error 
    }
}
