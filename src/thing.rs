use std::usize;

use rand::prelude::*;

pub struct Thing<const D: usize> {
    data: Vec<u8>,
}

// Housekeeping
impl<const D: usize> Thing<D> {
    // dimensions are number of bits for a repr
    pub fn from_rng(generator: &mut ThreadRng) -> Self {
        Self {
            data: (0..(D / 8)).map(|_| generator.gen()).collect(),
        }
    }

    pub fn dimensions(&self) -> usize {
        self.data.len() * 8
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

pub fn bind<const D: usize>(a: &Thing<D>, b: &Thing<D>) -> Thing<D> {
    let data = a
        .data()
        .iter()
        .zip(b.data().iter())
        .map(|(a, b)| a ^ b)
        .collect();

    Thing { data }
}

fn bundle_once<const D: usize>(a: &Thing<D>, b: &Thing<D>) -> Thing<D> {
    let data = a
        .data()
        .iter()
        .zip(b.data().iter())
        .map(|(a, b)| a | b)
        .collect();

    Thing { data }
}

pub fn bundle<const D: usize>(a: Vec<&Thing<D>>) -> Option<Thing<D>> {
    let mut init: Option<Thing<D>> = None;

    for i in a[1..].iter() {
        if let Some(ino) = init {
            init = Some(bundle_once(&ino, i));
        } else {
            init = Some(bundle_once(a[0], i));
        }
    }
    init
}

pub fn cos_sim<const D: usize>(a: &Thing<D>, b: &Thing<D>) -> f64 {
    let mut dot_sum = 0.0;
    let mut a_mag_sum: f64 = 0.0;
    let mut b_mag_sum = 0.0;

    for i in 0..D {
        let element_index = i / 8;
        let inner_index = i % 8;

        let a_elem = a.data()[element_index];
        let b_elem = b.data()[element_index];

        let inner_a_elem = if a_elem & (1 << inner_index) == 0 {
            0.0
        } else {
            1.0
        };
        let inner_b_elem = if b_elem & (1 << inner_index) == 0 {
            0.0
        } else {
            1.0
        };

        dot_sum += inner_a_elem * inner_b_elem;
        a_mag_sum += inner_a_elem;
        b_mag_sum += inner_b_elem;
    }

    dot_sum / (a_mag_sum.sqrt() * b_mag_sum.sqrt())
}

pub fn sim<const D: usize>(a: &Thing<D>, b: &Thing<D>, noise: &Thing<D>) {
    let cs = cos_sim(a, b);

    let csn = cos_sim(a, noise);

    println!("AvB: {}, Avnoise: {}", cs, csn);
}
