fn extract_number_from_line(line: &str) -> i32 {
    let mut l: i32 = -1i32;
    let mut r: i32 = 0i32;
    for c in line.chars() {
        if !c.is_numeric() {
            continue;
        }
        let v: i32 = c.to_digit(10).unwrap() as i32;
        if l == -1i32 {
            l = v;
        }
        r = v;
    }

    return l * 10 + r;
}

fn get_calibration_sum(document: &str) -> i32 {
    let mut sum: i32 = 0i32;
    for line in document.lines() {
        let number: i32 = extract_number_from_line(line);
        sum += number;
    }

    return sum;
}

pub struct Day1Puzzle {}
impl super::solve::Puzzle<i32> for Day1Puzzle {
    fn solve(&self, document: &str) -> i32 {
        return get_calibration_sum(document);
    }

    fn test_cases(&self) -> Vec<(&str, i32)> {
        return vec![
            ("1abc2", 12),
            ("h5ellowor6ld", 56),
            ("te7st", 77),
            ("mu1lti\nl2in3e\n45test", 79),
        ];
    }

    const DAY: &'static str = "day1";
}
