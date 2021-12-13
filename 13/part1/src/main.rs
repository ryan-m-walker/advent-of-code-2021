fn main() {
    let dots = include_str!("./dots.txt");
    let folds = parse_folds(include_str!("./folds.txt"));

    let mut paper = Paper::from_str(dots);
    paper.fold(folds);
    let count = paper.get_point_count();
    println!("Answer = {}", count);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Point {
    Empty,
    Dot,
}

#[derive(Debug, Clone)]
struct Paper {
    points: Vec<Point>,
    cols: u32,
    rows: u32,
}

impl Paper {
    fn from_str(input: &str) -> Self {
        let mut cols: u32 = 0;
        let mut rows: u32 = 0;

        let coords: Vec<_> = input
            .lines()
            .map(|p| {
                let split: Vec<_> = p.split(",").collect();
                let x: u32 = split[0].parse().unwrap();
                let y: u32 = split[1].parse().unwrap();

                if x + 1 > cols {
                    cols = x + 1;
                }

                if y + 1 > rows {
                    rows = y + 1;
                }

                (x, y)
            })
            .collect();

        let len = (rows * cols + cols) as usize;
        let mut points = vec![Point::Empty; len];

        for (x, y) in &coords {
            let index = get_index(*x, *y, cols);

            let coords = get_coords(index as u32, cols);
            assert_eq!(&coords.0, x);
            assert_eq!(&coords.1, y);

            points[index] = Point::Dot;
        }

        Self { points, rows, cols }
    }

    fn print(&self) {
        print!("\n");

        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = get_index(col, row, self.cols);
                let point = self.points[index];

                match point {
                    Point::Dot => print!("#"),
                    Point::Empty => print!("."),
                }
            }
            print!("\n");
        }

        print!("\n");
        print!("\n");
    }

    fn fold(&mut self, folds: Vec<Fold>) {
        let (fold_dir, fold_loc) = folds[0];

        let mut coords = vec![];

        for i in 0..self.points.len() {
            let point = self.points[i];

            if let Point::Dot = point {
                let (x, y) = get_coords(i as u32, self.cols);

                let mut new_x = x;
                let mut new_y = y;

                if fold_dir == FoldDirection::X && x > fold_loc {
                    let diff = x - fold_loc;
                    new_x = fold_loc - diff;
                }

                if fold_dir == FoldDirection::Y && y > fold_loc {
                    let diff = y - fold_loc;
                    new_y = fold_loc - diff;
                }

                coords.push((new_x, new_y));
            }
        }

        if fold_dir == FoldDirection::Y {
            self.rows = fold_loc;
        }

        if fold_dir == FoldDirection::X {
            self.cols = fold_loc;
        }

        let len = (self.rows * self.cols + self.cols) as usize;
        let mut new_points = vec![Point::Empty; len];

        for (x, y) in &coords {
            let index = get_index(*x, *y, self.cols);
            new_points[index] = Point::Dot;
        }

        self.points = new_points;
    }

    fn get_point_count(&self) -> u32 {
        self.points.iter().filter(|p| **p == Point::Dot).count() as u32
    }
}

fn get_index(x: u32, y: u32, cols: u32) -> usize {
    (y * cols + x) as usize
}

fn get_coords(index: u32, cols: u32) -> (u32, u32) {
    let x = index % cols;
    let y = index / cols;

    (x, y)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FoldDirection {
    X,
    Y,
}

type Fold = (FoldDirection, u32);

fn parse_folds(input: &str) -> Vec<Fold> {
    input
        .lines()
        .map(|l| {
            let line = str::replace(l, "fold along ", "");
            let components: Vec<_> = line.split("=").collect();
            let dir = components[0];
            let loc: u32 = components[1].parse().unwrap();

            let fold_direction = if dir == "x" {
                FoldDirection::X
            } else {
                FoldDirection::Y
            };
            (fold_direction, loc)
        })
        .collect()
}
