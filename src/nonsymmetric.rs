// Copyright 2014 James R. Garrison
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;
extern crate num;
extern crate test;

use self::libc::{c_char, c_double, c_float, c_int};
use self::num::complex::Complex;

#[link(name = "arpack")]
extern {

    // single precision non-symmetric

    // fixme: fn snaupd_();
    // fixme: fn sneupd_();

    // double precision non-symmetric

    // fixme: fn dnaupd_();
    // fixme: fn dneupd_();

    // single precision complex

    fn cnaupd_(ido: *mut c_int,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_float,
               resid: *mut Complex<c_float>,
               ncv: *const c_int,
               v: *mut Complex<c_float>,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex<c_float>,
               workl: *mut Complex<c_float>,
               lworkl: *const c_int,
               rwork: *mut c_float,
               info: *mut c_int);

    fn cneupd_(rvec: *const c_int,
               howmny: *const c_char,
               select: *mut c_int,
               d: *mut Complex<c_float>,
               v: *mut Complex<c_float>,
               ldv: *const c_int,
               sigma: *const Complex<c_float>,
               workev: *mut Complex<c_float>,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_float,
               resid: *mut Complex<c_float>,
               ncv: *const c_int,
               v: *mut Complex<c_float>,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex<c_float>,
               workl: *mut Complex<c_float>,
               lworkl: *const c_int,
               rwork: *mut c_float,
               ierr: *mut c_int);

    // double precision complex

    fn znaupd_(ido: *mut c_int,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_double,
               resid: *mut Complex<c_double>,
               ncv: *const c_int,
               v: *mut Complex<c_double>,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex<c_double>,
               workl: *mut Complex<c_double>,
               lworkl: *const c_int,
               rwork: *mut c_double,
               info: *mut c_int);

    fn zneupd_(rvec: *const c_int,
               howmny: *const c_char,
               select: *mut c_int,
               d: *mut Complex<c_double>,
               v: *mut Complex<c_double>,
               ldv: *const c_int,
               sigma: *const Complex<c_double>,
               workev: *mut Complex<c_double>,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_double,
               resid: *mut Complex<c_double>,
               ncv: *const c_int,
               v: *mut Complex<c_double>,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex<c_double>,
               workl: *mut Complex<c_double>,
               lworkl: *const c_int,
               rwork: *mut c_double,
               ierr: *mut c_int);

}

pub enum Which {
    LargestMagnitude,
    SmallestMagnitude,
    LargestRealPart,
    SmallestRealPart,
    LargestImaginaryPart,
    SmallestImaginaryPart,
}

fn which_to_str(which: Which) -> &'static str {
    match which {
        Which::LargestMagnitude => "LM",
        Which::SmallestMagnitude => "SM",
        Which::LargestRealPart => "LR",
        Which::SmallestRealPart => "SR",
        Which::LargestImaginaryPart => "LI",
        Which::SmallestImaginaryPart => "SI",
    }
}
