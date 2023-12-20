use crate::util::parse_lines;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut modules = parse_input(&input);
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (l, h) = push_button(&mut modules);
            low += l;
            high += h;
        }
        (low * high).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let modules = parse_input(&input);
        let broad_dests = &modules["broadcaster"].dests;
        let mut cycles = Vec::new();
        for dest in broad_dests {
            let mut c = 0i64;
            let mut place = 0u32;
            let mut current = dest;
            loop {
                log::debug!("looking at {current}: c {c} place {place}");
                let mo_dests = &modules[current].dests;
                if mo_dests.iter().any(|d| modules[d].typ.is_conjuction()) {
                    log::debug!("{current} connects");
                    c += 2i64.pow(place);
                }
                place += 1;
                let next = mo_dests
                    .iter()
                    .find(|d| modules[d.as_str()].typ.is_flip_flip());
                if let Some(next) = next {
                    current = next;
                } else {
                    break;
                }
            }
            cycles.push(c);
        }
        log::info!("{cycles:?}");
        cycles.into_iter().product::<i64>().to_string()
    }
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = parse_lines(input)
        .map(|mo: Module| (mo.name.clone(), mo))
        .collect();
    let mut inputs_for_conj = Vec::new();
    for mo in modules.values() {
        if mo.typ.is_conjuction() {
            let inputs: HashMap<_, _> = modules
                .values()
                .filter(|it| it.dests.contains(&mo.name))
                .map(|it| (it.name.clone(), false))
                .collect();
            inputs_for_conj.push((mo.name.clone(), inputs));
        }
    }
    for (name, inp) in inputs_for_conj {
        let mo = modules.get_mut(&name).unwrap();
        if let ModuleType::Conjunction { inputs } = &mut mo.typ {
            *inputs = inp;
        }
    }
    modules
}

fn push_button(modules: &mut HashMap<String, Module>) -> (i32, i32) {
    let mut low = 0;
    let mut high = 0;
    let mut q = vec![("broadcaster".to_string(), false, String::new())];
    while let Some((name, pulse, from)) = q.pop() {
        if pulse {
            high += 1;
        } else {
            low += 1;
        }
        let Some(mo) = modules.get_mut(&name) else {
            continue;
        };
        if let Some(result) = mo.receive(pulse, &from) {
            for dest in &mo.dests {
                q.push((dest.clone(), result, name.clone()));
            }
        }
    }
    (low, high)
}

struct Module {
    name: String,
    typ: ModuleType,
    dests: Vec<String>,
}

enum ModuleType {
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<String, bool> },
    Broadcaster,
}

impl Module {
    fn receive(&mut self, pulse: bool, from: &str) -> Option<bool> {
        match &mut self.typ {
            ModuleType::FlipFlop { state } => {
                if pulse {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            ModuleType::Conjunction { inputs } => {
                *inputs.get_mut(from).unwrap() = pulse;
                Some(!inputs.values().all(|b| *b))
            }
            ModuleType::Broadcaster => Some(pulse),
        }
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name_type_str, dests_str) = s.split_once(" -> ").unwrap();
        let typ = match name_type_str.chars().next().unwrap() {
            '%' => ModuleType::FlipFlop { state: false },
            '&' => ModuleType::Conjunction {
                inputs: HashMap::default(),
            },
            _ => ModuleType::Broadcaster,
        };
        let name = if let ModuleType::Broadcaster = &typ {
            name_type_str.to_string()
        } else {
            name_type_str[1..].to_string()
        };
        let dests = dests_str.split(", ").map(str::to_string).collect();
        Ok(Module { name, typ, dests })
    }
}

impl ModuleType {
    fn is_flip_flip(&self) -> bool {
        matches!(self, ModuleType::FlipFlop { .. })
    }

    fn is_conjuction(&self) -> bool {
        matches!(self, ModuleType::Conjunction { .. })
    }
}
