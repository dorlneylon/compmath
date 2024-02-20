use itertools::Itertools;
use crate::core::mat::Mat;
use crate::core::models::Response;

const M: u32 = 100;

fn accuracy(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    let mut ans = vec![0f32; x.len()];

    for i in 0..x.len() {
        ans[i] = (x[i] - y[i]).abs();
    }

    ans
}

fn converges(x: &Vec<f32>, y: &Vec<f32>, eps: f32) -> bool {
    accuracy(x, y).into_iter().all(|a| a < eps)
}

fn dominant(a: &mut Mat) -> bool {
    for i in 0..a.cols {
        let mut sum = 0f32;

        for j in 0..a.cols {
            if i != j {
                sum += a[i][j];
            }
        }

        if sum > a[i][i] {
            return false;
        }
    }

    true
}

fn permute(a: &mut Mat) -> Result<(), String> {
    let mut id = vec![0usize; a.rows];
    (0..a.rows as usize).for_each(|i| id[i] = i);

    for s in id.into_iter().permutations(a.rows) {
        for i in 0..a.rows {
            for j in 0..(a.cols+1)/2 {
                let swap = a[i][s[j]];
                a[i][s[j]] = a[i][j];
                a[i][j] = swap;
            }
        }

        if dominant(a) {
            return Ok(());
        }
    }

    Err("Matrix cannot be diagonally dominant".to_string())
}

fn conditions(a: &mut Mat, b: &Vec<f32>, eps: f32) -> Result<(), String> {
    let mut vc = a.data.clone();
    vc.push(b.clone());
    let ab = Mat::from(vc);

    if a.rk() != ab.rk() {
        return Err("Kronecker-Capelli's condition is not satisfied".to_string());
    }

    if a.det(eps) == 0f32 {
        return Err("Matrix is singular".to_string());
    }

    if let Err(msg) = permute(a) {
        return Err(msg);
    }

    Ok(())
}

pub fn gauss_seidel(a: &mut Mat, b: &Vec<f32>, eps: f32) -> Result<Response, String> {
    match conditions(a, b, eps) {
        Ok(_) => process(a, b, eps),
        Err(msg) => Err(msg),
    }
}

fn process(a: &mut Mat, b: &Vec<f32>, eps: f32) -> Result<Response, String> {
    let mut x = vec![0f32; a.rows];
    let mut y = vec![1f32; a.rows];

    for k in 1..=M {
        x = y.clone();

        for i in 0..a.rows {
            let mut var = 0f32;
            for j in 0..a.cols {
                if i != j {
                    var += a[i][j] * x[j];
                }
            }
            y[i] = (b[i] - var) / a[i][i];
        }

        if converges(&x, &y, eps) {
            return Ok(Response { x: x.clone(), acc: accuracy(&x, &y), eps, iters: k, error: "".to_string() });
        }
    }

    Err(format!("Failed to converge in {} iterations", M))
}