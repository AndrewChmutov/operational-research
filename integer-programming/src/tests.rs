use std::collections::HashMap;

use crate::problem::{Bounds, ProblemIR};
use crate::solver::node::{ByInterval, NoSort, NodeSearch, DFS};
use crate::solver::variable::{ByConstraints, ByLength, ByValue, NoSearch, VariableSearch};

struct Sample {
    problem: ProblemIR,
    bounds: Bounds,
    optimum: Option<f64>,
    max_solver_calls: Option<u32>,
}

fn lecture_sample() -> Sample {
    let coefficients: Vec<Vec<f64>> = vec![
        vec![4.0, 3.0, 4.0, 2.0],
        vec![0.0, 0.0, 1.0, 1.5],
        vec![1.25, 0.0, 1.0, 0.0],
    ];
    let (resources, objective_coefficients, lb, ub): (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) = (
        vec![15.0, 1.0, 5.0],
        vec![5.0, 4.0, 6.0, 8.0],
        vec![0.0; 4],
        vec![4.0, 4.0, 1.0, 1.0],
    );

    let is_integer = vec![true, true, false, false];

    let problem = ProblemIR::new(coefficients, objective_coefficients, resources, is_integer);
    let bounds = Bounds { lb, ub };

    Sample {
        problem,
        bounds,
        optimum: Some(22.5),
        max_solver_calls: None,
    }
}

fn lab_sample(relaxed: bool, check: bool) -> Sample {
    let coefficients: Vec<Vec<f64>> = vec![
        vec![0.0, 3.0, 2.0, 0.0, 0.0, 0.0, -3.0, -1.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.0, 2.0, 0.0, 0.0, 0.0, -1.0, 2.0, 1.0],
        vec![0.0, 0.0, 2.0, -2.0, 3.0, 0.0, -2.0, 2.0, 1.0, 0.0],
        vec![0.0, 0.0, 2.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0],
        vec![0.0, 2.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 1.0],
        vec![1.0, 4.0, 0.0, 0.0, 0.0, 0.0, -3.0, 6.0, 2.0, 0.0],
        vec![2.0, 2.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 2.0, 2.0],
        vec![0.0, 0.0, 3.0, 0.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0],
        vec![0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 1.0, 1.0, 0.0, 3.0],
        vec![2.0, -7.0, 0.0, 0.0, 0.0, 1.0, 0.0, 8.0, 2.0, 0.0],
    ];
    let (resources, objective_coefficients, lb, ub): (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) = (
        vec![10.0, 15.0, 20.0, 20.0, 30.0, 50.0, 40.0, 20.0, 25.0, 25.0],
        vec![5.0, 7.0, 5.0, 5.0, 5.0, 5.0, 7.0, 4.0, 9.0, 10.0],
        vec![0.0; 10],
        vec![5.0, 8.0, 4.0, 5.0, 4.0, 5.0, 5.0, 3.0, 3.0, 3.0],
    );

    let is_integer =
        // First five - integers if the problem is not relaxed
        vec![!relaxed; 5]
        .into_iter()

        // Add five real-valued
        .chain(vec![false; 5].into_iter())
        .collect::<Vec<_>>();

    let problem = ProblemIR::new(coefficients, objective_coefficients, resources, is_integer);
    let bounds = Bounds { lb, ub };
    let optimum = if check { Some(207.0) } else { None };
    let max_solver_calls = if check { Some(20) } else { None };

    Sample {
        problem,
        bounds,
        optimum,
        max_solver_calls,
    }
}

fn solve<N, V>(sample: Sample) -> (f64, u32)
where
    N: NodeSearch,
    V: VariableSearch,
{
    let (solution, iterations) = N::solve::<V>(&sample.problem, sample.bounds);
    println!("Answer: {}", solution);
    println!("Solver calls: {}", iterations);
    if let Some(optimum) = sample.optimum {
        assert!((solution - optimum).abs() <= f64::EPSILON);
    }
    if let Some(max_solver_calls) = sample.max_solver_calls {
        assert!(iterations < max_solver_calls, "Optimization is not enough");
    }
    (solution, iterations)
}

fn solve_lecture_classic() -> (f64, u32) {
    let sample = lecture_sample();
    solve::<DFS<NoSort>, ByConstraints>(sample)
}

fn solve_lab_parametrized<N: NodeSearch, V: VariableSearch>(check: bool) -> (f64, u32) {
    let sample = lab_sample(false, check);
    solve::<N, V>(sample)
}

fn solve_lab_default() -> (f64, u32) {
    solve_lab_parametrized::<DFS<NoSort>, ByLength>(false)
}

macro_rules! run_combinations_impl {
    // No first array left => exit
    ($algo:ident $out:tt [] $b:tt $init_b:tt) => {
        $out
    };

    // First array is not drained => copy the second array and continue matching
    (
        $algo:ident
        $out:tt [$a:ty, $($at:tt)*]
        []
        $init_b:tt
    ) => {
        run_combinations_impl!($algo $out [$($at)*] $init_b $init_b)
    };

    (
        $algo:ident
        [$($out:tt)*]
        [$a:ty, $($at:tt)*]
        [$b:ty, $($bt:tt)*]
        $init_b:tt
    ) => {
        run_combinations_impl!(
            $algo
            [$($out)* ((stringify!($a), stringify!($b)), $algo::<$a, $b>(false)),]
            [$a, $($at)*]
            [$($bt)*]
            $init_b
        )
    };
}

macro_rules! run_combinations {
    ($algo:ident, [$($a:tt)*], [$($b:tt)*]) => {
        run_combinations_impl!($algo [] [$($a)*,] [$($b)*,] [$($b)*,])
    };
}

fn matrix() {
    // Compute
    let cross = run_combinations!(
        solve_lab_parametrized,
        [DFS<NoSort>, DFS<ByInterval>],
        [NoSearch, ByConstraints, ByLength, ByValue]
    );

    // To HashMap
    let result = cross
        .into_iter()
        .fold(HashMap::new(), |mut acc, ((k1, k2), v)| {
            acc.entry(k1).or_insert_with(HashMap::new).insert(k2, v);
            acc
        });

    // Print
    print!(" {:<16} |", "");
    for node_search in result.keys() {
        print!(" {:<16} |", node_search)
    }
    println!();

    let var_search = result.values().next().unwrap().keys().collect::<Vec<_>>();
    for var_s in var_search {
        print!(" {:<16} |", var_s);
        for node_search in result.keys() {
            let (val, calls) = result[node_search][var_s];
            print!(" {:<16} |", format!("{val} / {calls}"))
        }
        println!();
    }
}

#[cfg(test)]
mod check {

    use super::*;

    #[test]
    fn solve_lecture() {
        solve_lecture_classic();
    }

    #[test]
    fn solve_lab() {
        solve_lab_default();
    }

    #[test]
    fn solve_matrix() {
        matrix();
    }
}

#[cfg(test)]
mod perf {
    use super::*;

    #[test]
    fn solve_lecture() {
        solve_lecture_classic();
    }

    #[test]
    fn solve_lab() {
        solve_lab_default();
    }

    #[test]
    fn solve_matrix() {
        matrix();
    }
}
