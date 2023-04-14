struct Solution {}

const MAX_N: usize = 1000;

impl Solution {
    pub fn my_solution(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        // precondition: popped is a permutation of push
        let mut pop_ptr = 0;

        let end = |x: &Vec<&i32>| *x[x.len() - 1];

        let res = pushed
            .iter()
            .fold(Vec::with_capacity(MAX_N), |mut acc, elt| {
                acc.push(elt);
                while acc.len() > 0 && end(&acc) == popped[pop_ptr] {
                    acc.pop();
                    pop_ptr += 1;
                }
                acc
            });

        res.len() == 0
    }
}

mod testing {
    use super::Solution;
    use std::mem::forget;
    use std::ptr;
    use std::time::{Duration, Instant};

    struct InputType(Vec<i32>, Vec<i32>);
    struct UnitTest<T, E> {
        input: T,
        output: E,
    }

    fn all_tests() -> Vec<UnitTest<InputType, bool>> {
        vec![
            UnitTest {
                input: InputType(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 5, 4]),
                output: true,
            },
            UnitTest {
                input: InputType(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
                output: true,
            },
            UnitTest {
                input: InputType(vec![1, 2, 3, 4, 5], vec![5, 1, 2, 3, 4]),
                output: false,
            },
            UnitTest {
                input: InputType(vec![], vec![]),
                output: true,
            },
            UnitTest {
                input: InputType((0..1000).collect(), (0..1000).collect()),
                output: true,
            },
        ]
    }

    pub fn assert_pass() {
        let mut iteration_cnt = 1;
        for i in all_tests() {
            let result = Solution::my_solution(i.input.0, i.input.1);
            assert!(result == i.output);
            println!("PASSED case {}", iteration_cnt);
            iteration_cnt += 1;
        }
    }
    pub fn bench(its: u32) {
        assert!(its > 0);

        let black_box = |x| unsafe {
            let ret = ptr::read_volatile(&x);
            forget(x);
            ret
        };

        let mut iteration_cnt = 0;
        for i in all_tests() {
            // do the benchmark
            let mut duration = Duration::ZERO;
            iteration_cnt += 1;

            for _ in 0..its {
                let closure = || Solution::my_solution(i.input.0.clone(), i.input.1.clone());
                let start = Instant::now();
                black_box(closure());
                duration += start.elapsed();
            }
            println!(
                "iteration {} for {} its. Time per it: {:?}",
                iteration_cnt,
                its,
                duration / its
            );
        }
    }
}

fn main() {
    println!("===== RUNNING TEST CASES =====");
    testing::assert_pass();
    println!("===== BENCHING =====");
    testing::bench(100000);
}
