mod ops;

#[cfg(test)]
mod test;

use crate::traits::Num;
use crate::utils::assert_len;

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T>(Vec<T>);

pub fn norm<T: Num<T>>(v: &Vector<T>) -> T {
    let mut p = T::zero();
    for e in v.0.iter() {
        p = p + *e * *e;
    }

    p.sqrt()
}

pub fn outer<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> Vector<Vector<T>> {
    let mut rows = Vector(Vec::with_capacity(v.0.len()));

    for ve in v.0.iter() {
        let mut row = Vec::with_capacity(w.0.len());
        for we in w.0.iter() {
            row.push(*ve * *we);
        }
        rows.0.push(Vector(row))
    }

    rows
}

pub fn dot<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> T {
    assert_len(v.0.len(), w.0.len());

    let mut prod = T::zero();
    let dim = v.0.len();
    for i in 0..dim {
        prod = prod + v.0[i] * w.0[i];
    }

    prod
}

pub fn cross<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> Vector<T> {
    assert_len(3, v.0.len());

    let x = v.0[1] * w.0[2] - v.0[2] * w.0[1];
    let y = v.0[2] * w.0[0] - v.0[0] * w.0[2];
    let z = v.0[0] * w.0[1] - v.0[1] * w.0[0];

    Vector(vec![x, y, z])
}

pub fn unit_vector<T: Num<T>>(v: &Vector<T>) -> Vector<T> {
    v / norm(v)
}