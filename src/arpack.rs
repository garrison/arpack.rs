extern crate libc;
extern crate num;

use libc::{c_char, c_double, c_float, c_int};
use num::complex::Complex;

#[link(name = "arpack")]
extern {

    // single precision symmetric

    fn ssaupd_(ido: *mut c_int,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_float,
               resid: *mut c_float,
               ncv: *const c_int,
               v: *mut c_float,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_float,
               workl: *mut c_float,
               lworkl: *const c_int,
               info: *mut c_int);

    fn sseupd_(rvec: *const c_int,
               All: *const c_char,
               select: *mut c_int,
               d: *mut c_float,
               z: *mut c_float,
               ldz: *const c_int,
               sigma: *const c_float,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_float,
               resid: *mut c_float,
               ncv: *const c_int,
               v: *mut c_float,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_float,
               workl: *mut c_float,
               lworkl: *const c_int,
               ierr: *mut c_int);

    // double precision symmetric

    fn dsaupd_(ido: *mut c_int,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_double,
               resid: *mut c_double,
               ncv: *const c_int,
               v: *mut c_double,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_double,
               workl: *mut c_double,
               lworkl: *const c_int,
               info: *mut c_int);

    fn dseupd_(rvec: *const c_int,
               All: *const c_char,
               select: *mut c_int,
               d: *mut c_double,
               z: *mut c_double,
               ldz: *const c_int,
               sigma: *const c_double,
               bmat: *const c_char,
               n: *const c_int,
               which: *const c_char,
               nev: *const c_int,
               tol: *const c_double,
               resid: *mut c_double,
               ncv: *const c_int,
               v: *mut c_double,
               ldv: *const c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_double,
               workl: *mut c_double,
               lworkl: *const c_int,
               ierr: *mut c_int);

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
               All: *const c_char,
               select: *mut c_int,
               d: *mut Complex<c_float>,
               v: *mut Complex<c_float>,
               ldv: *const c_int,
               sigma: *const c_float,
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
               All: *const c_char,
               select: *mut c_int,
               d: *mut Complex<c_double>,
               v: *mut Complex<c_double>,
               ldv: *const c_int,
               sigma: *const c_double,
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
