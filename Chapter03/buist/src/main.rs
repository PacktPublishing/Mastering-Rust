#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    beings: Option<Being>
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>
}
/*
impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square{ground: TerrainGround::Soil, block: None, beings: None});
        }

        Grid {
            size: (size_x, size_y),
            squares: squares
        }
    }
}
 */

impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 1..number_of_squares {
            squares.push(Square{ground: TerrainGround::Stone, block: None, beings: None});
        }

        Grid {
            size: (size_x, size_y),
            squares: squares
        };
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
          assert_eq!(square.ground, ::TerrainGround::Soil);
          assert_eq!(square.block, None);
          assert_eq!(square.beings, None);
          number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5*13);
        assert_eq!(number_of_squares, 5*13);

    }
}

