use phf::phf_map;

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

static NUMBERS: phf::Map<&'static str, i32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn extract_number_from_line_include_text(line: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    let mut prefix: String = String::new();

    for c in line.chars() {
        prefix.push(c);
        if c.is_numeric() {
            numbers.push(c.to_digit(10).unwrap() as i32);
        } else {
            for (k, v) in NUMBERS.entries() {
                if prefix.ends_with(*k) {
                    numbers.push(*v);
                    break;
                }
            }
        }
    }
    assert!(numbers.len() > 0);

    return numbers[0] * 10 + numbers[numbers.len() - 1];
}

fn get_updated_calibration_sum(document: &str) -> i32 {
    let mut sum: i32 = 0i32;
    for line in document.lines() {
        let number: i32 = extract_number_from_line_include_text(line);
        sum += number;
    }

    return sum;
}

pub struct Day1Puzzle {}
impl super::solve::Puzzle<i32> for Day1Puzzle {
    fn solve(&self, document: &str) -> i32 {
        return get_calibration_sum(document);
    }

    fn solve2(&self, document: &str) -> i32 {
        return get_updated_calibration_sum(document);
    }

    fn test_cases(&self) -> Vec<(&str, i32, i32)> {
        return vec![
            ("1abc2", 12, 12),
            ("h5ellowor6ld", 56, 56),
            ("te7st", 77, 77),
            ("mu1lti\nl2in3e\n45test", 79, 79),
            ("one3two", 33, 12),
            ("three4threeight", 44, 38),
        ];
    }

    const DAY: &'static str = "day1";
}
