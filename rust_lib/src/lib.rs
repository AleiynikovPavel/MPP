use rayon::prelude::*;
use std::time::SystemTime;

use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::jintArray;
use jni::sys::jsize;
use jni::sys::jint;

#[no_mangle]
pub extern "system" fn Java_com_company_RustJni_sumPerfectSquare(env: JNIEnv,
                                             _class: JClass,
                                             input: jintArray)
                                             -> jint {
    let input_size: jsize =
        env.get_array_length(input).expect("Couldn't get java array size!").into();
    let mut vec :Vec<i32> = vec![0; input_size as usize];
        env.get_int_array_region(input, 0,  &mut vec).expect("Couldn't get java array!");
    let now = SystemTime::now();
    let sum = vec.iter()
        .sum();
    match now.elapsed() {
        Ok(elapsed) => {
            println!("sumPerfectSquare time: {} ns", elapsed.as_nanos());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    sum
}

#[no_mangle]
pub extern "system" fn Java_com_company_RustJni_sumPerfectSquarePar___3I(env: JNIEnv,
                                                                 _class: JClass,
                                                                 input: jintArray)
                                                                 -> jint {
    let input_size: jsize =
        env.get_array_length(input).expect("Couldn't get java array size!").into();
    let mut vec :Vec<i32> = vec![0; input_size as usize];
    env.get_int_array_region(input, 0,  &mut vec).expect("Couldn't get java array!");
    let now = SystemTime::now();
    let sum = vec.par_iter()
        .sum();
    match now.elapsed() {
        Ok(elapsed) => {
            println!("sumPerfectSquarePar time: {} ns", elapsed.as_nanos());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    sum
}

#[no_mangle]
pub extern "system" fn Java_com_company_RustJni_sumPerfectSquarePar___3II(env: JNIEnv,
                                                                 _class: JClass,
                                                                 input: jintArray, thread_count: jint)
                                                                 -> jint {
    let input_size: jsize =
        env.get_array_length(input).expect("Couldn't get java array size!").into();
    let mut vec :Vec<i32> = vec![0; input_size as usize];
    env.get_int_array_region(input, 0,  &mut vec).expect("Couldn't get java array!");
    let pool = rayon::ThreadPoolBuilder::new().num_threads(thread_count as usize).build().unwrap();
    let now = SystemTime::now();
    let sum = pool.install(|| {
        vec.par_iter()
            .sum()
    });
    match now.elapsed() {
        Ok(elapsed) => {
            println!("sumPerfectSquarePar with pool size: {} time: {} ns", thread_count, elapsed.as_nanos());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    sum
}