fn flood_fill(matrix: &mut Vec<Vec<u8>>, x: usize, y: usize, new_color: u8) {
    if matrix[x][y] != new_color {
        fill_dfs(matrix, x, y, matrix[x][y], new_color)
    }
}

fn fill_dfs(matrix: &mut Vec<Vec<u8>>, x: usize, y: usize, old_color: u8, new_color: u8) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();

    if x >= rows || y >= cols {
        return;
    }

    if matrix[x][y] != old_color {
        return;
    }

    matrix[x][y] = new_color;
    fill_dfs(matrix, x + 1, y, old_color, new_color);
    fill_dfs(matrix, x, y + 1, old_color, new_color);
    if x != 0 {
        fill_dfs(matrix, x - 1, y, old_color, new_color);
    }
    if y != 0 {
        fill_dfs(matrix, x, y - 1, old_color, new_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let mut matrix: Vec<Vec<u8>> = vec![
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let expected: Vec<Vec<u8>> = vec![
            vec![0, 2, 2, 2, 0],
            vec![0, 0, 0, 2, 2],
            vec![0, 2, 2, 2, 0],
            vec![0, 2, 2, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        flood_fill(&mut matrix, 1, 3, 2);

        assert_eq!(matrix, expected);
    }
}
