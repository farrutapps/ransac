pub mod model;
use model::Model;
pub mod line2d;
use line2d::{Point2d, Line2d};
use rand::{seq::IteratorRandom, thread_rng};
use std::cmp;

pub struct RansacSettings<M: Model> {
    pub max_inlier_error: M::Error,
    pub max_iters: i32,
    pub expected_inlier_ratio: f64,
    pub convergence_probability: f64
}

pub fn run_ransac<M: Model>(data: &Vec<M::DataPoint>, params: &RansacSettings<M>) -> Option<M> {
    let mut rng = thread_rng();

    let mut best_num_inliers = 0;
    let mut best_model: Option<M> = None;

    if data.len() < 2 {
        return None;
    }

    let num_iters = ((1. - params.convergence_probability).ln() / (1.-params.expected_inlier_ratio.powi(2)).ln()).ceil() as i32;
    for _i in 0..cmp::min(num_iters, params.max_iters) {

        let sample = data.iter().choose_multiple(&mut rng, 2);
        let hypothesis_model = M::hypothesis(&sample);
        let num_inliers = data.iter().filter(|pt| hypothesis_model.is_inlier(&pt, &params.max_inlier_error)).count();

        if num_inliers > best_num_inliers {
            best_num_inliers = num_inliers;
            best_model = Some(hypothesis_model);
        }

    }
    best_model
}