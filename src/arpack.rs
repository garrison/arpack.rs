extern crate libc;
extern crate num;

use libc::{c_char, c_double, c_float, c_int};
use num::complex::{Complex32, Complex64};

#[link(name = "arpack")]
extern {

    // single precision symmetric

    fn ssaupd_(ido: *mut c_int,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_float,
               resid: *mut c_float,
               ncv: *mut c_int,
               v: *mut c_float,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_float,
               workl: *mut c_float,
               lworkl: *mut c_int,
               info: *mut c_int);

    fn sseupd_(rvec: *mut c_int,
               All: *mut c_char,
               select: *mut c_int,
               d: *mut c_float,
               z: *mut c_float,
               ldz: *mut c_int,
               sigma: *mut c_float,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_float,
               resid: *mut c_float,
               ncv: *mut c_int,
               v: *mut c_float,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_float,
               workl: *mut c_float,
               lworkl: *mut c_int,
               ierr: *mut c_int);

    // double precision symmetric

    fn dsaupd_(ido: *mut c_int,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_double,
               resid: *mut c_double,
               ncv: *mut c_int,
               v: *mut c_double,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_double,
               workl: *mut c_double,
               lworkl: *mut c_int,
               info: *mut c_int);

    fn dseupd_(rvec: *mut c_int,
               All: *mut c_char,
               select: *mut c_int,
               d: *mut c_double,
               z: *mut c_double,
               ldz: *mut c_int,
               sigma: *mut c_double,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_double,
               resid: *mut c_double,
               ncv: *mut c_int,
               v: *mut c_double,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut c_double,
               workl: *mut c_double,
               lworkl: *mut c_int,
               ierr: *mut c_int);

    // single precision non-symmetric

    // fixme: fn snaupd_();
    // fixme: fn sneupd_();

    // double precision non-symmetric

    // fixme: fn dnaupd_();
    // fixme: fn dneupd_();

    // single precision complex

    fn cnaupd_(ido: *mut c_int,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_float,
               resid: *mut Complex32,
               ncv: *mut c_int,
               v: *mut Complex32,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex32,
               workl: *mut Complex32,
               lworkl: *mut c_int,
               rwork: *mut c_float,
               info: *mut c_int);

    fn cneupd_(rvec: *mut c_int,
               All: *mut c_char,
               select: *mut c_int,
               d: *mut Complex32,
               v: *mut Complex32,
               ldv: *mut c_int,
               sigma: *mut c_float,
               workev: *mut Complex32,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_float,
               resid: *mut Complex32,
               ncv: *mut c_int,
               v: *mut Complex32,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex32,
               workl: *mut Complex32,
               lworkl: *mut c_int,
               rwork: *mut c_float,
               ierr: *mut c_int);

    // double precision complex

    fn znaupd_(ido: *mut c_int,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_double,
               resid: *mut Complex64,
               ncv: *mut c_int,
               v: *mut Complex64,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex64,
               workl: *mut Complex64,
               lworkl: *mut c_int,
               rwork: *mut c_double,
               info: *mut c_int);

    fn zneupd_(rvec: *mut c_int,
               All: *mut c_char,
               select: *mut c_int,
               d: *mut Complex64,
               v: *mut Complex64,
               ldv: *mut c_int,
               sigma: *mut c_double,
               workev: *mut Complex64,
               bmat: *mut c_char,
               n: *mut c_int,
               which: *mut c_char,
               nev: *mut c_int,
               tol: *mut c_double,
               resid: *mut Complex64,
               ncv: *mut c_int,
               v: *mut Complex64,
               ldv: *mut c_int,
               iparam: *mut c_int,
               ipntr: *mut c_int,
               workd: *mut Complex64,
               workl: *mut Complex64,
               lworkl: *mut c_int,
               rwork: *mut c_double,
               ierr: *mut c_int);

}
