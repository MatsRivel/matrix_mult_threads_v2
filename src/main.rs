pub(crate) fn no_thread_mult(mat_left:Vec<Vec<f64>>, mat_right:Vec<Vec<f64>>, mat_size:usize) -> Vec<Vec<f64>>{
    // TODO: Only works for nxn X nxn matrices. Should check for compatability first.
    let mut output:Vec<Vec<f64>> = vec![vec![0f64;mat_size as usize];mat_size as usize];
    for i in 0..mat_size as usize{
        for j in 0..mat_size as usize{
            for k in 0..mat_size as usize{
                output[i][j] += mat_left[i][k] * mat_right[k][j];
            }
        }
    }
    return output;
}

pub(crate) fn print_vec_of_vec(vec_in:Vec<Vec<f64>>){
    for line in vec_in{
        for element in line{
            print!("{:.0} ",element);
        }
        println!();
    }
    println!();
}

fn solve_1d_mult(left:Vec<f64>,mut right:Vec<f64>)->f64{
    let mut output: f64 = 0f64;
    for l_and_r in left.iter().zip(right.iter_mut()){
        output += *l_and_r.0 * *l_and_r.1;
    }
    return output;
}



pub(crate) fn multi_thread_mult(mat_left:Vec<Vec<f64>>, mat_right:Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut output = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mut mat_right_rot = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    for i in 0..MATRIX_SIZE{
        for j in 0..MATRIX_SIZE{
            mat_right_rot[i][j] = mat_right[j][i]; // TODO: Not correct rotating of matrix. Should be fixed.
        }
    }

    for i in 0..MATRIX_SIZE{
        let left = mat_left[i].clone();
        for j in 0..MATRIX_SIZE{
            let right = mat_right_rot[j].clone();
            output[i][j] = solve_1d_mult(left.clone(), right);
        }
    }

    return output;
}

const MATRIX_SIZE: usize = 20;
fn main() {
    let mat_left  = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mat_right = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let no_thread_output = no_thread_mult(mat_left,mat_right,MATRIX_SIZE);
    print_vec_of_vec(no_thread_output);



}
