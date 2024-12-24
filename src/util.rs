pub(crate) fn cosine_similarity(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f64 { // Dot product divided by magnitude
    assert_eq!(vec1.len(), vec2.len(), "Vectors must be of the same length");

    let dot_product = vec1.iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| f64::from(a) * f64::from(b))
        .sum::<f64>();

    let magnitude1 = vec1.iter().map(|&a| f64::from(a) * f64::from(a)).sum::<f64>().sqrt();
    let magnitude2 = vec2.iter().map(|&b| f64::from(b) * f64::from(b)).sum::<f64>().sqrt();

    dot_product / (magnitude1 * magnitude2)
}