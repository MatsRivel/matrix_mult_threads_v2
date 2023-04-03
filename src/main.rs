use std::vec;
use std::time::Instant;
#[allow(unused_imports)]
use tokio::time::sleep;
//use futures::stream::StreamExt;

pub(crate) fn no_thread_mult(mat_left:&Vec<Vec<f64>>, mat_right:&Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut output = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mat_right_rot = rot_90(&mat_right);

    for i in 0..MATRIX_SIZE{
        let left = &mat_left[i];
        for j in 0..MATRIX_SIZE{
            let right = &mat_right_rot[j];
            output[i][j] = solve_1d_mult(left, right);
        }
    }

    return output;
}
#[allow(unused)]

pub(crate) fn print_vec_of_vec(vec_in:&Vec<Vec<f64>>){
    for line in vec_in{
        for element in line{
            print!("{:.0} ",element);
        }
        println!();
    }
    println!();
}
#[allow(unused)]

fn solve_1d_mult(left:&Vec<f64>,mut right:&Vec<f64>)->f64{
    let mut output: f64 = 0f64;
    let mut right_clone = right.clone();
    for l_and_r in left.iter().zip(right_clone.iter_mut()){
        output += *l_and_r.0 * *l_and_r.1;
    }
    return output;
}

#[allow(dead_code)]
async fn solve_1d_async(left:&Vec<f64>,right:&Vec<f64>)->f64{
    let mut output: f64 = 0f64;
    let mut right_clone = right.clone();
    for l_and_r in left.iter().zip(right_clone.iter_mut()){
        let l = l_and_r.0;
        let r= l_and_r.1;

        output += *l * *r;
    }
    return output;
}


#[allow(unused)]
pub(crate) async fn multi_async(mat_left:&Vec<Vec<f64>>, mat_right:&Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut output = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mat_right_rot = rot_90(&mat_right);
    for i in 0..MATRIX_SIZE{
        for j in 0..MATRIX_SIZE{
            let left = &mat_left[i];
            let right = &mat_right_rot[j];
            output[i][j] = solve_1d_async(left, right).await;
        }
    }

    return output;
}

#[allow(unused)]
fn rot_90(mat_in:&Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut vec_out = vec![vec![0f64;mat_in.len()];mat_in[0].len()];
    for i in 0..mat_in.len(){
        for j in 0..mat_in[0].len(){
            vec_out[i][j] = mat_in[j][i]; // TODO: Not correct rotating of matrix. Should be fixed.
        }
    }
    return vec_out;
}
const MATRIX_SIZE: usize = 10000;
fn main() {
    let mut mat_left  = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mut mat_right = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    for i in 0..MATRIX_SIZE{
        for j in 0..MATRIX_SIZE{
            mat_left[i][j] = (i*10 + j) as f64;
            mat_right[i][j] = (i%3 + j) as f64;
        }
    }
    let start_time_no_thread = Instant::now();
    #[allow(unused)]
    //let no_thread_output = no_thread_mult(&mat_left,&mat_right);
    let no_thread_time = start_time_no_thread.elapsed();
    //println!("no_thread_time: {:3}sec", (no_thread_time.as_millis()as f32 )/1000f32 );

    let start_time_mult_thread = Instant::now();
    #[allow(unused)]
    let async_output = multi_async(&mat_left,&mat_right);
    let async_time = start_time_mult_thread.elapsed();
    println!("async_time: {:.3}ms", (async_time.as_millis()as f32 ));
    
    //print_vec_of_vec(&no_thread_output);
    //print_vec_of_vec(&mult_thread_output);

    /*
    let temp = vec![ vec![1f64,2f64,3f64],
                                    vec![4f64,5f64,6f64],
                                    vec![7f64,8f64,9f64]];
    */
}
