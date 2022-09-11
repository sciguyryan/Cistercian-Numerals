/// The size of the grid. This does not change the grid size.
const GRID_SIZE: usize = 15;

pub struct Numerals {
    pub grid: [[bool; 15]; 15],
}

impl Numerals {
    pub fn new(value: u32) -> Self {
        assert!(
            (0..=9999).contains(&value),
            "Value must be between 1 and 9999"
        );

        let mut grid = [[false; 15]; 15];
        for line in &mut grid {
            line[GRID_SIZE / 2] = true;
        }

        let mut s = Self { grid };
        s.draw(value);

        s
    }

    pub fn print(&self) {
        for line in &self.grid {
            let mut str = String::with_capacity(GRID_SIZE);
            for entry in line {
                if *entry {
                    str.push('#');
                } else {
                    str.push(' ');
                }
            }
            println!("{}", str);
        }
    }

    pub fn draw_horizontal(&mut self, col_start: usize, col_end: usize, row: usize) {
        for i in col_start..col_end {
            self.grid[row][i] = true;
        }
    }

    pub fn draw_vertical(&mut self, row_start: usize, row_end: usize, column: usize) {
        for i in row_start..row_end {
            self.grid[i][column] = true;
        }
    }

    pub fn draw_diagonal(&mut self, row_start: usize, row_end: usize, col_start: usize) {
        let rev = row_start > row_end;

        let mut start = row_start;
        let mut end = row_end;

        if rev {
            std::mem::swap(&mut start, &mut end);
        }

        let mut col = col_start;
        for i in start..end {
            self.grid[i][col] = true;

            if rev {
                // Sloping up.
                col -= 1;
            } else {
                // Sloping down.
                col += 1;
            }
        }
    }

    pub fn draw_ones(&mut self, value: u32) {
        match value {
            1 => {
                self.draw_horizontal(7, 12, 0);
            }
            2 => {
                self.draw_horizontal(7, 12, 4);
            }
            3 => {
                self.draw_diagonal(0, 5, 7);
            }
            4 => {
                self.draw_diagonal(4, 0, 11);
            }
            5 => {
                self.draw_ones(1);
                self.draw_ones(4);
            }
            6 => {
                self.draw_vertical(0, 5, 11);
            }
            7 => {
                self.draw_ones(1);
                self.draw_ones(6);
            }
            8 => {
                self.draw_ones(2);
                self.draw_ones(6);
            }
            9 => {
                self.draw_ones(1);
                self.draw_ones(8);
            }
            _ => {}
        }
    }

    pub fn draw_tens(&mut self, value: u32) {
        match value {
            1 => {
                self.draw_horizontal(3, 7, 0);
            }
            2 => {
                self.draw_horizontal(3, 7, 4);
            }
            3 => {
                self.draw_diagonal(5, 0, 7);
            }
            4 => {
                self.draw_diagonal(0, 5, 3);
            }
            5 => {
                self.draw_tens(1);
                self.draw_tens(4);
            }
            6 => {
                self.draw_vertical(0, 5, 3);
            }
            7 => {
                self.draw_tens(1);
                self.draw_tens(6);
            }
            8 => {
                self.draw_tens(2);
                self.draw_tens(6);
            }
            9 => {
                self.draw_tens(1);
                self.draw_tens(8);
            }
            _ => {}
        }
    }

    pub fn draw_hundreds(&mut self, value: u32) {
        match value {
            1 => {
                self.draw_horizontal(7, 12, 14);
            }
            2 => {
                self.draw_horizontal(7, 12, 10);
            }
            3 => {
                self.draw_diagonal(14, 10, 11);
            }
            4 => {
                self.draw_diagonal(10, 15, 7);
            }
            5 => {
                self.draw_hundreds(1);
                self.draw_hundreds(4);
            }
            6 => {
                self.draw_vertical(10, 15, 11); 
            }
            7 => {
                self.draw_hundreds(1);
                self.draw_hundreds(6);
            }
            8 => {
                self.draw_hundreds(2);
                self.draw_hundreds(6);
            }
            9 => {
                self.draw_hundreds(1);
                self.draw_hundreds(8);
            }
            _ => {}
        }
    }

    pub fn draw_thousands(&mut self, value: u32) {
        match value {
            1 => {
                self.draw_horizontal(3, 7, 14);
            }
            2 => {
                self.draw_horizontal(3, 7, 10);
            }
            3 => {
                self.draw_diagonal(10, 14, 3);
            }
            4 => {
                self.draw_diagonal(15, 10, 7);
            }
            5 => {
                self.draw_thousands(1);
                self.draw_thousands(4);
            }
            6 => {
                self.draw_vertical(10, 15, 3);
            }
            7 => {
                self.draw_thousands(1);
                self.draw_thousands(6);
            }
            8 => {
                self.draw_thousands(2);
                self.draw_thousands(6);
            }
            9 => {
                self.draw_thousands(1);
                self.draw_thousands(8);
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, value: u32) {
        let mut v = value;

        let thousands = v / 1000;
        v %= 1000;

        let hundreds = v / 100;
        v %= 100;

        let tens = v / 10;
        let ones = v % 10;

        if thousands > 0 {
            self.draw_thousands(thousands);
        }
        if hundreds > 0 {
            self.draw_hundreds(hundreds);
        }
        if tens > 0 {
            self.draw_tens(tens);
        }
        if ones > 0 {
            self.draw_ones(ones);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let testcases: Vec<(u32, &str)> = vec![
            (
                1,
                concat!(
                    "       #####   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                2,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #####   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                3,
                concat!(
                    "       #       ",
                    "       ##      ",
                    "       # #     ",
                    "       #  #    ",
                    "       #   #   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                4,
                concat!(
                    "       #   #   ",
                    "       #  #    ",
                    "       # #     ",
                    "       ##      ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                5,
                concat!(
                    "       #####   ",
                    "       #  #    ",
                    "       # #     ",
                    "       ##      ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                6,
                concat!(
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                7,
                concat!(
                    "       #####   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                8,
                concat!(
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #####   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                9,
                concat!(
                    "       #####   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #####   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                10,
                concat!(
                    "   #####       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                20,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #####       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                30,
                concat!(
                    "       #       ",
                    "      ##       ",
                    "     # #       ",
                    "    #  #       ",
                    "   #   #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                40,
                concat!(
                    "   #   #       ",
                    "    #  #       ",
                    "     # #       ",
                    "      ##       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                50,
                concat!(
                    "   #####       ",
                    "    #  #       ",
                    "     # #       ",
                    "      ##       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                60,
                concat!(
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                70,
                concat!(
                    "   #####       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                80,
                concat!(
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #####       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                90,
                concat!(
                    "   #####       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #####       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                100,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #####   ",
                ),
            ),
            (
                200,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #####   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                300,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #   #   ",
                    "       #  #    ",
                    "       # #     ",
                    "       ##      ",
                    "       #       ",
                ),
            ),
            (
                400,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       ##      ",
                    "       # #     ",
                    "       #  #    ",
                    "       #   #   ",
                ),
            ),
            (
                500,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       ##      ",
                    "       # #     ",
                    "       #  #    ",
                    "       #####   ",
                ),
            ),
            (
                600,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                ),
            ),
            (
                700,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #####   ",
                ),
            ),
            (
                800,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #####   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                ),
            ),
            (
                900,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #####   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #   #   ",
                    "       #####   ",
                ),
            ),
            (
                1000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #####       ",
                ),
            ),
            (
                2000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #####       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                ),
            ),
            (
                3000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #   #       ",
                    "    #  #       ",
                    "     # #       ",
                    "      ##       ",
                    "       #       ",
                ),
            ),
            (
                4000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "      ##       ",
                    "     # #       ",
                    "    #  #       ",
                    "   #   #       ",
                ),
            ),
            (
                5000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "      ##       ",
                    "     # #       ",
                    "    #  #       ",
                    "   #####       ",
                ),
            ),
            (
                6000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                ),
            ),
            (
                7000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #####       ",
                ),
            ),
            (
                8000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #####       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                ),
            ),
            (
                9000,
                concat!(
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #####       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #   #       ",
                    "   #####       ",
                ),
            ),
            (
                9999,
                concat!(
                    "   #########   ",
                    "   #   #   #   ",
                    "   #   #   #   ",
                    "   #   #   #   ",
                    "   #########   ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "       #       ",
                    "   #########   ",
                    "   #   #   #   ",
                    "   #   #   #   ",
                    "   #   #   #   ",
                    "   #########   ",
                ),
            ),
        ];

        for (value, t) in testcases {
            let encoder = Numerals::new(value);

            let mut str = String::new();
            for row in encoder.grid {
                for cell in row {
                    let c = if cell { '#' } else { ' ' };
                    str.push(c);
                }
            }

            assert_eq!(str, t, "Expected value did not match for value = {}", value);
        }
    }
}
