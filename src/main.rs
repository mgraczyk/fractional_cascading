#![feature(test)]

mod fc_ind;
mod naive;

use fc_ind::*;
use naive::*;
use rand;
use rand::Rng;
use std::time::Instant;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn create_without_crashing_naive() {
        let data = vec![vec![1., 2., 3.], vec![2., 3., 4.]];
        let _searcher = NaiveMultiListSearcher::new(&data);
    }

    #[test]
    fn search_one_naive() {
        let data = vec![vec![1., 2., 3.], vec![2., 3., 4.]];
        let searcher = NaiveMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let x = 2.5;
        searcher.search(x, &mut result);

        //  0 1 2
        // [1 2 3]
        //     ^ 2.5 at index 2
        // [2 3 4]
        //   ^ 2.5 at index 2
        assert_eq!(result, [2, 1]);
    }

    #[test]
    fn search_many_naive() {
        let data = vec![
            vec![-1., 2., 7., 11.],
            vec![-5., -5., -5., -5.],
            vec![3., 3., 3., 3.],
            vec![4., 5., 6., 7.],
            vec![2., 2., 2., 2.],
            vec![2.9, 3., 3.1, 4.],
            vec![2.9, 3.1, 4., 5.],
        ];
        let searcher = NaiveMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let x = 3.;
        searcher.search(x, &mut result);

        assert_eq!(result, [2, 4, 4, 0, 4, 2, 1]);
    }

    #[test]
    fn create_without_crashing_fc() {
        let data = vec![vec![1., 2., 3.], vec![2., 3., 4.]];
        let _searcher = FractionalCascadingMultiListSearcher::new(&data);
    }

    #[test]
    fn search_one_fc() {
        let data = vec![vec![1., 2., 3.], vec![2., 3., 4.]];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let x = 2.5;
        searcher.search(x, &mut result);

        //  0 1 2
        // [1 2 3]
        //     ^ 2.5 at index 2
        // [2 3 4]
        //   ^ 2.5 at index 2
        assert_eq!(result, [2, 1]);
    }

    #[test]
    fn search_many_simple_fc() {
        let data = vec![vec![1., 2., 3.], vec![2.1, 3., 4.]];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];

        searcher.search(0., &mut result);
        assert_eq!(result, [0, 0]);

        searcher.search(1., &mut result);
        assert_eq!(result, [1, 0]);

        searcher.search(1.5, &mut result);
        assert_eq!(result, [1, 0]);

        searcher.search(2., &mut result);
        assert_eq!(result, [2, 0]);

        searcher.search(2.2, &mut result);
        assert_eq!(result, [2, 1]);

        searcher.search(3., &mut result);
        assert_eq!(result, [3, 2]);

        searcher.search(4., &mut result);
        assert_eq!(result, [3, 3]);
    }

    #[test]
    fn search_many_fc() {
        let data = vec![
            vec![-1., 2., 7., 11.],
            vec![-5., -5., -5., -5.],
            vec![3., 3., 3., 3.],
            vec![4., 5., 6., 7.],
            vec![2., 2., 2., 2.],
            vec![2.9, 3., 3.1, 4.],
            vec![2.9, 3.1, 4., 5.],
        ];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let x = 3.;
        searcher.search(x, &mut result);

        assert_eq!(result, [2, 4, 4, 0, 4, 2, 1]);
    }

    #[test]
    fn search_many_same_fc() {
        let data = vec![vec![1., 1., 1., 1.], vec![1., 1.]];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);
        let mut result = vec![0; data.len()];

        searcher.search(0., &mut result);
        assert_eq!(result, [0, 0]);

        searcher.search(1., &mut result);
        assert_eq!(result, [4, 2]);

        searcher.search(2., &mut result);
        assert_eq!(result, [4, 2]);
    }

    #[test]
    fn search_many_same_different_lists_fc() {
        let data = vec![vec![1., 1., 1., 1.], vec![2., 2.]];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);
        let mut result = vec![0; data.len()];

        searcher.search(0., &mut result);
        assert_eq!(result, [0, 0]);

        searcher.search(1., &mut result);
        assert_eq!(result, [4, 0]);

        searcher.search(2., &mut result);
        assert_eq!(result, [4, 2]);
    }

    #[test]
    fn search_many_before_fc() {
        let data = vec![vec![2., 2.], vec![1., 1., 1., 1.]];
        let searcher = FractionalCascadingMultiListSearcher::new(&data);
        let mut result = vec![0; data.len()];

        searcher.search(0., &mut result);
        assert_eq!(result, [0, 0]);

        searcher.search(1., &mut result);
        assert_eq!(result, [0, 4]);

        searcher.search(2., &mut result);
        assert_eq!(result, [2, 4]);
    }

    fn create_random_vector<
        T: rand::distributions::uniform::SampleUniform + Copy + PartialOrd,
        R: Rng + ?Sized,
    >(
        min: T,
        max: T,
        n: usize,
        rng: &mut R,
        sorted: bool,
    ) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        for _ in 0..vec.capacity() {
            vec.push(rng.gen_range(min, max));
        }

        if sorted {
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        vec
    }

    fn check_keys_are_correct<T: PartialOrd + std::fmt::Debug + Copy>(
        data: &Vec<Vec<T>>,
        x: T,
        result: &Vec<usize>,
    ) {
        for (list, x_out) in data.iter().zip(result) {
            if *x_out >= list.len() {
                assert_eq!(*x_out, list.len());
                assert!(list[list.len() - 1] <= x);
            } else {
                assert!(x < list[*x_out]);
                if *x_out > 0 {
                    assert!(list[*x_out - 1] <= x);
                }
            }
        }
    }

    #[test]
    fn check_random_fc() {
        let mut rng = rand::thread_rng();

        let num_keys = 10000;
        let num_lists = 10;
        let min_list_len = 1000;
        let max_list_len = 10000;

        let data = (0..num_lists)
            .map(|_| {
                create_random_vector(
                    -100.,
                    100.,
                    if min_list_len < max_list_len {
                        rng.gen_range(min_list_len, max_list_len)
                    } else {
                        min_list_len
                    },
                    &mut rng,
                    true,
                )
            })
            .collect();
        let searcher = FractionalCascadingMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let search_keys = create_random_vector(-100., 100., num_keys, &mut rng, false);

        for x in search_keys.iter() {
            searcher.search(*x, &mut result);
            check_keys_are_correct(&data, *x, &result);
        }
    }

    #[test]
    fn bench_large_naive() {
        let mut rng = rand::thread_rng();

        let num_iter = 100;
        let num_keys = 10000;
        let num_lists = 10;
        let min_list_len = 100000;
        let max_list_len = 1000000;

        let data = (0..num_lists)
            .map(|_| {
                create_random_vector(
                    -100.,
                    100.,
                    rng.gen_range(min_list_len, max_list_len),
                    &mut rng,
                    true,
                )
            })
            .collect();
        let searcher = NaiveMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let search_keys = create_random_vector(-100., 100., num_keys, &mut rng, false);
        searcher.search(search_keys[0], &mut result);
        searcher.search(search_keys[search_keys.len() - 1], &mut result);

        let before = Instant::now();
        for _ in 0..num_iter {
            for x in search_keys.iter() {
                searcher.search(*x, &mut result);
            }
        }
        let after = Instant::now();

        println!("Took {} seconds", (after - before).as_secs_f64());
    }

    #[test]
    fn bench_large_fc() {
        let mut rng = rand::thread_rng();

        let num_iter = 100;
        let num_keys = 10000;
        let num_lists = 10;
        let min_list_len = 100000;
        let max_list_len = 1000000;

        let data = (0..num_lists)
            .map(|_| {
                create_random_vector(
                    -100.,
                    100.,
                    rng.gen_range(min_list_len, max_list_len),
                    &mut rng,
                    true,
                )
            })
            .collect();
        let searcher = FractionalCascadingMultiListSearcher::new(&data);

        let mut result = vec![0; data.len()];
        let search_keys = create_random_vector(-100., 100., num_keys, &mut rng, false);
        searcher.search(search_keys[0], &mut result);
        searcher.search(search_keys[search_keys.len() - 1], &mut result);

        let before = Instant::now();
        for _ in 0..num_iter {
            for x in search_keys.iter() {
                searcher.search(*x, &mut result);
            }
        }
        let after = Instant::now();

        println!("Took {} seconds", (after - before).as_secs_f64());
    }
}
