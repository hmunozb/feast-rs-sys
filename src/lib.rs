#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod bindings;
pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::*;

    #[test]
    fn test_feast() {
        // fortran calling convention is terrible
        // everything is a mut ptr
        let mut uplo: c_char = 'F' as c_char;
        let mut n : c_int = 4;
        let mut mat = [
            2.0, -1.0, -1.0, 0.0,
            -1.0, 3.0, -1.0, -1.0,
            -1.0, -1.0, 3.0, -1.0,
            0.0, -1.0, -1.0, 2.0
        ];
        let mut fpm : [c_int; 64] = [0; 64]; //feast params
        let mut m0: c_int = 3; // search subspace dim

        let mut eigvals : Vec<c_double> = Vec::with_capacity(m0 as usize);
        eigvals.resize_with(m0 as usize, Default::default);
        let mut res : Vec<c_double> = Vec::with_capacity(m0 as usize);
        res.resize_with(m0 as usize, Default::default);
        let mut eigvecs : Vec<c_double> = Vec::with_capacity((n*m0) as usize);
        eigvecs.resize_with((n*m0) as usize, Default::default);

        println!("Initializing FEAST...");

        unsafe{ feastinit_(fpm.as_mut_ptr()) }

        fpm[0] = 1; // Enable printing

        let mut epsout : f64 = 0.0;
        let mut nloops : c_int = 0;
        let mut info : c_int = 0;
        let mut e_min: f64 = 3.0;
        let mut e_max: f64 = 5.0;
        let mut mode :c_int = 0;
        unsafe{
            dfeast_syev_(&uplo , & n as *const c_int,
                         mat.as_ptr(), &n, fpm.as_mut_ptr(),
                        &mut epsout, &mut nloops, &mut e_min, &mut e_max, &mut m0,
            eigvals.as_mut_ptr(), eigvecs.as_mut_ptr(), &mut mode, res.as_mut_ptr(),
                        &mut info ) ;
        }

    }
}
