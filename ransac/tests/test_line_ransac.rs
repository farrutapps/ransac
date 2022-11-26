use std::{vec::Vec};

extern crate ransac;
use ransac::line2d::{Line2d, Point2d};


    fn params() -> ransac::RansacSettings<Line2d> {
    ransac::RansacSettings { max_inlier_error: 0.2, max_iters: 100, expected_inlier_ratio: 0.75, convergence_probability: 0.999 }
    }


    #[test]
    fn very_simple() {
        let vec = vec![Point2d{x:0.,y:0.}, Point2d{x:1.,y:1.}];
        let line = ransac::run_ransac(&vec, &params());
        assert_eq!(line, Some(Line2d{k:1., d:0.}));
    }

    #[test]
    fn still_simple() {
        let line = ransac::run_ransac(&vec![Point2d{x:1.,y:2.}, Point2d{x:3.,y:4.}], &params());
        assert_eq!(line, Some(Line2d{k:1., d:1.}));
    }

    #[test]
    fn medium() {
        let line = ransac::run_ransac(&vec![Point2d{x:1.,y:1.5}, Point2d{x:1.,y:2.},Point2d{x:7.,y:8.}, Point2d{x:3.,y:4.}], &params());
        assert_eq!(line, Some(Line2d{k:1., d:1.}));
    }

    #[test]
    fn vertical() {
        let line: Option<Line2d> = ransac::run_ransac(&vec![Point2d{x:0.,y:0.}, Point2d{x:0.,y:1.}], &params());
        assert_eq!(line, None);
    }

    #[test]
    fn single_and_empty_works() {
        let line: Option<Line2d> = ransac::run_ransac(&vec![Point2d{x:1.,y:2.}], &params());
        assert_eq!(line, None);
        let line: Option<Line2d> = ransac::run_ransac(&Vec::<Point2d>::new(), &params());
        assert_eq!(line, None);
    }