use std::any::type_name;
use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::consts::{DATA_PATH, PY_INTERPRETER_PATH};
use crate::problem::{Archaversine, City, Distance, Euclidean};
use crate::solver::solve;

use serde::Serialize;

fn mst<D: Distance>() -> ((Vec<(usize, usize)>, Vec<Vec<f32>>), Vec<City>) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(DATA_PATH)
        .expect(&format!("Could not open {DATA_PATH}"));

    let cities = rdr
        .deserialize::<City>()
        .map(|x| x.expect("Could not deserialize record"))
        .map(City::calculate_coordinates)
        .collect::<Vec<_>>();
    // dbg!(&cities);

    (solve::<D>(&cities), cities)
}

fn check_mst<D: Distance>() {
    let ((edges, distances), cities) = mst::<D>();

    assert_eq!(
        edges
            .iter()
            .flat_map(|x| [x.0, x.1])
            .collect::<HashSet<_>>(),
        (0..cities.len()).collect::<HashSet<_>>(),
        "Spanning tree does not cover the whole graph"
    );

    println!("Number of cities: {}", cities.len());
    println!("Number of edges in MST: {}", edges.len());
    println!(
        "Total weight sum: {}",
        edges.iter().map(|x| distances[x.0][x.1]).sum::<f32>()
    );
}

#[derive(Serialize)]
struct SerializableEdge {
    x: (u32, u32),
    y: (u32, u32),
}

impl SerializableEdge {
    fn from<D: Distance>(value: (usize, usize), cities: &[City]) -> Self {
        Self {
            x: (cities[value.0].x, cities[value.1].x),
            y: (cities[value.0].y, cities[value.1].y),
        }
    }
}

#[derive(Serialize)]
struct SerializableCities {
    x: Vec<u32>,
    y: Vec<u32>,
}

impl From<Vec<City>> for SerializableCities {
    fn from(value: Vec<City>) -> Self {
        Self {
            x: value.iter().map(|x| x.x).collect(),
            y: value.iter().map(|x| x.y).collect(),
        }
    }
}

#[derive(Serialize)]
struct PlotData {
    target: String,
    edges: Vec<SerializableEdge>,
    cities: SerializableCities,
}

fn get_type_name<T>() -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap_or(full_name)
}

fn plot_mst<D: Distance>() {
    let ((edges, _), cities) = mst::<D>();

    let mut child = Command::new(PY_INTERPRETER_PATH)
        .stdin(Stdio::piped())
        .args(["scripts/plot.py"])
        .spawn()
        .expect("Failed to start a subprocess");

    child
        .stdin
        .as_mut()
        .expect("Failed to pass data to the plotting script")
        .write_all(
            serde_json::to_string(&PlotData {
                target: format!("{}.png", get_type_name::<D>()),
                edges: edges
                    .into_iter()
                    .map(|x| SerializableEdge::from::<D>(x, &cities))
                    .collect(),
                cities: cities.into(),
            })
            .expect("Failed to serialize plot data")
            .as_bytes(),
        )
        .expect("Failed to write to subprocess stdin");

    child.wait_with_output().expect("Failed to read stdout");
}

#[cfg(test)]
mod check {
    use super::*;

    #[test]
    fn mst_euclidean() {
        check_mst::<Euclidean>();
    }

    #[test]
    fn mst_archaversine() {
        check_mst::<Archaversine>();
    }
}

#[cfg(test)]
mod perf {
    use super::*;

    #[test]
    fn mst_euclidean() {
        mst::<Euclidean>();
    }

    #[test]
    fn mst_archaversine() {
        mst::<Archaversine>();
    }
}

#[cfg(test)]
mod artifact {

    use super::*;

    #[test]
    fn mst_plot_euclidean() {
        plot_mst::<Euclidean>();
    }

    #[test]
    fn mst_plot_archaversine() {
        plot_mst::<Archaversine>();
    }
}
