use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

type LUT = [u8; 256];
type Histogram = [f64; 256];

pub fn histogram(data: &[u8]) -> Histogram {
    let mut histogram = [0.0; 256];
    let n = data.len() as f64;
    for &v in data {
        histogram[v as usize] += 1.0;
    }
    histogram.iter_mut().for_each(|v| {
        *v /= n;
    });
    histogram
}

pub fn build_equalization_lut(histogram: &Histogram) -> LUT {
    let mut lut = [0; 256];
    let mut acc = 0.0;
    for (idx, v) in histogram.iter().enumerate() {
        acc += v;
        lut[idx] = (255.0 * acc) as u8;
    }
    lut[255] = (255.0 * acc) as u8;
    lut
}

pub fn apply_lut_mut(data: &mut [u8], lut: &LUT) {
    data.par_iter_mut().for_each(|v| *v = lut[*v as usize]);
}

pub fn apply_lut(data: &[u8], lut: &LUT) -> Vec<u8> {
    data.par_iter().map(|&v| lut[v as usize]).collect()
}

pub fn equalize_mut(data: &mut [u8]) {
    let histogram = histogram(data);
    let lut = build_equalization_lut(&histogram);
    apply_lut_mut(data, &lut)
}

pub fn equalize(data: &[u8]) -> Vec<u8> {
    let histogram = histogram(data);
    let lut = build_equalization_lut(&histogram);
    apply_lut(data, &lut)
}
