use crate::problem::{Bounds, ProblemIR};
use crate::utils::{is_float, is_zero};

fn is_milp_solution(values: &[f64], integer: &[bool]) -> bool {
    values
        .iter()
        .zip(integer.iter())
        .all(|(val, int)| !int || *int && !is_float(*val))
}

fn solve_rec(problem: &ProblemIR, bounds: Bounds, lower: &mut f64, iterations: &mut u32) {
    *iterations += 1;
    println!("Try");
    // Solve relaxed problem
    let (solution, values) = match problem.with_bounds(&bounds) {
        // Unfeasible
        None => return,

        // Feasible
        Some((solution, values)) => (solution, values),
    };
    println!("Current solution: {}", solution);

    // Maximization problem
    // And relaxed problem is already worse,
    // than encountered non-relaxed one
    if solution <= *lower {
        return;
    }
    // Check whether solution is of MILP
    // if so => update the lower bound
    else if is_milp_solution(&values, &problem.is_integer) {
        *lower = solution;
        println!("updating the solution with {solution}");
        return;
    }

    let n = values.len();

    // Integer variables with real values
    let integer_but_real = (0..n).position(|i| {
        problem.is_integer[i] && is_float(values[i]) && !is_zero(bounds.lb[i] - bounds.ub[i])
    });

    println!("Integer to deal with {integer_but_real:?}");

    if let Some(i) = integer_but_real {
        let (left_bounds, right_bounds) = bounds.split(i, values[i]);
        if let Some(left_bounds) = left_bounds {
            solve_rec(problem, left_bounds, lower, iterations);
        }
        if let Some(right_bounds) = right_bounds {
            solve_rec(problem, right_bounds, lower, iterations);
        }
    }
}

pub fn solve(problem: &ProblemIR, bounds: Bounds) -> (f64, u32) {
    // Initialize stack
    let mut iterations = 0u32;
    let mut lower = 0.0f64;

    // Solve recursively
    solve_rec(problem, bounds, &mut lower, &mut iterations);
    (lower, iterations)
}
