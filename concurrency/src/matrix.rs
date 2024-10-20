use anyhow::{anyhow, Ok, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

#[derive(Debug)]
pub struct Matrix<T: fmt::Debug> {
    pub col: usize,
    pub row: usize,
    pub data: Vec<T>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Add<Output = T> + Copy + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }
    let mut data = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        col: b.col,
        row: a.row,
        data,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            col,
            row,
            data: data.into(),
        }
    }
}
