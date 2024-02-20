
#[cfg(test)]
mod tests {
    use crate::core::eq_solver::{gauss_seidel};
    use crate::core::mat::Mat;
    use crate::core::models::Response;

    #[test]
    fn helper() {
        let mat = Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let serialized = serde_json::to_string(&mat).unwrap();
        println!("serialized: {}", serialized);
        let deserialized: Mat = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mat, deserialized);
    }

    #[test]
    fn test_mat_ops() {
        let mut mat = Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        mat = mat * 2.0;
        assert_eq!(mat, Mat::from(vec![vec![2.0, 4.0], vec![6.0, 8.0]]));

        let mat2 = Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(mat * mat2 * 0.5f32, Mat::from(vec![vec![7.0, 10.0], vec![15.0, 22.0]]));

        let mat1 = Mat::from(vec![vec![1.0, 2.0, 3.0]]);
        let mat2 = Mat::from(vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]]);
        assert_eq!(mat1 * mat2, Mat::from(vec![vec![1.0, 2.0, 3.0]]));

        let mat1 = Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let mat2 = Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(mat1 + mat2, Mat::from(vec![vec![2.0, 4.0], vec![6.0, 8.0]]));

        assert_eq!(Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]])[0][1], 2.0);

        assert!((Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).det(1e-2)+2.0).abs() < 1e-2);
        assert_eq!(Mat::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).rk(), 2);
        assert_eq!(Mat::from(vec![vec![1.0,1.0,0.0],vec![1.0,1.0,0.0], vec![0.0,0.0,1.0]]).rk(), 2);
    }

    #[test]
    fn test_solver() {
        let mut A = Mat::from(vec![vec![2.0, 2.0, 10.0], vec![10.0, 1.0, 1.0], vec![2.0, 10.0, 1.0]]);
        let b = vec![14.0, 12.0, 13.0];

        match gauss_seidel(&mut A, &b, 1e-4) {
            Ok(Response { x, acc, eps, iters, error }) => {
                assert_eq!(x, vec![1.0, 1.0, 1.0]);
                assert_eq!(acc, vec![0.0, 0.0, 0.0]);
                assert_eq!(eps, 1e-4);
                assert_eq!(iters, 1);
            }
            Err(msg) => panic!("{}", msg)
        }
    }
}