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
    function: extern fn(f64) -> f64, 
    x_value: f64, 
    step_size: f64, 
    step_reduction_factor: f64,
    iterations: usize
)-> f64
{
    let mut result = 0f64;
    let mut iterations_performed = 0usize;
    let mut h = step_size;
    let mut prev_value = 0f64;
    let mut error: f64;
    loop{
        if iterations_performed == iterations{
            break result
        }
        result = (function(x_value + h) - function(x_value))/h;
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
    function: extern fn(f64) -> f64,
    x_value: f64,
    step_size: f64,
    step_reduction_factor: f64,
    iterations: usize
) -> f64
{
    let mut result = 0f64;
    let mut iterations_performed = 0usize;
    let mut h = step_size;
    let mut prev_value = 0f64;
    let mut error: f64;
    loop{
        if iterations_performed == iterations{
            break result
        }
        result = (function(x_value + h) - function(x_value - h))/(2.0 * h);
        iterations_performed += 1;
        error = result - prev_value;
        prev_value = result;
        println!("Error: {error}");
        println!("Current Value, f'(x): {result}");
        h *= step_reduction_factor;
    }
}

#[no_mangle]
pub extern "C" fn generic_io_fn() -> (){
    let mut buffer = String::new();
    println!("Hello World!");
    println!("Say hi back!");
    io::stdin().lock().read_line(&mut buffer).expect("Failed to use STDIN");
    println!("Your name is: {}", buffer);
}

#[cfg(test)]
mod test{
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
    
}