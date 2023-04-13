extern crate bencher;
struct Solution {}
impl Solution {
    pub fn my_solution(path: String) -> String {
        let output = path
            .split('/')
            .fold(vec![], |mut acc, s| match s {
                "." | "" => acc,
                ".." => {
                    acc.pop();
                    acc
                }
                a => {
                    acc.push(a);
                    acc
                }
            })
            .join("/");

        format!("/{}", output)
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

    fn all_tests() -> Vec<UnitTest<String, String>> {
        vec![
            UnitTest {
                input: "/".to_owned(),
                output: "/".to_owned(),
            },
            UnitTest {
                input: "//".to_owned(),
                output: "/".to_owned(),
            },
            UnitTest {
                input: "/hello/world/".repeat(100).to_owned(),
                output: "/hello/world".repeat(100).to_owned(),
            },
            UnitTest {
                input: "/hello/world".to_owned(),
                output: "/hello/world".to_owned(),
            },
        ]
    }

    pub fn assert_pass() {
        for i in all_tests() {
            let result = Solution::my_solution(i.input);
            assert!(result == i.output);
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
                let closure = || Solution::my_solution(i.input.clone());
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
    testing::assert_pass();
    testing::bench(100000);
}
