#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f32>>,
}

impl Mat {
    pub fn new(init: f32, rows: usize, cols: usize) -> Mat {
        Mat { rows, cols, data: vec![vec![init; cols]; rows] }
    }

    pub fn from(data: Vec<Vec<f32>>) -> Mat {
        match data.len() {
            0 => Mat { rows: 0, cols: 0, data: vec![] },
            _ => Mat { rows: data.len(), cols: data[0].len(), data }
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn det(&self, eps: f32) -> f32 {
        assert_eq!(self.cols, self.rows);

        let mut a = self.clone();
        let mut ans = 1f32;

        for i in 0..self.rows {
            let mut k = i;

            for j in i+1..self.rows {
                if a[j][i].abs() > a[k][i].abs() {
                    k = j;
                }
            }
            if a[k][i].abs() < eps {
                return 0f32;
            }
            let tmp = a[k].clone();
            a[k] = a[i].clone();
            a[i] = tmp;

            if i != k {
                ans = -ans;
            }

            ans *= a[i][i];

            for j in i+1..self.rows {
                a[i][j] /= a[i][i];
            }
            for j in 0..self.rows {
                if j != i && a[j][i].abs() > eps {
                    for k in i+1..self.rows {
                        a[j][k] -= a[j][i] * a[i][k];
                    }
                }
            }

        }

        ans
    }

    pub fn rk(&self) -> usize {
        let mut ans = 0;
        let mut a = self.clone();

        for j in 0..self.cols {
            let mut pivot = false;

            for i in ans..self.rows {
                if a[i][j] != 0.0 {
                    let tmp = a[i].clone();
                    a[i] = a[ans].clone();
                    a[ans] = tmp;
                    pivot = true;
                    break;
                }
            }

            if !pivot {
                continue;
            }
            ans += 1;

            for k in j..self.cols {
                a[ans-1][k] /= a[ans-1][j];
            }

            for r in ans..self.rows {
                for k in (j..self.cols).rev() {
                    a[r][k] -= a[ans-1][k]*a[r][j];
                }
            }
        }

        ans
    }
}

impl std::ops::Index<usize> for Mat {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Vec<f32> {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Mat {
    fn index_mut(&mut self, index: usize) -> &mut Vec<f32> {
        &mut self.data[index]
    }
}

use std::mem::swap;
use std::ops::{Add, Mul};

impl Add for Mat {
    type Output = Mat;

    fn add(self, other: Mat) -> Mat {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrix sizes do not match");
        }

        let data = self.data
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a.into_iter().zip(b.into_iter()).map(|(x, y)| x + y).collect())
            .collect();

        Mat { rows: self.rows, cols: self.cols, data }
    }
}

impl Mul<f32> for Mat {
    type Output = Mat;

    fn mul(self, other: f32) -> Mat {
        let data = self.data
            .into_iter()
            .map(|row| row.into_iter().map(|elem| elem * other).collect())
            .collect();

        Mat { rows: self.rows, cols: self.cols, data }
    }
}

impl Mul for Mat {
    type Output = Mat;

    fn mul(self, other: Mat) -> Mat {
        if self.cols != other.rows {
            panic!("Matrix sizes do not match");
        }

        let mut ans = Mat::from(vec![vec![0.0; other.cols]; self.rows]);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    ans[i][j] += self[i][k] * other[k][j];
                }
            }
        }

        ans
    }
}