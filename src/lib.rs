#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, BufRead};

/// The forward difference formula is a means of finding the derivative of some function at some point.<br/>
/// To achive this, the function takes in a function pointer, `function`, which is a programmatic representation of some `f(x)`<br/>
/// An `x` value, which is the point at which we want the _slope_ of the `function`. A `step_size` which will determine an initial `h` value,
/// which is basically the distance away from the point at which the _slope_ is initially calculated. As `h` tends to 0, the accuracy of the calculation
/// increases, if the function is converging. The `step_reduction` changes the rate at which _h_ is decreases, and `iterations` gives how many times we loop over to find a
/// better value for `f(x)` <br/>
///
/// # Example
/// for the function: f (x) = e<sup>x
/// ```
/// use estimators::forward_difference_formula;
/// fn test_forward_difference(){
///     let function = |x: f64| x.exp();
///     let initial = 1.0;
///     let step_size = 1.0;
///     let step_reduction_factor = 0.1;
///     let iterations = 5;
///     forward_difference_formula(&function, initial, step_size, step_reduction_factor, iterations);
/// }
/// ```

#[no_mangle]
pub extern "C" fn forward_difference_formula(
    function: extern "C" fn(f64) -> f64,
    x_value: f64,
    step_size: f64,
    step_reduction_factor: f64,
    iterations: usize,
) -> f64 {
    let mut result = 0f64;
    let mut iterations_performed = 0usize;
    let mut h = step_size;
    let mut prev_value = 0f64;
    let mut error: f64;
    loop {
        if iterations_performed == iterations {
            break result;
        }
        result = (function(x_value + h) - function(x_value)) / h;
        iterations_performed += 1;
        error = result - prev_value;
        prev_value = result;
        //now we actually have to test for convergence;
        //we can have a sort of error buffer, were we alloc some memory and test if the error keeps going down or not
        println!("Error: {error}");
        println!("Current Value, f'(x): {result}");
        h *= step_reduction_factor;
    }
}

#[no_mangle]
pub extern "C" fn central_difference_formula(
    function: extern "C" fn(f64) -> f64,
    x_value: f64,
    step_size: f64,
    step_reduction_factor: f64,
    iterations: usize,
) -> f64 {
    let mut result = 0f64;
    let mut iterations_performed = 0usize;
    let mut h = step_size;
    let mut prev_value = 0f64;
    let mut error: f64;
    loop {
        if iterations_performed == iterations {
            break result;
        }
        result = (function(x_value + h) - function(x_value - h)) / (2.0 * h);
        iterations_performed += 1;
        error = result - prev_value;
        prev_value = result;
        println!("Error: {error}");
        println!("Current Value, f'(x): {result}");
        h *= step_reduction_factor;
    }
}

#[no_mangle]
pub extern "C" fn generic_io_fn() -> () {
    let mut buffer = String::new();
    println!("Hello World!");
    println!("Say hi back!");
    io::stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Failed to use STDIN");
    println!("Your name is: {}", buffer);
}

#[no_mangle]
pub unsafe extern "C" fn eulers_method(
    function: extern "C" fn(f64, f64) -> f64,
    buffer: *mut f64,
    range_low: f64,
    range_hi: f64,
    y_zero: f64,
    step_size: f64,
) {
    let iters = ((range_hi + range_low) / step_size) as usize;
    let mut current_range = range_low + step_size;
    let mut current_value = y_zero;
    let mut count = 0;
    //SAFTEY: assuming the calling client has sense enough to point me to a buffer that not null and contains f64s
    *buffer.add(count) = current_value;

    loop {
        if count == iters {
            break;
        }
        current_value = current_value + (step_size * function(current_range, current_value));
        current_range += step_size;
        //SAFTEY: assuming the calling client has sense enough to point me to a buffer thats not null and contains f64
        *buffer.add(count) = current_value;
        count += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn simpsons_method(
    function: extern fn(f64) -> f64,
    hi: f64,
    lo: f64,
    n_steps: usize
) -> f64 {
    let mut four_step = true;
    let step_size = (hi - lo)/n_steps as f64;
    let first_compute = step_size/ 3f64;
    let mut summation = 0f64;
    for step in 0 ..=n_steps{
        if step == 0 || step == n_steps{
            summation += function(lo + (step_size * step as f64));
            continue;
        }
        if four_step{
            summation += 4f64 * (function(lo + (step_size * step as f64)));
            four_step = false
        }
        else{
            summation += 2f64 * (function(lo + (step_size * step as f64)));
            four_step = true
        }
    }
    summation * first_compute
}


//assumes that the functions given have a non zero x1, x2, x3...so basically pass in a system of eqns and solve
//assumes that the slack have been given; we cannot introduce this because this will require reallocation of memory on our end, which we'll like to avoid through FFI
#[no_mangle]
pub unsafe extern "C" fn simplex_method_max(
    standard_form: *mut f64,
    standard_form_len: usize,
    standard_form_num_rows: usize,
    standard_form_soln: *mut f64,
    objective_fn: *mut f64,
){
    //assume that the len of the objective fn and the standard form solution len is standard form len/2
    loop{
        if check_non_negative(standard_form_soln, standard_form_len/2){
            break
        }
        let pivot_col = idxmax(standard_form_soln, (standard_form_len - 1)/2);
        let mut pivot_vector: Vec<f64> = Vec::new();
        (0 .. standard_form_num_rows).for_each(|x| pivot_vector.push(standard_form.add(standard_form_num_rows * x + pivot_col).read()));
        //not eagerly evaluated, we'll have to reloop to find the idxmin
        (0 .. standard_form_len).for_each(|x|{
            *standard_form_soln.add(x) = *standard_form_soln.add(x)/pivot_vector[x];
        });
        let pivot_row = idxmin(standard_form_soln, standard_form_num_rows);
        let pivot_value = standard_form.add(pivot_row * standard_form_num_rows + pivot_col).read();
        //perform pivoting here
        //reduce the pivot to 1, devide through by pivot_elem
        (0 .. standard_form_len).for_each(|x|{
            *standard_form.add(pivot_row * standard_form_num_rows + x) = *standard_form.add(pivot_row * standard_form_num_rows + x)/pivot_value;
        });
        //check the upper col if it is zero
        


    }
}

pub fn idxmax(vector: *mut f64, len: usize) -> usize{
    let vector = unsafe {
        Vec::from_raw_parts(vector, len, len)
    };
    (0 .. len).fold(0, |x, y|{
        if vector[x] == f64::max(vector[x].abs(), vector[y].abs()){
            return x
        }
        y
    })
}

pub fn idxmin(vector: *mut f64, len: usize) -> usize{
    let vector = unsafe {
        Vec::from_raw_parts(vector, len, len)
    };
    (0 .. len).fold(0, |x, y|{
        if vector[x] == f64::min(vector[x].abs(), vector[y].abs()){
            return x
        }
        y
    })

}

pub fn check_non_negative(vector: *mut f64, len: usize) -> bool{
    let vector = unsafe {
        Vec::from_raw_parts(vector, len, len)
    };
    let mut res = true;
    vector.iter().for_each(|x| if x.is_sign_negative(){
        res = false;
    });
    res
}


pub fn make_eye_vector(shape: usize) -> Vec<Vec<f64>>{
    let res = (0 .. shape).map(|x|{
        let mut vec = Vec::new();
        vec.resize(shape, 0f64);
        vec[x] = 1f64;
        vec
    }).collect::<Vec<Vec<f64>>>(); 
    res
}

#[cfg(test)]
mod test {
    //use super::*;

    // #[test]
    // fn test_forward_difference(){
    //     let fn_ptr = &|x| f64::exp(x);
    //     forward_difference_formula(fn_ptr, 1.0, 1.0, 0.1, 4);
    // }

    // #[test]
    // fn test_central_difference_formula(){
    //     let fn_ptr = &|x| f64::exp(x);
    //     central_difference_formula(fn_ptr, 1.0, 1.0, 0.1, 4);
    // }
    use super::*;
    #[test]
    fn test_eulers(){
        extern "C" fn e(x: f64) -> f64{
            x.exp()
        }
        let hi = 2f64;
        let lo = 1f64;
        let n_steps = 100;
        unsafe{
            dbg!(simpsons_method(e, hi, lo, n_steps));
            let var = dbg!(2f64.exp() - 1f64.exp());
        }
    }
}
