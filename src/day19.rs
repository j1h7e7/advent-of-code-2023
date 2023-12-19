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
        panic!("Not implemented");
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
}
