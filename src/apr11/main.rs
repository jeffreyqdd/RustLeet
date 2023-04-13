struct Solution {}

impl Solution {
    pub fn remove_stars(s : String) -> String {
        let chars:Vec<_> = s.chars().collect();
        let mut stack = vec![];

        for i in chars {
            match i {
                '*' => {stack.pop();},
                a => {stack.push(a);},
            };
        }
        stack.iter().collect()
    }
}

fn run_test(word: &str) {
    let answer = Solution::remove_stars(String::from(word));
    println!("answer to \"{}\" is: \"{}\"", word, &answer);
}
    
fn main() {
    run_test("leet**cod*e");
    run_test("erase*****");
}
