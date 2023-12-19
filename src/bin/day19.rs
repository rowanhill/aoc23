use std::{collections::{HashMap, VecDeque}, ops::RangeInclusive};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Transition {
    Workflow(String),
    Accept,
    Reject,
    ContinueBranch,
}
impl Transition {
    fn parse(s: &str) -> Transition {
        match s {
            "A" => Transition::Accept,
            "R" => Transition::Reject,
            _ => Transition::Workflow(s.to_string()),
        }
    }

}

enum PartCategory { X, M, A, S }
impl PartCategory {
    fn parse(s: &str) -> PartCategory {
        match s {
            "x" => PartCategory::X,
            "m" => PartCategory::M,
            "a" => PartCategory::A,
            "s" => PartCategory::S,
            _ => panic!("Invalid part category {}", s),
        }
    }
}

enum Comparitor { Lt, Gt }
impl Comparitor {
    fn parse(s: &str) -> Comparitor {
        match s {
            "<" => Comparitor::Lt,
            ">" => Comparitor::Gt,
            _ => panic!("Invalid comparitor {}", s),
        }
    }
}

struct Branch {
    category: PartCategory,
    comparitor: Comparitor,
    comparison_value: usize,
    pass: Transition,
    fail: Transition,
}
impl Branch {
    fn parse(s: &str) -> Branch {
        let category = PartCategory::parse(&s[0..1]);
        let comparitor = Comparitor::parse(&s[1..2]);
        let (comparison_value, transition) = s[2..].split_once(':').unwrap();
        let comparison_value = comparison_value.parse().unwrap();
        let transition = Transition::parse(transition);
        Branch {
            category,
            comparitor,
            comparison_value,
            pass: transition,
            fail: Transition::ContinueBranch,
        }
    }

    fn test_part(&self, part: &Part) -> &Transition {
        let value = part.get(&self.category);
        let pass = match self.comparitor {
            Comparitor::Lt => value < self.comparison_value,
            Comparitor::Gt => value > self.comparison_value,
        };
        if pass {
            &self.pass
        } else {
            &self.fail
        }
    }
}

struct Workflow {
    branches: Vec<Branch>
}
impl Workflow {
    fn parse(line: &str) -> (String, Workflow) {
        let (name, branches) = line.split_once('{').unwrap();
        let branch_strs = &branches[0..branches.len()-1];
        let mut branches = Vec::new();
        for branch_str in branch_strs.split(',') {
            if branch_str.contains(':') {
                let branch = Branch::parse(branch_str);
                branches.push(branch);
            } else {
                // This should be the final "branch" - but it's just the fail transition of the previous branch
                let fail = Transition::parse(branch_str);
                branches.last_mut().unwrap().fail = fail;
            }
        }
        (name.to_string(), Workflow { branches })
    }

    fn test_part(&self, part: &Part) -> &Transition {
        for branch in &self.branches {
            match branch.test_part(part) {
                Transition::ContinueBranch => continue,
                transition => return transition,
            }
        }
        panic!("No transition found for part");
    }

    fn restrict_ranges(&self, ranges: &PartRanges) -> Vec<(PartRanges, Transition)> {
        let mut restricted_ranges = Vec::new();
        let mut cur_ranges = ranges.clone();
        for branch in &self.branches {
            let (pass_ranges, fail_ranges) = cur_ranges.split(&branch.category, &branch.comparitor, branch.comparison_value);
            if let Some(pass_ranges) = pass_ranges {
                restricted_ranges.push((pass_ranges, branch.pass.clone()));
            }
            if let Some(fail_ranges) = fail_ranges {
                if matches!(branch.fail, Transition::ContinueBranch) {
                    cur_ranges = fail_ranges;
                } else {
                    restricted_ranges.push((fail_ranges, branch.fail.clone()));
                }
            }
        }
        restricted_ranges
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn parse(s: &str) -> Part {
        let mut part = Part { x: 0, m: 0, a: 0, s: 0 };
        let s = s.trim_start_matches('{').trim_end_matches('}');
        for cat_spec in s.split(',') {
            let (cat, count) = cat_spec.split_once('=').unwrap();
            let count = count.parse().unwrap();
            match cat {
                "x" => part.x = count,
                "m" => part.m = count,
                "a" => part.a = count,
                "s" => part.s = count,
                _ => panic!("Invalid part category {}", cat),
            }
        }
        part
    }

    fn get(&self, category: &PartCategory) -> usize {
        match category {
            PartCategory::X => self.x,
            PartCategory::M => self.m,
            PartCategory::A => self.a,
            PartCategory::S => self.s,
        }
    }
}

trait LimitableRange where Self: Sized {
    fn split_less_than(&self, value: u16) -> (Option<Self>, Option<Self>);
    fn split_greater_than(&self, value: u16) -> (Option<Self>, Option<Self>);
}

impl LimitableRange for RangeInclusive<u16> {
    fn split_less_than(&self, value: u16) -> (Option<Self>, Option<Self>) {
        if *self.end() < value {
            (Some(self.clone()), None)
        } else if *self.start() >= value {
            (None, Some(self.clone()))
        } else {
            (Some(*self.start()..=value-1), Some(value..=*self.end()))
        }
    }

    fn split_greater_than(&self, value: u16) -> (Option<Self>, Option<Self>) {
        if *self.start() > value {
            (Some(self.clone()), None)
        } else if *self.end() <= value {
            (None, Some(self.clone()))
        } else {
            (Some(value+1..=*self.end()), Some(*self.start()..=value))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PartRanges {
    x: RangeInclusive<u16>,
    m: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
}
impl PartRanges {
    fn new() -> Self {
        PartRanges {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn split(&self, category: &PartCategory, comparitor: &Comparitor, comparison_value: usize) -> (Option<Self>, Option<Self>) {
        let cur_range = self.get(category);
        let comparison_value = comparison_value as u16;
        let (pass_range, fail_range) = match comparitor {
            Comparitor::Lt => cur_range.split_less_than(comparison_value),
            Comparitor::Gt => cur_range.split_greater_than(comparison_value),
        };
        (
            pass_range.map(|r| self.clone_with(category, r)),
            fail_range.map(|r| self.clone_with(category, r))
        )
    }

    fn get(&self, category: &PartCategory) -> &RangeInclusive<u16> {
        match category {
            PartCategory::X => &self.x,
            PartCategory::M => &self.m,
            PartCategory::A => &self.a,
            PartCategory::S => &self.s,
        }
    }

    fn clone_with(&self, category: &PartCategory, value: RangeInclusive<u16>) -> Self {
        match category {
            PartCategory::X => PartRanges { x: value, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() },
            PartCategory::M => PartRanges { x: self.x.clone(), m: value, a: self.a.clone(), s: self.s.clone() },
            PartCategory::A => PartRanges { x: self.x.clone(), m: self.m.clone(), a: value, s: self.s.clone() },
            PartCategory::S => PartRanges { x: self.x.clone(), m: self.m.clone(), a: self.a.clone(), s: value },
        }
    }
}

struct System {
    workflows: HashMap<String, Workflow>,
}
impl System {
    fn new(workflows: HashMap<String, Workflow>) -> System {
        System { workflows }
    }

    fn test_part(&self, part: &Part) -> bool {
        let mut cur_workflow = &self.workflows["in"];
        loop {
            match cur_workflow.test_part(part) {
                Transition::Workflow(w) => cur_workflow = &self.workflows[w],
                Transition::Accept => return true,
                Transition::Reject => return false,
                Transition::ContinueBranch => panic!("Workflow returned ContinueBranch"),
            }
        }
    }

    fn calculate_accepted_ranges(&self) -> Vec<PartRanges> {
        let mut ranges = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((PartRanges::new(), &self.workflows["in"]));

        while let Some((cur_ranges, cur_workflow)) = queue.pop_front() {
            for (next_ranges, transition) in cur_workflow.restrict_ranges(&cur_ranges) {
                match transition {
                    Transition::Workflow(w) => queue.push_back((next_ranges, &self.workflows[&w])),
                    Transition::Accept => ranges.push(next_ranges),
                    Transition::Reject => {},
                    Transition::ContinueBranch => panic!("Workflow returned ContinueBranch"),
                }
            }
        }

        ranges
    }
}

fn parse_file(input: &str) -> (System, Vec<Part>) {
    let mut lines = input.lines();

    let mut workflows = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, workflow) = Workflow::parse(line);
        workflows.insert(name, workflow);
    }

    let mut parts = Vec::new();
    for line in lines {
        let part = Part::parse(line);
        parts.push(part);
    }

    (System::new(workflows), parts)
}

fn sum_all_categories_of_parts_accepted_by_system(system: &System, parts: &[Part]) -> usize {
    let mut sum = 0;
    for part in parts {
        if system.test_part(part) {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    sum
}

fn calculate_possibilities(accept_ranges: &[PartRanges]) -> usize {
    accept_ranges.iter().map(|r| r.x.len() * r.m.len() * r.a.len() * r.s.len()).sum()
}

fn count_all_possible_valid_parts(system: &System) -> usize {
    let accept_ranges = system.calculate_accepted_ranges();
    calculate_possibilities(&accept_ranges)
}

fn main() {
    let input = include_str!("../../input/day19");
    let (system, parts) = parse_file(input);
    println!("Part 1: {}", sum_all_categories_of_parts_accepted_by_system(&system, &parts));

    let possibilities = count_all_possible_valid_parts(&system);
    println!("Part 2: {}", possibilities);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_example_1() {
        let (system, parts) = parse_file(EXAMPLE_1);
        assert_eq!(sum_all_categories_of_parts_accepted_by_system(&system, &parts), 19114);
    }

    #[test]
    fn test_example_2() {
        let (system, _) = parse_file(EXAMPLE_1);
        assert_eq!(count_all_possible_valid_parts(&system), 167409079868000);
    }

    #[test]
    fn test_example_2_first_workflow_restriction() {
        let (system, _) = parse_file(EXAMPLE_1);
        let result = system.workflows["in"].restrict_ranges(&PartRanges::new());
        assert_eq!(result, vec![
            (PartRanges { x: 1..=4000, m: 1..=4000, a: 1..=4000, s: 1..=1350 }, Transition::Workflow("px".to_string())),
            (PartRanges { x: 1..=4000, m: 1..=4000, a: 1..=4000, s: 1351..=4000 }, Transition::Workflow("qqz".to_string())),
        ]);
    }

    #[test]
    fn text_calculating_possibilities_base() {
        let accept_ranges = vec![
            PartRanges { x: 1..=4000, m: 1..=4000, a: 1..=4000, s: 1..=4000 },
        ];
        assert_eq!(calculate_possibilities(&accept_ranges), 4000 * 4000 * 4000 * 4000);
    }

    #[test]
    fn text_calculating_possibilities_multiple() {
        let accept_ranges = vec![
            PartRanges { x: 1..=2, m: 1..=3, a: 1..=3, s: 1..=3 }, // 2 * 3 * 3 * 3 = 54
            PartRanges { x: 3..=3, m: 1..=3, a: 1..=3, s: 1..=3 }, // 1 * 3 * 3 * 3 = 27
        ];
        assert_eq!(calculate_possibilities(&accept_ranges), 54 + 27);
    }
}