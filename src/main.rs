use std::{vec, thread};
use std::time::Instant;
use std::sync::{Arc,Mutex, MutexGuard};

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

fn solve_1d_mult_thread(left:&Vec<f64>,mut right:&Vec<f64>)->f64{
    let mut output: f64 = 0f64;
    let mut right_clone = right.clone();
    for l_and_r in left.iter().zip(right_clone.iter_mut()){
        output += *l_and_r.0 * *l_and_r.1;
    }
    return output;
}


#[allow(unused)]
pub(crate) fn multi_thread_mult(mat_left:&Vec<Vec<f64>>, mat_right:&Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut output_base = vec![Arc::new(Mutex::new(vec![0f64;MATRIX_SIZE]));MATRIX_SIZE];
    let mat_right_rot = rot_90(&mat_right);
    let mut output = Vec::<Vec::<f64>>::with_capacity(MATRIX_SIZE);
    let mut handles = Vec::with_capacity(MATRIX_SIZE);
    for i in 0..MATRIX_SIZE{
        let output_clone = output_base[i].clone();
            for j in 0..MATRIX_SIZE{
                let handle = thread::spawn(move||{
                    let left = mat_left[i].clone();
                    let right = &mat_right_rot[j].clone();
                    let result = solve_1d_mult(&left, &right);
                    let mut guard = match output_clone.lock(){
                        Ok(v) => v,
                        Err(s) => panic!{"{}",s}, //<-- Temp error handelig.
                    };
                    guard[j] = result;
                });
            handles.push(handle);

            }
        
    }
    for handle in handles{
        handle.join();
    }

    return vec![vec![0f64;1];1]; // TEMP OUTPUT
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
const MATRIX_SIZE: usize = 500;
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
    let no_thread_output = no_thread_mult(&mat_left,&mat_right);
    let no_thread_time = start_time_no_thread.elapsed();
    println!("no_thread_time: {}sec", (no_thread_time.as_millis()as f32 )/1000f32 );

    let start_time_mult_thread = Instant::now();
    #[allow(unused)]
    let mult_thread_output = multi_thread_mult(&mat_left,&mat_right);
    let mult_thread_time = start_time_mult_thread.elapsed();
    println!("mult_thread_time: {}sec", (mult_thread_time.as_millis()as f32 )/1000f32 );
    
    //print_vec_of_vec(&no_thread_output);
    //print_vec_of_vec(&mult_thread_output);

    /*
    let temp = vec![ vec![1f64,2f64,3f64],
                                    vec![4f64,5f64,6f64],
                                    vec![7f64,8f64,9f64]];
    */
}
