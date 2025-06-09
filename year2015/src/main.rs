fn day1(text: String) -> isize {
    // let up = text.chars().filter(|&c| c == '(').count();
    // let down = text.chars().filter(|&c| c == ')').count();
    let mut count = 0;
    let mut step = 0;
    for char in text.chars() {
        step += 1;
        if char == '(' {
            count += 1
        }
        if char == ')' {
            count -= 1
        }
        if count == -1 {
            return step;
        }
    }
    count
}

fn day1_v2(text: &str) -> (isize, Option<usize>) {
    let mut count = 0;
    let mut first_basement_step = None;

    for (step, char) in text.chars().enumerate() {
        match char {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count == -1 && first_basement_step.is_none() {
                    first_basement_step = Some(step + 1); // +1 because enumerate is 0-based
                }
            }
            _ => (), // Ignore other characters
        }
    }

    (count, first_basement_step)
}

fn day2(text: &str) {
    for line in text.lines() {
        println!("{}", line.split('x').collect())
    }
}
fn main() {
    let test = "20x3x11
15x27x5
6x29x7";
    day2(test);
}
