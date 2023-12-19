use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn score(&self) -> usize {
        return self.x + self.m + self.a + self.s;
    }
}

type PartAcceptor = dyn Fn(Part) -> bool;
type Workflow<'a> = Vec<(Box<PartAcceptor>, &'a str)>;

fn parse_part(line: &str) -> Part {
    let clean_line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    let mut parts = clean_line
        .split(',')
        .map(|part| part[2..].parse::<usize>().unwrap());
    let x = parts.next().unwrap();
    let m = parts.next().unwrap();
    let a = parts.next().unwrap();
    let s = parts.next().unwrap();
    return Part { x, m, a, s };
}

fn parse_workflow(line: &'static str) -> (&str, Workflow) {
    let name = line.split("{").nth(0).unwrap();
    let rules = line
        .split("{")
        .nth(1)
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(",");

    let workflow = rules
        .map(|rule| {
            let parts = rule.split(':').collect::<Vec<&str>>();
            match parts.len() {
                1 => (Box::new(|_| true) as Box<PartAcceptor>, parts[0]),
                2 => (parse_condition(parts[0]), parts[1]),
                _ => panic!("Invalid rule"),
            }
        })
        .collect();

    return (name, workflow);
}
fn parse_condition(condition: &'static str) -> Box<PartAcceptor> {
    let value = condition[2..].parse::<usize>().unwrap();

    let comparer = move |v: usize| -> bool {
        match &condition[1..2] {
            "<" => v < value,
            ">" => v > value,
            _ => panic!("Invalid comparer"),
        }
    };

    return Box::new(move |part: Part| -> bool {
        match &condition[0..1] {
            "x" => comparer(part.x),
            "m" => comparer(part.m),
            "a" => comparer(part.a),
            "s" => comparer(part.s),
            _ => panic!("Invalid category"),
        }
    });
}

fn execute_workflow(workflow: &Workflow, part: Part) -> String {
    for (acceptor, name) in workflow {
        if acceptor(part) {
            return name.to_string();
        }
    }
    panic!("No matching rule found");
}

fn sort_part(
    workflows: &HashMap<&str, Workflow>,
    part: Part,
    accepted: &mut Vec<Part>,
    rejected: &mut Vec<Part>,
) {
    let mut active: &str = "in";
    let mut next_active: String;
    loop {
        let workflow = workflows.get(active).unwrap();
        next_active = execute_workflow(workflow, part);
        match next_active.as_str() {
            "A" => {
                accepted.push(part);
                return;
            }
            "R" => {
                rejected.push(part);
                return;
            }
            x @ _ => active = x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl PartRange {
    fn size(&self) -> usize {
        return (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0);
    }
    fn with_comp(&self, comp: &str, value: (usize, usize)) -> Self {
        match comp {
            "x" => PartRange {
                x: value,
                m: self.m,
                a: self.a,
                s: self.s,
            },
            "m" => PartRange {
                x: self.x,
                m: value,
                a: self.a,
                s: self.s,
            },
            "a" => PartRange {
                x: self.x,
                m: self.m,
                a: value,
                s: self.s,
            },
            "s" => PartRange {
                x: self.x,
                m: self.m,
                a: self.a,
                s: value,
            },
            _ => panic!("Invalid component"),
        }
    }
    fn get_comp(&self, comp: &str) -> (usize, usize) {
        match comp {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid component"),
        }
    }
}

fn apply_condition_to_range(condition: &str, part_range: &PartRange) -> (PartRange, PartRange) {
    let value = condition[2..].parse::<usize>().unwrap();
    let comparer = &condition[1..2];
    let component = &condition[0..1];

    let comp_val = part_range.get_comp(component);
    let (good_val, bad_val) = match comparer {
        "<" => (
            (comp_val.0.min(value), comp_val.1.min(value)),
            (comp_val.0.max(value), comp_val.1.max(value)),
        ),
        ">" => (
            (comp_val.0.max(value + 1), comp_val.1.max(value + 1)),
            (comp_val.0.min(value + 1), comp_val.1.min(value + 1)),
        ),
        _ => panic!("Invalid comparer"),
    };

    let good_range = part_range.with_comp(component, good_val);
    let bad_range = part_range.with_comp(component, bad_val);
    return (good_range, bad_range);
}

fn apply_workflow_to_range(workflow: &str, mut part_range: PartRange) -> Vec<(String, PartRange)> {
    let mut result: Vec<(String, PartRange)> = Vec::new();
    let rules = workflow.split(",");
    let mut bad: PartRange;
    let mut good: PartRange;

    for rule in rules {
        let condition = rule.split(":").nth(0).unwrap();
        let destination: &str;
        match rule.split(":").nth(1) {
            Some(x) => {
                destination = x;
            }
            None => {
                result.push((condition.to_string(), part_range));
                break;
            }
        }

        (good, bad) = apply_condition_to_range(condition, &part_range);
        if good.size() > 0 {
            result.push((destination.to_string(), good));
        }
        if bad.size() > 0 {
            part_range = bad;
        } else {
            break;
        }
    }
    return result;
}

pub struct Day19Puzzle {}
impl super::solve::Puzzle<String> for Day19Puzzle {
    fn solve(&self, document: &str) -> String {
        let static_doc = Box::leak(document.to_string().into_boxed_str());

        let mut chunks = static_doc.split("\n\n");
        let workflows = chunks
            .next()
            .unwrap()
            .lines()
            .map(parse_workflow)
            .collect::<HashMap<&str, Workflow>>();
        let parts = chunks.next().unwrap().lines().map(parse_part);

        let mut accepted: Vec<Part> = Vec::new();
        let mut rejected: Vec<Part> = Vec::new();
        for part in parts {
            sort_part(&workflows, part, &mut accepted, &mut rejected);
        }
        return accepted
            .iter()
            .map(|part| part.score())
            .sum::<usize>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let mut chunks = document.split("\n\n");
        let mut workflows: HashMap<&str, &str> = HashMap::new();
        for line in chunks.next().unwrap().lines() {
            let parts = line.split("{").collect::<Vec<&str>>();
            workflows.insert(parts[0], parts[1].strip_suffix('}').unwrap());
        }

        let mut queue: Vec<(String, PartRange)> = Vec::new();
        queue.push((
            "in".to_string(),
            PartRange {
                x: (1, 4001),
                m: (1, 4001),
                a: (1, 4001),
                s: (1, 4001),
            },
        ));
        let mut ans = 0_usize;
        while queue.len() > 0 {
            let (workflow, part_range) = queue.pop().unwrap();
            let mut next_queue = apply_workflow_to_range(&workflows[workflow.as_str()], part_range);
            while next_queue.len() > 0 {
                let (workflow, part_range) = next_queue.pop().unwrap();
                if workflow == "A" {
                    ans += part_range.size();
                } else if workflow != "R" {
                    queue.push((workflow, part_range));
                }
            }
        }
        return ans.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part() {
        let part = parse_part("{x=0,m=1,a=0,s=0}");
        assert_eq!(part.x, 0);
        assert_eq!(part.m, 1);
        assert_eq!(part.a, 0);
        assert_eq!(part.s, 0);
    }

    #[test]
    fn test_parse_condition() {
        let cond = parse_condition("x<3");
        assert_eq!(
            cond(Part {
                x: 2,
                m: 0,
                a: 0,
                s: 0
            }),
            true
        );
        assert_eq!(
            cond(Part {
                x: 3,
                m: 0,
                a: 0,
                s: 0
            }),
            false
        );
    }

    #[test]
    fn test_partial_splitter() {
        let part_range = PartRange {
            x: (10, 20),
            m: (10, 20),
            a: (10, 20),
            s: (10, 20),
        };
        let (good, bad) = apply_condition_to_range("x<15", &part_range);
        assert_eq!(good.x, (10, 15));
        assert_eq!(good.m, (10, 20));
        assert_eq!(bad.x, (15, 20));

        let (good, bad) = apply_condition_to_range("x>15", &part_range);
        assert_eq!(good.x, (16, 20));
        assert_eq!(bad.x, (10, 16));

        let (good, bad) = apply_condition_to_range("x>0", &part_range);
        assert_eq!(good.x, (10, 20));
        assert_eq!(bad.x.0, bad.x.1);

        let (good, bad) = apply_condition_to_range("x<0", &part_range);
        assert_eq!(good.x.0, good.x.1);
        assert_eq!(bad.x, (10, 20));
    }
}
