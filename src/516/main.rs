use std::cmp::max;

struct Solution {}
const MAX_N: usize = 1000;
impl Solution {
    pub fn my_function(s: String) -> i32 {
        if s == "" {
            return 0;
        }
        // opt(i,j) ∀i,j , where 0 ≤ i < j
        // opt(i,j) denotes the longest palindrome using letters s[i,j]
        // base case: opt(i,i) = 1
        // if s[i] == s[j], opt(i,j) = 2 + opt(i+1, j-1)
        // if s[i] != s[j], opt(i,j) = max(opt(i, j-1), opt(i+1, j))
        // idea: two of the same letters wrapping x is pal iff x is pal.

        let mut dp = [[0; MAX_N]; MAX_N];

        // base case
        let byte_str = s.as_bytes();
        let length = byte_str.len();

        for i in 0..length {
            dp[i][i] = 1;
        }
        // inductive step
        for gap in 1..length {
            for start_idx in 0..length - gap {
                let end_idx = start_idx + gap;
                if byte_str[start_idx] == byte_str[end_idx] {
                    dp[start_idx][end_idx] = 2 + dp[start_idx + 1][end_idx - 1];
                } else {
                    dp[start_idx][end_idx] =
                        max(dp[start_idx + 1][end_idx], dp[start_idx][end_idx - 1]);
                }
            }
        }
        dp[0][length - 1]
    }
}
mod testing {
    use super::Solution;
    use std::mem::forget;
    use std::ptr;
    use std::time::{Duration, Instant};

    struct UnitTest<T, E> {
        input: T,
        output: E,
    }

    fn all_tests() -> Vec<UnitTest<String, i32>> {
        vec![
            UnitTest {
                input: String::from("abba"),
                output: 4,
            },
            UnitTest {
                input: String::from("abbca"),
                output: 4,
            },
            UnitTest {
                input: String::from("cbbd"),
                output: 2,
            },
            UnitTest {
                input: String::from("abcdefg"),
                output: 1,
            },
            UnitTest {
                input: String::from(""),
                output: 0,
            },
            UnitTest {
                input: String::from("aaaaqwertyuiaaaa"),
                output: 9,
            },
            UnitTest {
                input: String::from("a"),
                output: 1,
            },
            UnitTest {
                input: String::from("a".repeat(1000)),
                output: 1000,
            },
        ]
    }

    fn gen_test_closure(x: &UnitTest<String, i32>) -> impl FnOnce() -> i32 {
        let input = x.input.clone();
        move || Solution::my_function(input)
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
    testing::bench(1000);
}
