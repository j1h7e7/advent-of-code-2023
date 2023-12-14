use cached::proc_macro::cached;
use cached::UnboundCache;

fn get_pattern_and_blobs(line: &str) -> (&str, Vec<i64>) {
    let pattern = line.split(' ').nth(0).unwrap();
    let blobs = line.split(' ').nth(1).unwrap();
    // let re = Regex::new(r"\.+").unwrap();
    return (
        // re.replace_all(pattern, ".").clone(),
        pattern,
        blobs
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    );
}

#[cached(
    type = "UnboundCache<String, i64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{} {:?}", pattern, blobs) }"#
)]
fn get_possibilities(pattern: &str, blobs: &Vec<i64>) -> i64 {
    if pattern.len() == 0 && blobs.len() == 0 {
        return 1;
    } else if pattern.len() == 0 {
        return 0;
    } else if blobs.len() == 0 {
        if pattern.chars().all(|c| c == '.' || c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    let c = pattern.chars().nth(0).unwrap();

    if c == '.' {
        return get_possibilities(&pattern[1..], blobs);
    } else if c == '#' {
        let s = blobs[0];
        let small = pattern.len() <= 1;

        if small {
            if blobs.len() > 1 {
                return 0;
            }
            return match s {
                1 => 1,
                _ => 0,
            };
        }

        let c2 = pattern.chars().nth(1).unwrap();

        return match (c2, s) {
            ('#', 1) => 0,
            ('.', 1) => get_possibilities(&pattern[2..], &blobs[1..].to_vec()),
            ('?', 1) => get_possibilities(&pattern[2..], &blobs[1..].to_vec()),
            ('.', 2..) => 0,
            _ => get_possibilities(
                &format!("#{}", &pattern[2..]),
                &vec![s - 1]
                    .to_vec()
                    .into_iter()
                    .chain(blobs[1..].to_vec().into_iter())
                    .collect(),
            ),
        };
    }

    return get_possibilities(&format!("#{}", &pattern[1..]), blobs)
        + get_possibilities(&format!(".{}", &pattern[1..]), blobs);
}

pub struct Day12Puzzle {}
impl super::solve::Puzzle<String> for Day12Puzzle {
    fn solve(&self, document: &str) -> String {
        let lines = document.lines();
        let mut ans = 0_i64;
        for line in lines {
            let (pattern, blobs) = get_pattern_and_blobs(line);
            ans += get_possibilities(pattern, &blobs);
        }

        return ans.to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pattern_and_blobs() {
        assert_eq!(
            get_pattern_and_blobs("#..## 1,3,3"),
            ("#..##", vec![1, 3, 3])
        );
    }

    #[test]
    fn test_get_possibilities() {
        assert_eq!(get_possibilities("#", &vec![1]), 1);
        assert_eq!(get_possibilities(".", &vec![1]), 0);
        assert_eq!(get_possibilities("?", &vec![1]), 1);
        assert_eq!(get_possibilities("#.", &vec![1]), 1);
        assert_eq!(get_possibilities(".#", &vec![1]), 1);
        assert_eq!(get_possibilities("##", &vec![1]), 0);
        assert_eq!(get_possibilities("..", &vec![1]), 0);
        assert_eq!(get_possibilities("??", &vec![1]), 2);
        assert_eq!(get_possibilities("???", &vec![1]), 3);
        assert_eq!(get_possibilities("???", &vec![2]), 2);
        assert_eq!(get_possibilities("???", &vec![1, 1]), 1);
        assert_eq!(get_possibilities("?????", &vec![1, 3]), 1);
        assert_eq!(get_possibilities("?????", &vec![1, 2]), 3);
        assert_eq!(get_possibilities("?????", &vec![1, 1]), 6);
        assert_eq!(get_possibilities("#????", &vec![1, 1]), 3);
    }
}
