use std::fmt::Debug;

pub trait DistanceMetric: Debug + Send + Sync {
    fn distance(a: &[f32], b: &[f32]) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct CosineDistance;

impl DistanceMetric for CosineDistance {
    fn distance(a: &[f32], b: &[f32]) -> f32 {
        let (dot, norm_a, norm_b) = a
            .iter()
            .zip(b.iter())
            .fold((0.0f32, 0.0f32, 0.0f32), |(d, na, nb), (&x, &y)| {
                (d + x * y, na + x * x, nb + y * y)
            });
        let denom = (norm_a * norm_b).sqrt();
        if denom == 0.0 {
            0.0
        } else {
            1.0 - (dot / denom)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EuclideanDistance;

impl DistanceMetric for EuclideanDistance {
    fn distance(a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x - y) * (x - y))
            .sum::<f32>()
            .sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DotProduct;

impl DistanceMetric for DotProduct {
    fn distance(a: &[f32], b: &[f32]) -> f32 {
        -a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_identical_vectors_distance_zero() {
        let v = vec![1.0, 2.0, 3.0];
        let d = CosineDistance::distance(&v, &v);
        assert!((d - 0.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_vectors_distance_one() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let d = CosineDistance::distance(&a, &b);
        assert!((d - 1.0).abs() < 1e-6);
    }

    #[test]
    fn euclidean_same_point_zero() {
        let v = vec![1.0, 2.0, 3.0];
        let d = EuclideanDistance::distance(&v, &v);
        assert!((d - 0.0).abs() < 1e-6);
    }

    #[test]
    fn euclidean_3d_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![3.0, 4.0, 0.0];
        let d = EuclideanDistance::distance(&a, &b);
        assert!((d - 5.0).abs() < 1e-6);
    }

    #[test]
    fn dot_product_negated() {
        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];
        let d = DotProduct::distance(&a, &b);
        assert!((d + 11.0).abs() < 1e-6);
    }
}
