use std::collections::{HashMap, VecDeque};

struct FlipFlop {
    name: String,
    state: bool,
    outputs: Vec<String>,
}
struct Conjunction {
    name: String,
    last_input_pulses: HashMap<String, Pulse>,
    outputs: Vec<String>,
}
struct Broadcast {
    name: String,
    outputs: Vec<String>,
}
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}
impl Module {
    fn parse(line: &str) -> Self {
        let (first, rest) = line.split_at(1);
        let (name, neighbours) = rest.split_once(" -> ").unwrap();
        let name = name.to_string();
        let outputs = neighbours.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        match first {
            "b" => Self::Broadcast(Broadcast { name: "broadcaster".to_string(), outputs }),
            "%" => Self::FlipFlop(FlipFlop { name, state: false, outputs }),
            "&" => Self::Conjunction(Conjunction { name, last_input_pulses: HashMap::new(), outputs }),
            c => panic!("Unknown module type: {}", c),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::FlipFlop(f) => &f.name,
            Self::Conjunction(c) => &c.name,
            Self::Broadcast(b) => &b.name,
        }
    }

    fn outputs(&self) -> &Vec<String> {
        match self {
            Self::FlipFlop(f) => &f.outputs,
            Self::Conjunction(c) => &c.outputs,
            Self::Broadcast(b) => &b.outputs,
        }
    }

    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Vec<(String, Pulse)> {
        match self {
            Self::Conjunction(c) => c.receive_pulse(pulse, from),
            Self::FlipFlop(f) => f.receive_pulse(pulse, from),
            Self::Broadcast(b) => b.receive_pulse(pulse, from),
        }
    }
}

#[derive(Clone, Copy)]
enum Pulse {
    High,
    Low,
}

trait PulseModule {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Vec<(String, Pulse)>;
}
impl PulseModule for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse, _from: &str) -> Vec<(String, Pulse)> {
        match pulse {
            Pulse::Low => {
                self.state = !self.state;
                let next_pulse = if self.state { Pulse::High } else { Pulse::Low };
                self.outputs.iter().map(|s| (s.clone(), next_pulse)).collect()
            },
            _ => vec![],
        }
    }
}
impl PulseModule for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Vec<(String, Pulse)> {
        self.last_input_pulses.insert(from.to_owned(), pulse);
        let all_high = self.last_input_pulses.values().all(|p| matches!(p, Pulse::High));
        self.outputs.iter().map(|s| (s.clone(), if all_high { Pulse::Low } else { Pulse::High })).collect()
    }
}
impl PulseModule for Broadcast {
    fn receive_pulse(&mut self, pulse: Pulse, _from: &str) -> Vec<(String, Pulse)> {
        self.outputs.iter().map(|s| (s.clone(), pulse)).collect()
    }
}

struct Circuit {
    modules: HashMap<String, Module>,
}
impl Circuit {
    fn parse(input: &str) -> Self {
        let mut modules = HashMap::new();
        let mut inputs = HashMap::new();
        for line in input.lines() {
            let module = Module::parse(line);
            let name = module.name().to_string();
            for mod_out in module.outputs() {
                inputs.entry(mod_out.clone()).or_insert(Vec::new()).push(name.clone());
            }
            modules.insert(name, module);
        }
        for (name, module) in modules.iter_mut() {
            if let Module::Conjunction(c) = module {
                c.last_input_pulses.extend(inputs.remove(name).unwrap().into_iter().map(|s| (s, Pulse::Low)));
            }
        }
        Self { modules }
    }

    // Counts the numbers of (low, high) pulses sent
    fn push_button(&mut self) -> (usize, usize) {
        let mut count_low = 0;
        let mut count_high = 0;
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

        while let Some((from, pulse, to)) = queue.pop_front() {
            match pulse {
                Pulse::Low => count_low += 1,
                Pulse::High => count_high += 1,
            };
            if let Some(module) = self.modules.get_mut(&to) {
                let next_pulses = module.receive_pulse(pulse, &from)
                    .into_iter()
                    .map(|(next, pulse)| (to.clone(), pulse, next));
                queue.extend(next_pulses);
            }
        }

        (count_low, count_high)
    }

    fn push_button_times(&mut self, n: usize) -> (usize, usize) {
        let mut count_low = 0;
        let mut count_high = 0;
        for _ in 0..n {
            let (low, high) = self.push_button();
            count_low += low;
            count_high += high;
        }
        (count_low, count_high)
    }
}

fn main() {
    let input = include_str!("../../input/day20");
    let mut circuit = Circuit::parse(input);
    let (low, high) = circuit.push_button_times(1000);
    println!("Part 1: {}", low * high);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn test_example1_low_high_counts() {
        let mut circuit = Circuit::parse(EXAMPLE_1);
        let (low, high) = circuit.push_button();
        assert_eq!(low, 8);
        assert_eq!(high, 4);
    }

    const EXAMPLE_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_example_2_low_high_counts() {
        let mut circuit = Circuit::parse(EXAMPLE_2);

        let (low, high) = circuit.push_button();
        assert_eq!(low, 4);
        assert_eq!(high, 4);

        let (low, high) = circuit.push_button();
        assert_eq!(low, 4);
        assert_eq!(high, 2);

        let (low, high) = circuit.push_button();
        assert_eq!(low, 5);
        assert_eq!(high, 3);

        let (low, high) = circuit.push_button();
        assert_eq!(low, 4);
        assert_eq!(high, 2);

        let (low, high) = circuit.push_button();
        assert_eq!(low, 4);
        assert_eq!(high, 4);
    }

    #[test]
    fn test_example_2_low_high_counts_1000_times() {
        let mut circuit = Circuit::parse(EXAMPLE_2);
        let (low, high) = circuit.push_button_times(1000);
        assert_eq!(low, 4250);
        assert_eq!(high, 2750);
    }
}