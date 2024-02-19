use std::ops::{Add, Index, IndexMut, Mul};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct Vect {
    pub data: Vec<f32>,
}

impl Vect {
    pub fn new(data: Vec<f32>) -> Vect {
        Vect { data }
    }
}


impl Add for Vect {
    type Output = Vect;

    fn add(self, other: Vect) -> Vect {
        let data = self.data
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Vect { data }
    }
}

impl Mul<f32> for Vect {
    type Output = Vect;

    fn mul(self, other: f32) -> Vect {
        let data = self.data
            .into_iter()
            .map(|a| a * other)
            .collect();

        Vect { data }
    }
}

impl Mul for Vect {
    type Output = f32;

    fn mul(self, other: Vect) -> f32 {
        self.data
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl Index<usize> for Vect {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vect {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Serialize for Vect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.data.serialize(serializer)
    }
}

impl<'d> Deserialize<'d> for Vect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'d> {
        let data = Vec::<f32>::deserialize(deserializer)?;
        Ok(Vect::new(data))
    }
}