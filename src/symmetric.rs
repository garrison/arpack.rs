// Copyright 2014 James R. Garrison
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;
extern crate num;

use self::libc::{c_char, c_double, c_float, c_int};

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
               howmny: *const c_char,
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
               howmny: *const c_char,
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

}

pub enum Which {
    LargestMagnitude,
    SmallestMagnitude,
    LargestAlgebraic,
    SmallestAlgebraic,
    BothEnds,
}

fn which_to_str(which: Which) -> &'static str {
    match which {
        LargestMagnitude => "LM",
        SmallestMagnitude => "SM",
        LargestAlgebraic => "LA",
        SmallestAlgebraic => "SA",
        BothEnds => "BE",
    }
}

// note: evals may potentially be modified even if there is an error.
fn dsaupd(n: c_int,
          nev: c_int,
          mv_multiply: |slice: &[c_double]| -> Vec<c_double>,
          evals: &mut Vec<c_double>,
          evecs: Option<&mut Vec<c_double>>) {
    assert!(nev > 0);
    assert!(nev < n);

    let mut ido: c_int = 0;
    let bmat = "I".to_c_str(); // not configurable, for now
    let which = which_to_str(SmallestAlgebraic).to_c_str();
    let tol: c_double = 0.0;
    let mut resid = Vec::from_elem(n as uint, 0.0 as c_double);
    let ncv = {
        let possible_ncv: c_int = 4 * nev;
        if possible_ncv > n {
            n
        } else {
            possible_ncv
        }
    };
    let ldv: c_int = n;
    let mut v = Vec::from_elem((ldv * ncv) as uint, 0.0 as c_double);
    let mut iparam = vec![1 as c_int, 0, 3 * n, 0, 0, 0, 1, 0, 0, 0, 0];
    // fixme: may wish to tune iparam[2] differently.
    let mut ipntr = Vec::from_elem(11, 0 as c_int);
    let mut workd = Vec::from_elem(3 * n as uint, 0.0 as c_double);
    let lworkl: c_int = ncv * (ncv + 8);
    let mut workl = Vec::from_elem(lworkl as uint, 0.0 as c_double);
    let mut info: c_int = 0;

    loop {
        unsafe {
            dsaupd_(&mut ido, bmat.as_ptr(), &n, which.as_ptr(), &nev, &tol,
                    resid.as_mut_ptr(), &ncv, v.as_mut_ptr(), &ldv,
                    iparam.as_mut_ptr(), ipntr.as_mut_ptr(), workd.as_mut_ptr(),
                    workl.as_mut_ptr(), &lworkl, &mut info);
        }

        if ido != 1 && ido != -1 {
            break;
        }

        // fixme: seems overkill
        let output = {
            let in_slice = workd.slice((ipntr[0] - 1) as uint,
                                       (ipntr[0] - 1 + n) as uint);
            mv_multiply(in_slice)
        };
        let out_slice = workd.mut_slice((ipntr[1] - 1) as uint,
                                        (ipntr[1] - 1 + n) as uint);
        out_slice.copy_from(output.as_slice());
    }

    if info < 0 {
        // fixme: return error using enum
        return;
    }

    let rvec: c_int = match evecs {
        Some(ref evecs) => 1,
        None => 0
    };
    let mut select = Vec::from_elem(ncv as uint, 0 as c_int);
    evals.clear(); // called ``d`` in arpack
    evals.grow(nev as uint, &0.0);
    let sigma: c_double = 0.0;
    let mut ierr: c_int = 0;

    let howmny = "A".to_c_str();
    unsafe {
        dseupd_(&rvec, howmny.as_ptr(), select.as_mut_ptr(), evals.as_mut_ptr(),
                v.as_mut_ptr(), &ldv, &sigma, bmat.as_ptr(),
	        &n, which.as_ptr(), &nev, &tol, resid.as_mut_ptr(), &ncv, v.as_mut_ptr(), &ldv,
	        iparam.as_mut_ptr(), ipntr.as_mut_ptr(), workd.as_mut_ptr(), workl.as_mut_ptr(), &lworkl, &mut ierr);
    }

    if ierr != 0 {
        // fixme: return error code
        return;
    }

    match evecs {
        Some(evecs) => {
            // copy the vector.  fixme: seems overkill.
            evecs.clear();
            evecs.grow((nev * n) as uint, &0.0);
            evecs.as_mut_slice().copy_from(v.as_slice());
        },
        None => ()
    };
}

#[cfg(test)]
mod tests {

    extern crate libc;
    extern crate test;

    use super::dsaupd;
    use self::test::Bencher;
    use self::libc::c_double;

    #[test]
    fn test_dsaupd() {
        let mut evals: Vec<c_double> = Vec::new();
        let mut evecs: Vec<c_double> = Vec::new();
        dsaupd(8, 2, |slice: &[c_double]| {
            // FIXME: this is probably more involved than necessary
            slice.to_vec().move_iter().map(|i| 2.0 * i).collect()
        }, &mut evals, Some(&mut evecs));
    }

    #[bench]
    fn benchmark_dsaupd(b: &mut Bencher) {
        b.iter(|| test_dsaupd());
    }

}
