use serde::{Deserialize, Serialize};
use superslice::*;

type TData = f64;

#[derive(Serialize, Deserialize, Debug)]
struct FCNode {
    value: TData,
    orig_list_upper_bound: usize,
    prev_list_upper_bound: usize,
    //// Debug
    //new_list: usize,
    //orig_list: usize,
    //orig_list_idx: usize,
    //idx: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FractionalCascadingMultiListSearcher {
    nodes: Vec<FCNode>,
    list_ranges: Vec<(usize, usize)>,
}

impl FractionalCascadingMultiListSearcher {
    pub fn new(data: &Vec<Vec<TData>>) -> FractionalCascadingMultiListSearcher {
        let mut nodes: Vec<FCNode> = Vec::new();
        let mut list_ranges_rev = Vec::new();

        let mut prev_start: usize = 0;
        let mut prev_end: usize = 0;

        for (i, curr_list) in data.iter().enumerate().rev() {
            let mut curr_orig_list_upper_bound: usize = 0;
            let mut curr_prev_list_upper_bound: usize = prev_start;

            // Merge the lists.
            let mut curr_i: usize = 0;
            let mut prev_i: usize = prev_start;
            while prev_i < prev_end || curr_i < curr_list.len() {
                if (prev_i >= prev_end)
                    || (curr_i < curr_list.len() && curr_list[curr_i] <= nodes[prev_i].value)
                {
                    curr_orig_list_upper_bound = curr_i + 1;
                    nodes.push(FCNode {
                        value: curr_list[curr_i],
                        orig_list_upper_bound: curr_orig_list_upper_bound,
                        prev_list_upper_bound: curr_prev_list_upper_bound,
                        //new_list: i,
                        //orig_list: i,
                        //orig_list_idx: curr_i,
                        //idx: nodes.len(),
                    });
                    curr_i += 1;
                } else {
                    curr_prev_list_upper_bound = prev_i + 1;
                    nodes.push(FCNode {
                        value: nodes[prev_i].value,
                        orig_list_upper_bound: curr_orig_list_upper_bound,
                        prev_list_upper_bound: curr_prev_list_upper_bound,
                        //new_list: i,
                        //orig_list: nodes[prev_i].orig_list,
                        //orig_list_idx: nodes[prev_i].orig_list_idx,
                        //idx: nodes.len(),
                    });

                    // "Fraction" is 1/2
                    prev_i += 2;
                };
            }

            prev_start = prev_end;
            prev_end = nodes.len();
            list_ranges_rev.push((prev_start, prev_end));
        }

        FractionalCascadingMultiListSearcher {
            nodes: nodes,
            list_ranges: list_ranges_rev.into_iter().rev().collect(),
        }
    }

    fn num_lists(&self) -> usize {
        self.list_ranges.len()
    }

    pub fn search(&self, x: TData, out: &mut [usize]) {
        assert_eq!(out.len(), self.num_lists());

        let first_list_upper_bound = self.nodes[self.list_ranges[0].0..]
            .upper_bound_by(|node| node.value.partial_cmp(&x).unwrap());

        let mut upper_bound = self.list_ranges[0].0 + first_list_upper_bound;

        for i in 0..out.len() {
            let (curr_list_begin, curr_list_end) = self.list_ranges[i];
            if upper_bound < curr_list_end && self.nodes[upper_bound].value <= x {
                upper_bound += 1;
            }

            out[i] = if upper_bound == curr_list_begin {
                0
            } else {
                self.nodes[upper_bound - 1].orig_list_upper_bound
            };

            upper_bound = if upper_bound == curr_list_begin {
                if i + 1 < out.len() {
                    self.list_ranges[i + 1].0
                } else {
                    0
                }
            } else {
                self.nodes[upper_bound - 1].prev_list_upper_bound
            };
        }
    }
}
