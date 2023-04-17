struct Solution {}
impl Solution {
    pub fn my_function(words: Vec<String>, target: String) -> i32 {
        // opt(i,j,k) = the number of ways to make string target[0..k]
        // after taking the ith character from the jth word.

        // the ith character of the jth word

        // opt(i,j,k) = opt(i-1)

        // acca
        // bbbb
        // caca
        0
    }
}

mod testing {
    use super::Solution;
    use std::mem::forget;
    use std::ptr;
    use std::time::{Duration, Instant};

    struct InputPair(Vec<String>, String);
    struct UnitTest<T, E> {
        input: T,
        output: E,
    }

    fn all_tests() -> Vec<UnitTest<InputPair, i32>> {
        vec![]
    }

    fn gen_test_closure(x: &UnitTest<InputPair, i32>) -> impl FnOnce() -> i32 {
        let input1 = x.input.0.clone();
        let input2 = x.input.1.clone();
        move || Solution::my_function(input1, input2)
    }

    pub fn assert_pass() {
        let mut iteration_cnt = 1;
        for i in all_tests() {
            let result = gen_test_closure(&i)();
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
                let closure = gen_test_closure(&i);
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
