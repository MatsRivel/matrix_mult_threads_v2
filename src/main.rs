use std::vec;
use std::time::Instant;
#[allow(unused_imports)]
use tokio::runtime::Runtime;
#[allow(unused_imports)]
use tokio::task;
#[allow(unused_imports)]
use tokio::time::sleep;
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread::{self, Thread};
use std::sync::mpsc;

#[allow(unused)]
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

#[allow(unused)]
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

    output
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

fn solve_1d_thread(x: &[f64], y: &[f64], output: Arc<Mutex<Vec<Vec<f64>>>>, pos: (usize,usize)) {
    let mut result = 0f64;
    for n in 0..MATRIX_SIZE {
        result += x[n] * y[n];
    }
    let mut output = output.lock().unwrap();
    output[pos.0][pos.1] = result;
}


fn multi_thread_pool(x: &[Vec<f64>], y: &[Vec<f64>] ) -> Vec<Vec<f64>> {
    const NUM_THREADS:usize = 8;
    let len = x[0].len();
    let mut threads = Vec::new();
    let output = Arc::new(Mutex::new(vec![vec![0.0; len];len]));
    let mut txs = Vec::with_capacity(NUM_THREADS);
    for _ in 0..NUM_THREADS{
        let (tx, rx) = mpsc::channel::<>();
        txs.push(tx);
        let output = output.clone();
        let thread = thread::spawn(move||loop{
            let (x_in, y_in, i): (Vec<f64>, Vec<Vec<f64>>, usize)= match rx.recv().unwrap(){
                Some((x_in, y_in, i)) => (x_in, y_in, i),
                None => break,
            };
            let xi_clone = x_in.clone();
            let y_clone = y_in.clone();

            for j in 0..MATRIX_SIZE{
                //println!("{:?} | {:?}",xi_clone,y_clone);
                let yj_clone = y_clone[j].clone();
                solve_1d_thread(&xi_clone, &yj_clone, output.clone(), (i,j));
            }
        });
        threads.push(thread);
    }
    let mut i = 0;
    while i < MATRIX_SIZE{
        let xi_owned = x[i].to_owned();
        let y_owned = y.to_owned();
        match txs[i%NUM_THREADS].send(Some((xi_owned, y_owned, i))){
            Ok(()) => {},
            Err(s) => panic!("{}",s)
        };
        i+=1;
    }
    for tx in txs{
        match tx.send(None){
            Ok(()) => {},
            Err(s) => panic!("{}",s)
        };
    }
    for t in threads {
        t.join().unwrap();
    }

    Arc::try_unwrap(output)
        .expect("Mutex was poisoned")
        .into_inner()
        .expect("Mutex was empty")
}

#[allow(unused)]
fn multi_thread(x: &[Vec<f64>], y: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let num_threads = 4;
    let len = x[0].len();
    let output = Arc::new(Mutex::new(vec![vec![0.0; len];len]));

    let mut threads = Vec::with_capacity(num_threads);
    for _tid in 0..num_threads {
        let output = output.clone();
        let x = x.to_vec();
        let y = y.to_vec();

        threads.push(thread::spawn(move || {
            for i in 0..x.len() {
                for j in 0..y.len() {
                    solve_1d_thread(&x[i], &y[j], output.clone(), (i,j));
                }
            }
        }));
        
    }

    for t in threads {
        t.join().unwrap();
    }

    Arc::try_unwrap(output).unwrap().into_inner().unwrap()

}

const MATRIX_SIZE: usize = 2000;
fn main() {
    let mut mat_left  = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    let mut mat_right = vec![vec![0f64;MATRIX_SIZE];MATRIX_SIZE];
    for i in 0..MATRIX_SIZE{
        for j in 0..MATRIX_SIZE{
            mat_left[i][j]  = (i*10 + j) as f64;
            mat_right[i][j] = (i%3  + j) as f64;
        }
    }
    /*
    let start_time_no_thread = Instant::now();
    #[allow(unused)]
    let no_thread_output = no_thread_mult(&mat_left,&mat_right);
    let no_thread_time = start_time_no_thread.elapsed();
    println!("no_thread_time: {:3}sec", (no_thread_time.as_millis()as f32 )/1000f32 );


    let start_time_async = Instant::now();
    let rt = Runtime::new().unwrap();
    #[allow(unused)]
    let async_output = rt.block_on(multi_async(&mat_left,&mat_right));
    let async_time = start_time_async.elapsed();
    println!("async_time: {:.3}sec", (async_time.as_millis()as f32/1000f32 ));
     */
    let start_time_thread = Instant::now();
    #[allow(unused)]
    let thread_output = multi_thread_pool(&mat_left,&mat_right);
    let thread_time = start_time_thread.elapsed();
    println!("thread_time: {:.3}sec", (thread_time.as_millis()as f32/1000f32 ));

    /* 
    if MATRIX_SIZE < 5{
        print_vec_of_vec(&no_thread_output);
        print_vec_of_vec(&async_output);
        print_vec_of_vec(&thread_output);
    }
    */

}
