use color_eyre::Result;

struct Grid {
    size: usize,
    heights: Vec<Vec<u8>>,
}

impl Grid {
    fn get_height(&self, row: usize, col: usize) -> Option<u8> {
        self.heights.get(row).and_then(|r| r.get(col)).copied()
    }

    fn lower_than_dir(&self, row: i8, col: i8, height: u8, dx: i8, dy: i8) -> bool {
        if row == 0 && dx < 0
            || col == 0 && dy < 0
            || col == (self.size - 1) as i8 && dy > 0
            || row == (self.size - 1) as i8 && dx > 0
        {
            return true;
        }
        let nextx = row + dx;
        let nexty = col + dy;
        let next_height = self.get_height(nextx as usize, nexty as usize).unwrap();
        if next_height >= height {
            return false;
        }
        self.lower_than_dir(nextx, nexty, height, dx, dy)
    }

    fn on_edge(&self, row: usize, col: usize) -> bool {
        row == 0 || col == 0 || row == (self.size - 1) || col == (self.size - 1)
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if self.on_edge(row, col) {
            return true;
        }
        let rowi8 = row as i8;
        let coli8 = col as i8;
        let cur_height = self.get_height(row, col).unwrap();

        self.lower_than_dir(rowi8, coli8, cur_height, -1, 0)
            || self.lower_than_dir(rowi8, coli8, cur_height, 1, 0)
            || self.lower_than_dir(rowi8, coli8, cur_height, 0, -1)
            || self.lower_than_dir(rowi8, coli8, cur_height, 0, 1)
    }

    fn count_until_lower(&self, row: usize, col: usize, dx: i8, dy: i8, height: u8) -> usize {
        if row == 0 && dx < 0
            || col == 0 && dy < 0
            || col == (self.size - 1) && dy > 0
            || row == (self.size - 1) && dx > 0
        {
            return 0;
        }
        let newrow = (row as i8 + dx) as usize;
        let newcol = (col as i8 + dy) as usize;
        let here_height = self.get_height(newrow, newcol).unwrap();
        if here_height >= height {
            return 1;
        }
        1 + self.count_until_lower(newrow, newcol, dx, dy, height)
    }

    fn trees_visible_score(&self, row: usize, col: usize) -> usize {
        let tree_height = self.get_height(row, col).unwrap();
        self.count_until_lower(row, col, -1, 0, tree_height)
            * self.count_until_lower(row, col, 1, 0, tree_height)
            * self.count_until_lower(row, col, 0, -1, tree_height)
            * self.count_until_lower(row, col, 0, 1, tree_height)
    }
}

fn parse_grid(s: &str) -> Grid {
    let lines = s.lines();
    let mut vec = Vec::new();
    for line in lines {
        let row: Vec<_> = line.chars().map(|x| x as u8 - '0' as u8).collect();
        vec.push(row);
    }
    let size = vec[0].len();
    Grid { size, heights: vec }
}

fn main() -> Result<()> {
    let grid = parse_grid(include_str!("../../day8.txt"));

    let mut visible_trees = 0;
    let mut max_visible_score = 0;
    for x in 0..grid.size {
        for y in 0..grid.size {
            if grid.is_visible(x, y) {
                println!("({x}, {y}) is visible");
                visible_trees += 1;
            }
            let score = grid.trees_visible_score(x, y);
            if score > max_visible_score {
                dbg!((x, y, score));
                max_visible_score = score;
            }
        }
    }

    println!("There are {visible_trees} visible in the grid");
    println!("Max visible score is {max_visible_score}");

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    fn example_grid() -> Grid {
        let grid = parse_grid(
            "30373
25512
65332
33549
35390",
        );
        grid
    }

    #[test]
    fn lower_than_dir_answers_right() {
        let grid = example_grid();

        assert_eq!(false, grid.lower_than_dir(1, 3, 1, -1, 0));
        assert_eq!(false, grid.lower_than_dir(1, 3, 1, 1, 0));
        assert_eq!(false, grid.lower_than_dir(1, 3, 1, 0, 1));
        assert_eq!(false, grid.lower_than_dir(1, 3, 1, 0, -1));
    }

    #[test]
    fn is_visible_answers_right() {
        let grid = example_grid();
        assert_eq!(false, grid.is_visible(1, 3));
        assert_eq!(true, grid.is_visible(2, 1));
    }

    #[test]
    fn count_until_lower() {
        let grid = example_grid();

        assert_eq!(1, grid.count_until_lower(1, 2, -1, 0, 5));
        assert_eq!(2, grid.count_until_lower(1, 2, 1, 0, 5));
        assert_eq!(1, grid.count_until_lower(1, 2, 0, -1, 5));
        assert_eq!(2, grid.count_until_lower(1, 2, 0, 1, 5));

        assert_eq!(1, grid.count_until_lower(1, 1, -1, 0, 5));
        assert_eq!(1, grid.count_until_lower(1, 1, 1, 0, 5));
        assert_eq!(1, grid.count_until_lower(1, 1, 0, -1, 5));
        assert_eq!(1, grid.count_until_lower(1, 1, 0, 1, 5));

        assert_eq!(1, grid.count_until_lower(2, 2, -1, 0, 3));
        assert_eq!(1, grid.count_until_lower(2, 2, 1, 0, 3));
        assert_eq!(1, grid.count_until_lower(2, 2, 0, -1, 3));
        assert_eq!(1, grid.count_until_lower(2, 2, 0, 1, 3));

        assert_eq!(2, grid.count_until_lower(3, 2, -1, 0, 5));
        assert_eq!(1, grid.count_until_lower(3, 2, 1, 0, 5));
        assert_eq!(2, grid.count_until_lower(3, 2, 0, -1, 5));
        assert_eq!(2, grid.count_until_lower(3, 2, 0, 1, 5));

        assert_eq!(1, grid.count_until_lower(0, 3, 0, 1, 7));
        assert_eq!(3, grid.count_until_lower(0, 3, 0, -1, 7));
        assert_eq!(0, grid.count_until_lower(0, 3, -1, 0, 7));
        assert_eq!(4, grid.count_until_lower(0, 3, 1, 0, 7));
    }

    #[test]
    fn trees_visible_score() {
        let grid = example_grid();

        assert_eq!(4, grid.trees_visible_score(1, 2));
        assert_eq!(8, grid.trees_visible_score(3, 2));
    }
}
