use serde::{Deserialize, Serialize};
use crate::core::mat::Mat;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Request {
    pub A: Vec<Vec<f32>>,
    pub b: Vec<f32>,
    pub eps: f32,
    pub n: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Response {
    pub x: Vec<f32>,
    pub acc: Vec<f32>,
    pub eps: f32,
    pub iters: u32,
    pub error: String,
}