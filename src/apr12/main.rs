extern crate bencher;
struct Solution {}
impl Solution {
    pub fn my_solution(path: String) -> String {
        let output = path
            .split('/')
            .fold(Vec::with_capacity(100), |mut acc, s| match s {
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
    use std::time::Instant;
    use std::mem::forget;
    use std::ptr;

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
                input: "/hello/world/".to_owned(),
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
    pub fn bench_each(its: u32) {
        assert!(its > 0);
        
        let black_box = |x | {
            unsafe {
                let ret = ptr::read_volatile(&x);
                forget(x);
                ret
            }
        }

        for i in all_tests() {
            // do the benchmark
            for _ in 0..its {
                let closure = || Solution::my_solution(i.input.clone());
                let start = Instant::now();
                black_box(closure());
                let duration = start.elapsed();
            }
        }
    }
}

fn main() {
    testing::assert_pass();
}
