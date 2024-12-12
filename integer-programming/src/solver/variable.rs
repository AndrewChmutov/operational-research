use crate::problem::{Bounds, ProblemIR};
use crate::utils::{is_float, is_zero};

pub trait VariableSearch {
    fn pick(problem: &ProblemIR, values: &[f64], bounds: &Bounds) -> Option<usize>;
}

pub struct NoSearch;
impl VariableSearch for NoSearch {
    fn pick(problem: &ProblemIR, values: &[f64], bounds: &Bounds) -> Option<usize> {
        let integer_but_real = values
            .iter()
            .enumerate()
            .find(|(i, _)| {
                problem.is_integer[*i]
                    && is_float(values[*i])
                    && !is_zero(bounds.lb[*i] - bounds.ub[*i])
            })
            .map(|x| x.0);

        integer_but_real
    }
}

pub struct ByValue;
impl VariableSearch for ByValue {
    fn pick(problem: &ProblemIR, values: &[f64], bounds: &Bounds) -> Option<usize> {
        let integer_but_real = values
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                problem.is_integer[*i]
                    && is_float(values[*i])
                    && !is_zero(bounds.lb[*i] - bounds.ub[*i])
            })
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|x| x.0);

        integer_but_real
    }
}

pub struct ByConstraints;
impl VariableSearch for ByConstraints {
    fn pick(problem: &ProblemIR, values: &[f64], bounds: &Bounds) -> Option<usize> {
        let integer_but_real = problem
            .constraints_per_variable
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                problem.is_integer[*i]
                    && is_float(values[*i])
                    && !is_zero(bounds.lb[*i] - bounds.ub[*i])
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|x| x.0);

        integer_but_real
    }
}

pub struct ByLength;
impl VariableSearch for ByLength {
    fn pick(problem: &ProblemIR, values: &[f64], bounds: &Bounds) -> Option<usize> {
        let integer_but_real = bounds
            .lengths()
            .enumerate()
            .filter(|(i, _)| {
                problem.is_integer[*i]
                    && is_float(values[*i])
                    && !is_zero(bounds.lb[*i] - bounds.ub[*i])
            })
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|x| x.0);

        integer_but_real
    }
}
