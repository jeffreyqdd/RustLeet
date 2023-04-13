struct Solution {}
impl Solution {
    pub fn my_function(s : String) -> String {
        s
    }
}

fn run_test(word: &str) {
    let answer = Solution::my_function(String::from(word));
    println!("answer to \"{}\" is: \"{}\"", word, &answer);
}

fn main() {
    run_test("hello world");
}