use std::collections::HashMap;

pub fn a(input: &str) {
    let cave_network = parse_input(input);
    let paths = cave_network.paths(PathRule::Single);

    println!("{}", paths)
}

pub fn b(input: &str) {
    let cave_network = parse_input(input);
    let paths = cave_network.paths(PathRule::Multiple);

    println!("{}", paths)
}

fn parse_input(input: &str) -> Network {
    let mut network = Network::new();

    input.lines().for_each(|l| {
        let (name0, name1) = l.split_once('-').unwrap();
        network.connect(name0, name1);
    });

    network
}

enum Size {
    Big,
    Small,
}

struct Cave {
    name: String,
    size: Size,
    exits: Vec<usize>,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            name: name.to_owned(),
            size: if name.chars().next().unwrap().is_lowercase() {
                Size::Small
            } else {
                Size::Big
            },
            exits: Vec::new(),
        }
    }
}

struct Network<'a> {
    caves: Vec<Cave>,
    index_map: HashMap<&'a str, usize>,
}

enum PathRule {
    Single,
    Multiple,
}

impl<'a> Network<'a> {
    fn new() -> Network<'a> {
        Network {
            caves: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    fn connect(&mut self, name1: &'a str, name2: &'a str) {
        let idx1 = self.add_or_get(name1);
        let idx2 = self.add_or_get(name2);

        self.caves[idx1].exits.push(idx2);
        self.caves[idx2].exits.push(idx1);
    }

    fn add_or_get(&mut self, name: &'a str) -> usize {
        if let Some(cave_idx) = self.index_map.get(name) {
            return *cave_idx;
        }

        let idx = self.caves.len();
        self.index_map.insert(name, idx);

        let cave = Cave::new(name);
        self.caves.push(cave);

        idx
    }

    fn paths(&self, rule: PathRule) -> u32 {
        let start_idx = self.index_map.get("start").unwrap();

        let mut paths = 0;

        self.find_paths(&mut paths, vec![*start_idx], &rule, false);

        paths
    }

    fn find_paths(&self, count: &mut u32, path: Vec<usize>, rule: &PathRule, revisited_once: bool) {
        for exit_idx in self.caves[*path.last().unwrap()].exits.iter() {
            let exit = &self.caves[*exit_idx];
            if exit.name == "end" {
                *count += 1;
                continue;
            }
            let mut revisit = revisited_once;
            if matches!(exit.size, Size::Small) {
                match rule {
                    PathRule::Single => {
                        if path.contains(exit_idx) {
                            continue;
                        }
                    }
                    PathRule::Multiple => {
                        if exit.name == "start" {
                            continue;
                        }
                        if path.contains(exit_idx) {
                            if !revisit {
                                revisit = true;
                            } else {
                                continue;
                            }
                        }
                    }
                }
            }

            self.find_paths(
                count,
                [path.clone(), vec![*exit_idx]].concat(),
                rule,
                revisit,
            );
        }
    }
}
