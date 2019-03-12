#![cfg_attr(rustfmt, rustfmt_skip)]

use na::{DMatrix, Matrix3, Matrix4};

#[test]
fn schur_simpl_mat3() {
    let m = Matrix3::new(-2.0, -4.0, 2.0,
                         -2.0,  1.0, 2.0,
                          4.0,  2.0, 5.0);

    let schur = m.real_schur();
    let (vecs, vals) = schur.unpack();

    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7));
}

#[cfg(feature = "arbitrary")]
mod quickcheck_tests {
    use std::cmp;
    use na::{DMatrix, Matrix2, Matrix3, Matrix4};
    use core::helper::{RandScalar, RandComplex};

    quickcheck! {
        fn schur(n: usize) -> bool {
            let n = cmp::max(1, cmp::min(n, 10));
            let m = DMatrix::<RandComplex<f64>>::new_random(n, n).map(|e| e.0);

            let (vecs, vals) = m.clone().real_schur().unpack();

            if !relative_eq!(&vecs * &vals * vecs.conjugate_transpose(), m, epsilon = 1.0e-7) {
                println!("{:.5}{:.5}", m, &vecs * &vals * vecs.conjugate_transpose());
            }

            relative_eq!(&vecs * vals * vecs.conjugate_transpose(), m, epsilon = 1.0e-7)
        }

        fn schur_static_mat2(m: Matrix2<RandComplex<f64>>) -> bool {
            let m = m.map(|e| e.0);
            let (vecs, vals) = m.clone().real_schur().unpack();

            let ok = relative_eq!(vecs * vals * vecs.conjugate_transpose(), m, epsilon = 1.0e-7);
            if !ok {
                println!("Vecs: {:.5} Vals: {:.5}", vecs, vals);
                println!("Reconstruction:{}{}", m, &vecs * &vals * vecs.conjugate_transpose());
            }
            ok
        }

        fn schur_static_mat3(m: Matrix3<RandComplex<f64>>) -> bool {
            let m = m.map(|e| e.0);
            let (vecs, vals) = m.clone().real_schur().unpack();

            let ok = relative_eq!(vecs * vals * vecs.conjugate_transpose(), m, epsilon = 1.0e-7);
            if !ok {
                println!("Vecs: {:.5} Vals: {:.5}", vecs, vals);
                println!("{:.5}{:.5}", m, &vecs * &vals * vecs.conjugate_transpose());
            }
            ok
        }

        fn schur_static_mat4(m: Matrix4<RandComplex<f64>>) -> bool {
            let m = m.map(|e| e.0);
            let (vecs, vals) = m.clone().real_schur().unpack();

            let ok = relative_eq!(vecs * vals * vecs.conjugate_transpose(), m, epsilon = 1.0e-7);
            if !ok {
                println!("{:.5}{:.5}", m, &vecs * &vals * vecs.conjugate_transpose());
            }
            ok
        }
    }
}

#[test]
fn schur_static_mat4_fail() {
    let m = Matrix4::new(
         33.32699857679677,  46.794945978960044, -20.792148817005838,   84.73945485997737,
        -53.04896234480401,  -4.031523330630989,  19.022858300892366,   -93.2258351951158,
        -94.61793793643038,  -18.64216213611094,   88.32376703241675,  -99.30169870309795,
         90.62661897246733,   96.74200696130146,    34.7421322611369,   84.86773307198098);

    let (vecs, vals) = m.clone().real_schur().unpack();
    println!("{:.6}{:.6}", m, &vecs * &vals * vecs.transpose());
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

#[test]
fn schur_static_mat4_fail2() {
    let m = Matrix4::new(
        14.623586538485966, 7.646156622760756, -52.11923331576265, -97.50030223503413,
        53.829398131426785, -33.40560799661168, 70.31168286972388, -81.25248138434173,
        27.932377940728202, 82.94220150938, -35.5898884705951, 67.56447552434219,
        55.66754906908682, -42.14328890569226, -20.684709585152206, -87.9456949841046);

    let (vecs, vals) = m.clone().real_schur().unpack();
    println!("{:.6}{:.6}", m, &vecs * &vals * vecs.transpose());
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

#[test]
fn schur_static_mat3_fail() {
    let m = Matrix3::new(
        -21.58457553143394,   -67.3881542667948, -14.619829849784338,
        -7.525423104386547, -17.827350599642287,  11.297377444555849,
        38.080736654870464,  -84.27428302131528,  -95.88198590331922);

    let (vecs, vals) = m.clone().real_schur().unpack();
    println!("{:.6}{:.6}", m, &vecs * &vals * vecs.transpose());
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

// Test proposed on the issue #176 of rulinalg.
#[test]
fn schur_singular() {
    let m = DMatrix::from_row_slice(24, 24, &[
        1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,
        0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0]);

    let (vecs, vals) = m.clone().real_schur().unpack();
    println!("{:.6}{:.6}", m, &vecs * &vals * vecs.transpose());
    assert!(relative_eq!(&vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}
