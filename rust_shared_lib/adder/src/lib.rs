use core::slice;
use std::os::raw::c_void;

use log::info;
// use zfp_sys::*;

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    println!("Rust says hello to C++");
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn compress_array(number_lines: i32, data: *mut f32) -> i32 {
    println!("RUST PRINTLN compress_array got into");
    let q = 6 * number_lines;
    // let res = env_logger::try_init();
    // if res.is_err(){
    //     info!("Logger cannot be set anew because : {:#?}", res.err().unwrap().to_string());
    // }
    // println!("RUST PRINTLN compress_array try_init");

    // let slice_before = slice::from_raw_parts(data, 20);
    // for i in 0..slice_before.len() {
    //     info!("Element with id {i:?} is {:?}", slice_before[i])
    // }
    // println!("RUST PRINTLN compress_array slice::from_raw_parts(");

    // let nx = 30;
    // let ny = 30;
    // let nz = 30;
    // let nt = 30;
    // let _tolerance = 256;
    // let stream = zfp_stream_open(std::ptr::null_mut());
    // println!("RUST PRINTLN compress_array zfp_stream_open");

    // zfp_stream_set_reversible(stream);
    // println!("RUST PRINTLN compress_array zfp_stream_set_reversible");

    // let temp_data_type = zfp_type_zfp_type_float;
    // let field = zfp_field_4d(data as *mut c_void, temp_data_type, nx, ny, nz, nt);
    // println!("RUST PRINTLN compress_array zfp_field_4d");

    // zfp_field_set_pointer(field, data as *mut c_void);
    // println!("RUST PRINTLN compress_array zfp_field_set_pointer");

    // let bufsize = zfp_stream_maximum_size(stream, field);
    // println!("RUST PRINTLN compress_array zfp_stream_maximum_size");

    // let mut buffer: Vec<u8> = vec![0; bufsize as usize];

    // let bstr = stream_open(buffer.as_mut_ptr() as *mut c_void, bufsize);
    // println!("RUST PRINTLN compress_array stream_open");

    // zfp_stream_set_bit_stream(stream, bstr);
    // println!("RUST PRINTLN compress_array zfp_stream_set_bit_stream");

    // zfp_compress(stream, field);
    // info!("Compress assumed successfull");
    // println!("RUST PRINTLN compress_array zfp_compress");

    // stream_close(bstr);

    // let compressed_size = zfp_stream_compressed_size(stream);
    // info!("Compressed size of data is : {compressed_size:?}");
    // println!("RUST PRINTLN compress_array zfp_stream_compressed_size");

    // let bstr = stream_open(buffer.as_mut_ptr() as *mut c_void, compressed_size);
    // println!("RUST PRINTLN compress_array stream_open");

    // zfp_stream_set_bit_stream(stream, bstr);
    // println!("RUST PRINTLN compress_array zfp_stream_set_bit_stream");

    // zfp_decompress(stream, field);
    // info!("Decompress assumed successfull");
    // println!("RUST PRINTLN compress_array zfp_decompress");

    // let decompressed_data = zfp_field_pointer(field) as *mut f32;
    // println!("RUST PRINTLN compress_array zfp_field_pointer");

    // let slice_after = slice::from_raw_parts(decompressed_data, 20);
    // for i in 0..slice_after.len() {
    //     info!("Element with id {i:?} is {:?}", slice_after[i])
    // }
    // println!("RUST PRINTLN compress_array slice::from_raw_parts");

    // stream_close(bstr);
    // println!("RUST PRINTLN compress_array stream_close");

    // zfp_field_free(field);
    // println!("RUST PRINTLN compress_array zfp_field_free");

    // zfp_stream_close(stream);
    // println!("RUST PRINTLN compress_array zfp_stream_close");

    return 0;

    // //based on examples/simple.c
    // let nx = 100;
    // let ny = 100;
    // let nz = 100;

    // let mut array: Vec<f64> = vec![0.0; nx * ny * nz];

    // for i in 0..nx {
    //     for j in 0..ny {
    //         for k in 0..nz {
    //             let x = 2.0 * (i as f64) / (nx as f64);
    //             let y = 2.0 * (j as f64) / (ny as f64);
    //             let z = 2.0 * (k as f64) / (nz as f64);
    //             array[i + nx * (j + ny * k)] = (-(x * x + y * y + z * z)).exp();
    //         }
    //     }
    // }

    // info!("log 1");
    // //println!("original data sample: {:?}", &array[0..nx]);

    // //compression
    // /* allocate meta data for the 3D array a[nz][ny][nx] */
    // let data_type = zfp_type_zfp_type_double;
    // let field = unsafe {
    //     zfp_field_3d(
    //         array.as_mut_ptr() as *mut std::ffi::c_void,
    //         data_type,
    //         nx as u32,
    //         ny as u32,
    //         nz as u32,
    //     )
    // };

    // /* allocate meta data for a compressed stream */
    // let zfp = unsafe { zfp_stream_open(std::ptr::null_mut() as *mut bitstream) };

    // /* set compression mode and parameters via one of three functions */
    // unsafe { zfp_stream_set_rate(zfp, 8.0, data_type, 3, 0) };
    // /*  zfp_stream_set_precision(zfp, precision); */
    // //let tolerance = 1.0e-3;
    // //unsafe { zfp_stream_set_accuracy(zfp, tolerance) };

    // #[cfg(feature = "cuda")]
    // {
    //     let ret = unsafe { zfp_stream_set_execution(zfp, zfp_exec_policy_zfp_exec_cuda) };

    //     if ret == 0 {
    //         //println!("failed to set the execution policy to zfp_exec_cuda");
    //         info!("log not reached");
    //         assert!(false);
    //     }
    // }

    // /* allocate buffer for compressed data */
    // let bufsize = unsafe { zfp_stream_maximum_size(zfp, field) };
    // let mut buffer: Vec<u8> = vec![0; bufsize as usize];

    // /* associate bit stream with allocated buffer */
    // let stream = unsafe { stream_open(buffer.as_mut_ptr() as *mut std::ffi::c_void, bufsize) };
    // unsafe {
    //     zfp_stream_set_bit_stream(zfp, stream);
    //     zfp_stream_rewind(zfp);
    // }

    // /* compress array and output compressed stream */
    // let zfpsize = unsafe { zfp_compress(zfp, field) };
    // if zfpsize == 0 {
    //     info!("log compression failed");
    //     //println!("compression failed");
    //     assert!(false);
    // } else {
    //     let original_size = nx * ny * nz * std::mem::size_of::<f64>();
    //     let ratio = (original_size as f64) / (zfpsize as f64);

    //     info!("log 2");
    //     // println!(
    //     //     "bufsize: {} bytes, original size: {} bytes, compressed size: {} bytes, ratio: {}",
    //     //     bufsize, original_size, zfpsize, ratio
    //     // );
    // }

    // /* rewind compressed stream and decompress array */
    // unsafe { zfp_stream_rewind(zfp) };
    // let ret = unsafe { zfp_decompress(zfp, field) };
    // if ret == 0 {
    //     info!("log decompression failed");
    //     //println!("decompression failed");
    //     assert!(false);
    // } else {
    //     info!("log 3");
    //     //println!("ret: {}", ret);
    // }

    // info!("log 4");
    // //println!("decompressed data sample: {:?}", &array[0..nx]);

    // /* clean up */
    // unsafe {
    //     zfp_field_free(field);
    //     zfp_stream_close(zfp);
    //     stream_close(stream);
    // }

    // assert!(true);
}
