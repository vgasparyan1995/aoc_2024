use std::io::stdin;

#[derive(Debug, Clone)]
struct State {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    memory: Vec<i64>,
    pointer: usize,
}

impl State {
    fn is_halted(&self) -> bool {
        self.pointer >= self.memory.len()
    }

    fn instruction(&self) -> i64 {
        self.memory[self.pointer]
    }

    fn arg(&self) -> i64 {
        self.memory[self.pointer + 1]
    }

    fn combo_arg(&self) -> i64 {
        match self.memory[self.pointer + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!(),
        }
    }

    fn advance(self) -> (State, Option<i64>) {
        let mut output = None;
        let state = if self.is_halted() {
            self
        } else {
            match self.instruction() {
                0 => State {
                    reg_a: self.reg_a / (1 << self.combo_arg()),
                    pointer: self.pointer + 2,
                    ..self
                },
                1 => State {
                    reg_b: self.reg_b ^ self.arg(),
                    pointer: self.pointer + 2,
                    ..self
                },
                2 => State {
                    reg_b: self.combo_arg() % 8,
                    pointer: self.pointer + 2,
                    ..self
                },
                3 => State {
                    pointer: if self.reg_a == 0 {
                        self.pointer + 2
                    } else {
                        self.arg() as usize
                    },
                    ..self
                },
                4 => State {
                    reg_b: self.reg_b ^ self.reg_c,
                    pointer: self.pointer + 2,
                    ..self
                },
                5 => {
                    output = Some(self.combo_arg() % 8);
                    State {
                        pointer: self.pointer + 2,
                        ..self
                    }
                }
                6 => State {
                    reg_b: self.reg_a / (1 << self.combo_arg()),
                    pointer: self.pointer + 2,
                    ..self
                },
                7 => State {
                    reg_c: self.reg_a / (1 << self.combo_arg()),
                    pointer: self.pointer + 2,
                    ..self
                },
                _ => panic!(),
            }
        };
        (state, output)
    }
}

fn parse_state() -> State {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let reg_a = line
        .trim()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let reg_b = line
        .trim()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let reg_c = line
        .trim()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    stdin().read_line(&mut line).unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let memory = line
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|cell| cell.parse().unwrap())
        .collect();
    State {
        reg_a,
        reg_b,
        reg_c,
        memory,
        pointer: 0,
    }
}

fn execute(init_state: State) -> Vec<i64> {
    let mut state = init_state;
    let mut outputs = Vec::new();
    while !state.is_halted() {
        let (next_state, output) = state.advance();
        state = next_state;
        if let Some(output) = output {
            outputs.push(output);
        }
    }
    outputs
}

fn part1(init_state: State) -> String {
    execute(init_state)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(init_state: State) -> i64 {
    let mut result = 0;
    for _ in init_state.memory.iter().rev() {
        for candidate in 0.. {
            println!("Trying {candidate}");
            let mut state = init_state.clone();
            state.reg_a = (result << 3) + candidate;
            let output = execute(state);
            println!("Output: {output:?}");
            if init_state.memory.ends_with(&output[..]) {
                result = (result << 3) + candidate;
                break;
            }
        }
    }
    result
}

fn main() {
    let state = parse_state();
    println!("Part 1: {}", part1(state.clone()));
    println!("Part 2: {}", part2(state));
}
