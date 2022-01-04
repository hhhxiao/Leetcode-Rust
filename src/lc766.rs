pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let mut res = true;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if row > 0 && col > 0 && matrix[row][col] != matrix[row - 1][col - 1] {
                res = false;
                break;
            }
        }
    }
    res
}
