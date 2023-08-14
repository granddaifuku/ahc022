use proconio::{derive_readable, input, source::line::LineSource};
use std::cmp;
use std::collections::HashMap;
use std::io::{stdin, BufReader, Stdin};

const MAX_TEMPERATURE: i32 = 1000;

#[derive_readable]
#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
}

#[allow(non_snake_case)]
struct Input {
    L: usize,
    N: usize,
    S: usize,
    exit_cells: Vec<Cell>,
}

impl Input {
    #[allow(non_snake_case)]
    fn read(source: &mut LineSource<BufReader<Stdin>>) -> Self {
        input! {
            from source,
            L: usize,
            N: usize,
            S: usize,
            exit_cells: [Cell; N],
        }

        Input {
            L,
            N,
            S,
            exit_cells,
        }
    }
}

struct Solver {
    input: Input,
    temperature: Vec<Vec<i32>>,
    estimate: Vec<usize>,
    temperature_to_cell: HashMap<i32, Vec<Cell>>,
}

impl Solver {
    fn new(input: Input) -> Self {
        let temperature = vec![vec![0; input.L]; input.L];
        let estimate = vec![0; input.N];
        Solver {
            input,
            temperature,
            estimate,
            temperature_to_cell: HashMap::new(),
        }
    }

    fn set_temperature(&mut self) {
        let step = MAX_TEMPERATURE / (self.input.L as i32 * 2);
        for i in 0..self.input.L {
            for j in 0..self.input.L {
                let temp = cmp::max(MAX_TEMPERATURE - i as i32 * step - j as i32 * step, 0);
                self.temperature[i][j] = temp;

                if self.temperature_to_cell.contains_key(&temp) {
                    let v = self.temperature_to_cell.get_mut(&temp).unwrap();
                    v.push(Cell { x: i, y: j });
                } else {
                    self.temperature_to_cell
                        .insert(temp, vec![Cell { x: i, y: j }]);
                }
            }
        }
    }

    fn measure(&self, i: usize, x: usize, y: usize, f: &mut LineSource<BufReader<Stdin>>) -> i32 {
        println!("{} {} {}", i, x, y);
        input! {
            from f,
            v:i32
        }
        v
    }

    fn estimate(&mut self, source: &mut LineSource<BufReader<Stdin>>) {
        for i in 0..self.input.N {
            let mut sum = 0;
            for _ in 0..5 {
                let measured = self.measure(i, 0, 0, source);
                sum += measured;
            }
            sum /= 5;

            let mut diff = 9999;
            for (j, cell) in self.input.exit_cells.iter().enumerate() {
                let d = (self.temperature[cell.x][cell.y] - sum).abs();
                if d < diff {
                    diff = d;
                    self.estimate[i] = j;
                }
            }
        }
    }

    fn output_temperature(&self) {
        for i in 0..self.input.L {
            for j in 0..self.input.L {
                if j == self.input.L - 1 {
                    println!("{}", self.temperature[i][j]);
                } else {
                    print!("{} ", self.temperature[i][j]);
                }
            }
        }
    }

    fn output_final(&self) {
        println!("-1 -1 -1");
        for e in &self.estimate {
            println!("{}", e);
        }
    }

    fn solve(&mut self, source: &mut LineSource<BufReader<Stdin>>) {
        self.set_temperature();
        self.output_temperature();
        self.estimate(source);
        self.output_final();
    }
}

fn main() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    let input = Input::read(&mut source);
    let mut solver = Solver::new(input);
    solver.solve(&mut source);
}
