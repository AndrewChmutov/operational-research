use crate::problem::{Bounds, ProblemIR};
use crate::solver;

fn lecture_sample() -> (ProblemIR, Bounds) {
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

    let problem = ProblemIR {
        coefficients,
        objective_coefficients,
        resources,
        is_integer,
    };
    let bounds = Bounds { lb, ub };

    (problem, bounds)
}

fn lab_sample(relaxed: bool) -> (ProblemIR, Bounds) {
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

    let problem = ProblemIR {
        coefficients,
        objective_coefficients,
        resources,
        is_integer,
    };
    let bounds = Bounds { lb, ub };

    (problem, bounds)
}

#[cfg(test)]
mod check {
    use super::*;

    #[test]
    fn solve_lecture() {
        let (problem, bounds) = lecture_sample();
        let (solution, iterations) = solver::solve(&problem, bounds);
        println!("Answer: {}", solution);
        println!("Iterations: {}", iterations);
        assert!((solution - 22.5).abs() <= f64::EPSILON);
    }

    #[test]
    fn solve_lab() {
        let (problem, bounds) = lab_sample(false);
        let (solution, iterations) = solver::solve(&problem, bounds);
        println!("Answer: {}", solution);
        println!("Iterations: {}", iterations);
        assert!((solution - 207.0).abs() <= f64::EPSILON);
    }
}
