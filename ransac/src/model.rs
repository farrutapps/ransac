pub trait Model {
    type DataPoint;
    type Error;

    fn hypothesis(data: &Vec<&Self::DataPoint>) -> Self;
    fn is_inlier(&self, item: &Self::DataPoint, max_inlier_error: &Self::Error) -> bool;
}