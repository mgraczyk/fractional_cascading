use serde::{Deserialize, Serialize};
use superslice::*;

type TData = f64;

#[derive(Serialize, Deserialize, Debug)]
pub struct NaiveMultiListSearcher {
    data: Vec<Vec<TData>>,
}

impl NaiveMultiListSearcher {
    pub fn new(data: &Vec<Vec<TData>>) -> NaiveMultiListSearcher {
        NaiveMultiListSearcher { data: data.clone() }
    }

    pub fn search(&self, x: TData, out: &mut [usize]) {
        for (vals, x_out) in self.data.iter().zip(out) {
            *x_out = vals.upper_bound_by(|a| a.partial_cmp(&x).unwrap());
        }
    }
}
