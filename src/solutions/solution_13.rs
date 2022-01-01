pub fn a(input: &str) {
    let (mut paper, instructions) = parse_input(input);

    paper.fold(&instructions[0]);

    println!("{}", paper.values.len())
}

pub fn b(input: &str) {
        let (mut paper, instructions) = parse_input(input);

        instructions.iter().for_each(|i| paper.fold(i));

        paper.print()
}

fn parse_input(input: &str) -> (Paper, Instructions) {
    let (coords, instructions) = input.split_once("\n\n").unwrap();
    let paper = Paper::from_coords(coords);
    let instr = from_str(instructions);

    (paper, instr)
}

struct Paper {
    values: Vec<(u32, u32)>,
    width: u32,
    height: u32,
}

impl Paper {
    fn from_coords(coords: &str) -> Self {
        let mut width = 0;
        let mut height = 0;

        let values = coords
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(',').unwrap();
                let x = a.parse::<u32>().unwrap();
                let y = b.parse::<u32>().unwrap();
                if x > width {
                    width = x
                };
                if y > height {
                    height = y
                };

                (x, y)
            })
            .collect();

        Paper {
            values,
            width,
            height,
        }
    }
    fn fold(&mut self, fold: &Fold) {
        match *fold {
            Fold::Y(y0) => self.height = y0,
            Fold::X(x0) => self.width = x0,
        }
        self.values = self
            .values
            .iter()
            .filter_map(|&(x, y)| match *fold {
                Fold::X(x0) => {
                    if x > x0 {
                        self.unique((2 * x0 - x, y))
                    } else {
                        Some((x, y))
                    }
                }
                Fold::Y(y0) => {
                    if y > y0 {
                        self.unique((x, 2 * y0 - y))
                    } else {
                        Some((x, y))
                    }
                }
            })
            .collect();
    }

    fn unique(&self, v: (u32, u32)) -> Option<(u32, u32)> {
        if self.values.iter().any(|&w| v.0 == w.0 && v.1 == w.1) {
            None
        } else {
            Some(v)
        }
    }

    fn print(&self) {
        let mut matrix = vec!["."; (self.width * self.height) as usize];

        for &(x, y) in self.values.iter() {
            matrix[(x + y * self.width) as usize] = "#";
        }
        
        for y in 0..self.height {
            let s = (y * self.width) as usize;
            let e = (self.width + y * self.width) as usize;
            let line = matrix[s..e].join("");
            println!("{}", line);
        }
    }
}

enum Fold {
    X(u32),
    Y(u32),
}

type Instructions = Vec<Fold>;

fn from_str(s: &str) -> Instructions {
    s.lines()
        .map(|l| {
            let (xy, v) = l.split_once('=').unwrap_or_else(|| panic!("shit"));
            let n = v.parse::<u32>().unwrap();
            match xy.chars().last().unwrap() {
                'x' => Fold::X(n),
                'y' => Fold::Y(n),
                _ => panic!("wtf!"),
            }
        })
        .collect()
}
