fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Loop through each row and column of the matrix using the FOR loop
    // for row in 0..rows {
    //     for col in 0..cols {
    //         println!(
    //             "Element at row {} and column {}: {}",
    //             row, col, matrix[row][col]
    //         );
    //     }
    // }

    // Loop through each row and column of the matrix using the WHILE loop
    // let mut row = 0;
    // let mut col;

    // while row < rows {
    //     col = 0;

    //     while col < cols {
    //         println!(
    //             "Element at row {} and col {}: {}",
    //             row, col, matrix[row][col]
    //         );
    //         col += 1;
    //     }

    //     row += 1;
    // }


    // Initialize row and column indices
    let mut row = 0;
    let mut col = 0;
    
    // // Loop through each row and column of the matrix using the LOOP loop
    loop {
        println!(
            "Element at row {} and column {}: {}",
            row, col, matrix[row][col]
        );

        col += 1;

        if col == cols {
            col = 0;
            row += 1;

            if row == rows {
                break;
            }
        }
    }
    
}