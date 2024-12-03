use crate::problem::{City, Distance};

use rayon::prelude::*;

pub fn solve<D: Distance>(cities: &[City]) -> (Vec<(usize, usize)>, Vec<Vec<f32>>) {
    let mut distances = vec![vec![0f32; cities.len()]; cities.len()];
    let matrix_ptr = distances.as_mut_ptr() as u64;
    (0..cities.len()).into_par_iter().for_each(|i| {
        for j in (i + 1)..cities.len() {
            let distance = D::distance(&cities[i], &cities[j]);
            unsafe {
                let ptr = matrix_ptr as *mut Vec<f32>;
                let a = ptr.add(i);
                let b = ptr.add(j);
                (*a)[j] = distance;
                (*b)[i] = distance;
            }
        }
    });

    let mut in_spanning_tree = bit_set::BitSet::with_capacity(cities.len());
    let mut cheapest_connection = vec![f32::MAX; cities.len()];
    cheapest_connection[536] = 0.0; // City `Hel`
    let mut cheapest_connected_to: Vec<Option<usize>> = vec![None; cities.len()];
    for _ in 0..cities.len() {
        // Get argmin
        let v = cheapest_connection
            .iter()
            .enumerate()
            .filter(|x| !in_spanning_tree.contains(x.0))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap()
            .0;

        in_spanning_tree.insert(v);

        for w in 0..cities.len() {
            if !in_spanning_tree.contains(w) && distances[v][w] < cheapest_connection[w] {
                cheapest_connection[w] = distances[v][w];
                cheapest_connected_to[w] = Some(v);
            }
        }
    }

    let edges = cheapest_connected_to
        .iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|(v, w)| (v, w.unwrap()))
        .collect::<Vec<_>>();

    (edges, distances)
}
