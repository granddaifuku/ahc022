use proconio::{derive_readable, input, source::line::LineSource};
use std::io::{stdin, BufReader, Stdin};

#[derive_readable]
#[derive(Debug)]
struct ExitCell {
    x: usize,
    y: usize,
}

#[allow(non_snake_case)]
struct Input {
    L: usize,
    N: usize,
    S: usize,
    exit_cells: Vec<ExitCell>,
}

impl Input {
    #[allow(non_snake_case)]
    fn read(source: &mut LineSource<BufReader<Stdin>>) -> Self {
        input! {
            from source,
            L: usize,
            N: usize,
            S: usize,
            exit_cells: [ExitCell; N],
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
}

impl Solver {
    fn new(input: Input) -> Self {
        let mut temperature = vec![vec![0; input.L]; input.L];
        for i in 0..input.N {
            temperature[input.exit_cells[i].x][input.exit_cells[i].y] = i as i32 * 10;
        }
        let estimate = vec![0; input.N];
        Solver {
            input,
            temperature,
            estimate,
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
            let measured = self.measure(i, 0, 0, source);
            let mut diff = 9999;
            for (j, cell) in self.input.exit_cells.iter().enumerate() {
                let d = (self.temperature[cell.x][cell.y] - measured).abs();
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
