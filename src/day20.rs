use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Signal {
    LOW,
    HIGH,
}
impl Signal {
    fn invert(&self) -> Self {
        match self {
            Signal::LOW => Signal::HIGH,
            Signal::HIGH => Signal::LOW,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Conjunction {
    state: BTreeMap<String, Signal>,
}
impl ComModule for Conjunction {
    fn output(&mut self, signal: Signal, origin: &str) -> Option<Signal> {
        self.state.insert(origin.to_string(), signal);
        if self.state.values().all(|&x| x == Signal::HIGH) {
            return Some(Signal::LOW);
        }
        return Some(Signal::HIGH);
    }
    fn my_hash(&self, state: &mut DefaultHasher) {
        self.hash(state);
    }
    fn reset(&mut self, input_nodes: Vec<String>) {
        self.state = input_nodes
            .iter()
            .map(|x| (x.clone(), Signal::LOW))
            .collect::<BTreeMap<String, Signal>>();
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct FlipFlop {
    state: Signal,
}
impl ComModule for FlipFlop {
    fn output(&mut self, signal: Signal, _origin: &str) -> Option<Signal> {
        if signal == Signal::HIGH {
            return None;
        }
        self.state = self.state.invert();
        return Some(self.state);
    }
    fn my_hash(&self, state: &mut DefaultHasher) {
        self.hash(state);
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct Broadcast {}
impl ComModule for Broadcast {
    fn output(&mut self, signal: Signal, _origin: &str) -> Option<Signal> {
        return Some(signal);
    }
    fn my_hash(&self, _state: &mut DefaultHasher) {}
}

trait ComModule {
    fn output(&mut self, signal: Signal, origin: &str) -> Option<Signal>;
    fn reset(&mut self, _input_nodes: Vec<String>) {}
    fn my_hash(&self, state: &mut DefaultHasher);
}

struct ComModuleNetwork {
    modules: BTreeMap<String, Box<dyn ComModule>>,
    outputs: BTreeMap<String, Vec<String>>,
}
impl ComModuleNetwork {
    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        for (name, module) in &self.modules {
            name.hash(&mut s);
            module.my_hash(&mut s);
        }
        s.finish()
    }
}

fn load_module(line: &str) -> (String, Box<dyn ComModule>, Vec<String>) {
    let parts = line.split(" -> ").collect::<Vec<&str>>();
    let outputs = parts[1]
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if parts[0] == "broadcaster" {
        return ("broadcaster".to_string(), Box::new(Broadcast {}), outputs);
    }
    let mod_type: &str = &parts[0][0..1];
    let mod_name = parts[0][1..].to_string();

    return match mod_type {
        "&" => (
            mod_name,
            Box::new(Conjunction {
                state: BTreeMap::new(),
            }),
            outputs,
        ),
        "%" => (mod_name, Box::new(FlipFlop { state: Signal::LOW }), outputs),
        _ => panic!("Invalid module type"),
    };
}

fn load_all_modules(document: &str) -> ComModuleNetwork {
    let mut modules: BTreeMap<String, Box<dyn ComModule>> = BTreeMap::new();
    let mut node_outputs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut node_inputs: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in document.lines() {
        let (name, module, outputs) = load_module(line);
        modules.insert(name.clone(), module);
        for output in &outputs {
            if !node_inputs.contains_key(output) {
                node_inputs.insert(output.clone(), Vec::new());
            }
            node_inputs.get_mut(output).unwrap().push(name.clone());
        }
        node_outputs.insert(name.clone(), outputs);
    }
    for (name, module) in &mut modules {
        module.reset(node_inputs.get(name).unwrap_or(&vec![]).clone());
    }

    return ComModuleNetwork {
        modules,
        outputs: node_outputs,
    };
}

fn run_network(network: &mut ComModuleNetwork) -> (usize, usize) {
    let mut queue: Vec<(String, String, Signal)> = Vec::new();
    queue.push(("broadcaster".to_string(), "".to_string(), Signal::LOW));

    let mut high_count = 0;
    let mut low_count = 0;

    while queue.len() > 0 {
        let (node, origin, signal) = queue.pop().unwrap();
        match signal {
            Signal::HIGH => {
                high_count += 1;
            }
            Signal::LOW => {
                low_count += 1;
            }
        }

        if !network.modules.contains_key(&node) {
            continue;
        }
        let outputs = network.outputs.get(&node).unwrap();
        let module = network.modules.get_mut(&node).unwrap();
        let output_signal = module.output(signal, &origin);

        for output in outputs {
            if let Some(output_signal) = output_signal {
                queue.push((output.clone(), node.clone(), output_signal));
            }
        }
    }

    return (high_count, low_count);
}

fn sum_tuples(a: Vec<(usize, usize)>) -> (usize, usize) {
    return a.iter().fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
}

pub struct Day20Puzzle {}
impl super::solve::Puzzle<String> for Day20Puzzle {
    fn solve(&self, document: &str) -> String {
        let mut network = load_all_modules(document);
        let mut states: HashMap<u64, (usize, (usize, usize))> = HashMap::new();

        let mut iters = 0;
        loop {
            let (high_count, low_count) = run_network(&mut network);
            let cur_hash = network.calculate_hash();
            if states.contains_key(&cur_hash) {
                break;
            }
            states.insert(cur_hash, (iters, (high_count, low_count)));
            iters += 1;
            if iters == 1000 {
                break;
            }
        }

        let mut seq_states: Vec<(usize, usize)> = vec![(0, 0); states.len()];
        for (_, (iter, data)) in &states {
            seq_states[*iter] = *data;
        }
        if iters == 1000 {
            let (high_count, low_count) = sum_tuples(seq_states);
            return (high_count as u128 * low_count as u128).to_string();
        }
        let dif = iters - states.get(&network.calculate_hash()).unwrap().0;
        let start_loop = iters - dif;

        let (ihs, ils) = sum_tuples(seq_states[0..start_loop].to_vec());
        let (lhs, lls) = sum_tuples(seq_states[start_loop..].to_vec());

        let reps = (1000 - start_loop) / dif;
        let rem = (1000 - start_loop) % dif;

        let (rhs, rls) = sum_tuples(seq_states[start_loop..start_loop + rem].to_vec());

        let high_count = ihs + reps * lhs + rhs;
        let low_count = ils + reps * lls + rls;

        return (high_count as u128 * low_count as u128).to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_conjunction_module() {
        let (name, mut action, outputs) = load_module("&a -> b");
        assert_eq!(name, "a".to_string());
        assert_eq!(outputs, vec!["b".to_string()]);
        action.output(Signal::LOW, "c");
        action.output(Signal::LOW, "d");
        assert_eq!(action.output(Signal::HIGH, "c"), Some(Signal::HIGH));
        assert_eq!(action.output(Signal::LOW, "d"), Some(Signal::HIGH));
        assert_eq!(action.output(Signal::HIGH, "d"), Some(Signal::LOW));
    }

    #[test]
    fn test_load_flip_flop_module() {
        let (name, mut action, outputs) = load_module("%a -> b, c");
        assert_eq!(name, "a".to_string());
        assert_eq!(outputs, vec!["b".to_string(), "c".to_string()]);
        assert_eq!(action.output(Signal::HIGH, "c"), None);
        assert_eq!(action.output(Signal::LOW, "c"), Some(Signal::HIGH));
        assert_eq!(action.output(Signal::LOW, "c"), Some(Signal::LOW));
    }
}
