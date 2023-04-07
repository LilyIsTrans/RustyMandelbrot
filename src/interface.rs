use num::complex::Complex;

fn apply_transform() {
    let scalar = 1;

    

    let matrix = arr2(&[
                                                [1, -3],
                                                [2, 4 ]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}
