use aoc23::parse_input;
use ndarray::Array2;

fn main() {
    let input = parse_input("03.test");
    part1(input);
}

fn part1(input: String) {
    let map = Array2::<char>::default((10, 10));
    ///
    /// let us first mark the places that are 
    /// touched by the special symbol
    ///
    /// If the symbol is placed like this; 
    /// . . . .
    /// . & . .
    /// . . . . 
    /// 
    /// Then you mark the following places in map:
    /// x x x .
    /// x & x .
    /// x x x .
    /// 
    /// Now you have a all the points touched by the 
    /// special symbol. Now you parse the the numbers
    /// while maintaining the i pos and j pos you are
    /// at. If you encounter any 'x' at the i or j 
    /// then you mark the number and use it in sum.
    /// 
    /// The psuedo code looks like:
    /// 
    /// Create the map with touch points
    /// MAP = {
    ///     new_map = zeroes(HEIGHT, WIDTH)
    ///     for (i, line) in input.line.enumerate
    ///         for (j, char) in line.chars
    ///             if char.is_special
    ///                 mark_map(new_map, i, j)
    ///     new_map
    /// }
    /// 
    /// for i in 0..HEIGHT
    ///     for j in 0..WIDTH
    ///         let is_special = false
    ///         let num_string = String::new()
    ///         if input_map[i][j].is_num
    ///             num_string += input_map[i][j]
    ///             if MAP[i][j].is_an_X
    ///                 is_special = true
    ///         else
    ///             if num_string.len > 0 && is_special == true
    ///                 MAIN_SUM += to_num(num_string)
    ///             num_string.reset()
    ///             if num_string.len > 0 && is_special == true
    ///                 MAIN_SUM += to_num(num_string)
    ///             num_string.reset()
    ////             
}

fn get_adjacent_coordinates(abs_coord: usize, width: usize, height: usize) -> Vec<usize> {
    let i = abs_coord / width;
    let j = abs_coord % width;

    let mut adjacent = Vec::new();
    if i >= 1 {
        let top = abs_coord - width;
        adjacent.push(top);
        if j >= 1 {
            adjacent.push(top - 1)
        }
        if j < width - 1 {
            adjacent.push(top + 1)
        }
    }

    if j >= 1 {
        adjacent.push(abs_coord - 1)
    }
    if j < width - 1 {
        adjacent.push(abs_coord + 1)
    }

    if i < height - 1 {
        let bottom = abs_coord + width;
        adjacent.push(bottom);
        if j >= 1 {
            adjacent.push(bottom - 1)
        }
        if j < width - 1 {
            adjacent.push(bottom + 1)
        }
    }

    adjacent.sort();

    adjacent
}

#[test]
fn test() {
    let og = "467..114..";
    println!(
        "{:?}",
        og.split('.')
            .filter_map(|s| {
                let start = og.find(s).unwrap();
                let end = start + s.len();
                s.parse::<usize>().ok().map(|ss| (ss, (start, end)))
            })
            .collect::<Vec<(usize, (usize, usize))>>()
    );
}
