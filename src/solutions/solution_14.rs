use std::collections::HashMap;

pub fn a(input: &str) {
    solve(input, 10);
}

pub fn b(input: &str) {
    solve(input, 40);
}

fn solve(input: &str, n: u32) {
    let (t, r) = input.split_once("\n\n").unwrap();
    let polymer = Polymer::new(t, r);
    let result = polymer.react(n);
    println!("{}", analyze(&result));
}

struct Polymer<'a> {
    template: String,
    rule_map: HashMap<&'a str, Node<'a>>,
}

impl<'a> Polymer<'a> {
    fn new(t: &str, r: &'a str) -> Self {
        let template = t.to_owned();
        let rules: HashMap<&str, &str> = r.lines().map(|l| l.split_once(" -> ").unwrap()).collect();
        let rule_map = make_rule_map(rules);

        Polymer { template, rule_map }
    }

    fn react(&self, depth: u32) -> String {
        self.make_trees(depth)
    }

    fn make_trees(&self, depth: u32) -> String {
        let seed = self.template.to_owned();

        let mut patterns = Vec::new();

        for i in 0..seed.len() - 1 {
            patterns.push(&seed[i..i + 1]);
        }

        let mut trees = Vec::new();
        for pattern in patterns {
            let tree = self.tree(pattern);
            trees.push(tree);
        }

        self.join_trees(&trees, depth)
    }

    fn tree(&self, value: &'a str) -> Node {
        match self.rule_map.get(value) {
            Some(node) => node.clone(),
            None => Node {
                value,
                children: [None; 2],
            },
        }
    }

    fn join_trees(&self, trees: &[Node], depth: u32) -> String {
        let mut result = String::new();

        trees.iter().map(|tree| {});

        result
    }
}

type Rule<'a> = (&'a str, &'a str);

fn make_rule_map<'a>(rules: HashMap<&'a str, &'a str>) -> HashMap<&'a str, Node<'a>> {
    let mut rule_map = HashMap::new();
    for (pattern, element) in rules.iter() {
        let node = rule_to_node((*pattern, *element));
        rule_map.insert(*pattern, node);
    }

    for (pattern, node) in rule_map.iter() {
        for opt_child in node.children.iter() {
            let mut child = opt_child.unwrap(); // first layer (rules) children exist
            if let Some(rule_node) = rule_map.get(child.value) {
                child = rule_node;
            }
        }
    }
    rule_map
}

fn rule_to_node((pattern, element): Rule) -> Node {
    todo!()
    // let mut node = Node::new();
    // let mut lhs = Node {
    //     value: &(pattern[..1].to_owned() + element),
    //     children: Default::default(),
    // };
    // let mut rhs = Node {
    //     value: &(element.to_owned() + &pattern[1..]),
    //     children: Default::default(),
    // };

    // node.value = pattern;
    // node.children = [Some(&lhs), Some(&rhs)];

    // node
}

#[derive(Clone, Copy)]
struct Node<'a> {
    value: &'a str,
    children: [Option<&'a Node<'a>>; 2],
}

impl Node<'_> {
    fn new() -> Self {
        Node {
            value: Default::default(),
            children: [None; 2],
        }
    }

    fn traverse_r(&self, string: &str, depth: u32) -> String {
        todo!()
        // let mut next: String;// = string.to_owned();
        // if depth == 0 {
        //     next = self.value[..1].to_owned() + string;
        // }

        // if let Some(lhs) = self.children[0] {
        //     next = lhs.traverse(string.to_owned()depth - 1)
        // } else {
        //     next =
        //     }

        // string.to_owned() + next
    }
}


fn analyze(polymer: &str) -> u32 {
    let mut counts = HashMap::new();
    polymer.chars().for_each(|c| {
        match counts.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                counts.insert(c, 1);
            }
        };
    });

    let mut common = ('.', 0);
    let mut uncommon = ('.', u32::MAX);

    for (c, v) in counts.iter() {
        if *v > common.1 {
            common = (*c, *v);
        }
        if *v < uncommon.1 {
            uncommon = (*c, *v);
        }
    }

    common.1 - uncommon.1
}

