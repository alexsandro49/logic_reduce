//! Data-only counterpart of Logisim's OutputExpressions.computeColumn,
//! TruthTable.isInputSet and Implicant.computePartition. No Swing dependencies.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::models::{
    expression::Expr,
    notation::Notation,
    parser::Parser,
    truth_table::{
        DisplayMode, MAX_INPUTS, MAX_REDUCED_INPUTS, MAX_TABLE_CELLS, TruthTable, TruthTableRow,
    },
};
use crate::services::boolean_algebra;

pub fn generate(
    source: &str,
    notation: Notation,
    display: DisplayMode,
    include_intermediate: bool,
) -> Result<TruthTable, String> {
    let expression =
        Parser::parse(source).map_err(|error| format!("Expressão inválida: {error}"))?;
    let mut names = BTreeSet::new();
    collect_inputs(&expression, &mut names);
    let inputs: Vec<String> = names.into_iter().map(str::to_owned).collect();
    if inputs.len() > MAX_INPUTS {
        return Err(format!(
            "a tabela verdade permite no máximo {MAX_INPUTS} entradas"
        ));
    }
    if display == DisplayMode::Reduced && inputs.len() > MAX_REDUCED_INPUTS {
        return Err(format!(
            "a tabela reduzida permite no máximo {MAX_REDUCED_INPUTS} entradas; use a exibição completa"
        ));
    }

    let mut expressions = Vec::new();
    if include_intermediate {
        collect_columns(&expression, &mut expressions);
    }
    // A bare input or constant still needs a final output column.
    if expressions.is_empty() {
        expressions.push(&expression);
    }
    let total_rows = 1usize << inputs.len();
    if inputs.len() + expressions.len() > MAX_TABLE_CELLS / total_rows {
        return Err(format!(
            "a tabela excede o limite de {MAX_TABLE_CELLS} células"
        ));
    }

    let mut assignments: HashMap<String, bool> =
        inputs.iter().map(|name| (name.clone(), false)).collect();
    let mut outputs = Vec::with_capacity(total_rows);
    for index in 0..total_rows {
        for (column, name) in inputs.iter().enumerate() {
            *assignments.get_mut(name).expect("collected input") =
                is_input_set(index, column, inputs.len());
        }
        outputs.push(
            expressions
                .iter()
                .map(|expression| expression.evaluate(&assignments).map(u8::from))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    let rows = match display {
        DisplayMode::Complete => outputs
            .into_iter()
            .enumerate()
            .map(|(index, outputs)| TruthTableRow {
                inputs: row_inputs(index, 0, inputs.len()),
                outputs,
            })
            .collect(),
        DisplayMode::Reduced => compact(outputs, inputs.len()),
    };
    Ok(TruthTable {
        inputs,
        columns: expressions
            .iter()
            .map(|expression| boolean_algebra::format(expression, None, false, notation))
            .collect(),
        rows,
        total_rows,
    })
}

fn collect_inputs<'a>(expression: &'a Expr, names: &mut BTreeSet<&'a str>) {
    match expression {
        Expr::Var(name) => {
            names.insert(name);
        }
        Expr::Not(inner) => collect_inputs(inner, names),
        Expr::Bin(left, _, right) => {
            collect_inputs(left, names);
            collect_inputs(right, names);
        }
        Expr::Const(_) => {}
    }
}

/// Post-order puts each subexpression before expressions that depend on it.
/// Structurally identical subexpressions are displayed only once.
fn collect_columns<'a>(expression: &'a Expr, columns: &mut Vec<&'a Expr>) {
    match expression {
        Expr::Not(inner) => collect_columns(inner, columns),
        Expr::Bin(left, _, right) => {
            collect_columns(left, columns);
            collect_columns(right, columns);
        }
        Expr::Var(_) | Expr::Const(_) => return,
    }
    if !columns.contains(&expression) {
        columns.push(expression);
    }
}

fn is_input_set(index: usize, column: usize, inputs: usize) -> bool {
    index & (1 << (inputs - column - 1)) != 0
}

fn row_inputs(index: usize, unknowns: usize, count: usize) -> Vec<Option<u8>> {
    (0..count)
        .map(|column| {
            if is_input_set(unknowns, column, count) {
                None
            } else {
                Some(u8::from(is_input_set(index, column, count)))
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Implicant {
    /// Known input values; bits marked unknown are always cleared.
    values: usize,
    unknowns: usize,
}

impl Implicant {
    fn overlaps(self, other: Self) -> bool {
        let unknowns = self.unknowns | other.unknowns;
        self.values & !unknowns == other.values & !unknowns
    }
}

/// Like Logisim, group by the *entire* output vector, merge adjacent cubes,
/// then greedily select the most general disjoint cubes in each region.
/// This produces a partition, not overlapping terms of a minimized formula.
fn compact(outputs: Vec<Vec<u8>>, inputs: usize) -> Vec<TruthTableRow> {
    let mut regions: BTreeMap<Vec<u8>, HashSet<Implicant>> = BTreeMap::new();
    for (values, output) in outputs.into_iter().enumerate() {
        regions.entry(output).or_default().insert(Implicant {
            values,
            unknowns: 0,
        });
    }
    let mut partition = BTreeMap::new();
    for (output, mut current) in regions {
        let mut all = HashSet::new();
        while !current.is_empty() {
            let mut next = HashSet::new();
            for &implicant in &current {
                for column in 0..inputs {
                    let bit = 1 << column;
                    if implicant.unknowns & bit != 0 {
                        continue;
                    }
                    let opposite = Implicant {
                        values: implicant.values ^ bit,
                        unknowns: implicant.unknowns,
                    };
                    if current.contains(&opposite) {
                        next.insert(Implicant {
                            values: implicant.values & !bit,
                            unknowns: implicant.unknowns | bit,
                        });
                    }
                }
            }
            all.extend(current);
            current = next;
        }
        let mut sorted: Vec<Implicant> = all.into_iter().collect();
        sorted.sort_by_key(|implicant| {
            (
                std::cmp::Reverse(implicant.unknowns.count_ones()),
                implicant.values,
                implicant.unknowns,
            )
        });
        let mut chosen = Vec::new();
        for implicant in sorted {
            if chosen.iter().all(|&other| !implicant.overlaps(other)) {
                chosen.push(implicant);
                partition.insert(implicant, output.clone());
            }
        }
    }
    partition
        .into_iter()
        .map(|(implicant, outputs)| TruthTableRow {
            inputs: row_inputs(implicant.values, implicant.unknowns, inputs),
            outputs,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(source: &str, display: DisplayMode, intermediate: bool) -> TruthTable {
        generate(source, Notation::Default, display, intermediate).unwrap()
    }

    #[test]
    fn evaluates_all_operators_in_logisim_row_order() {
        let table = table("~A + (B & C) ^ A", DisplayMode::Complete, false);
        assert_eq!(table.inputs, ["A", "B", "C"]);
        assert_eq!(table.total_rows, 8);
        for (index, row) in table.rows.iter().enumerate() {
            let a = index & 4 != 0;
            let b = index & 2 != 0;
            let c = index & 1 != 0;
            assert_eq!(
                row.inputs,
                [Some(u8::from(a)), Some(u8::from(b)), Some(u8::from(c))]
            );
            assert_eq!(row.outputs, [u8::from(!a | ((b & c) ^ a))]);
        }
    }

    #[test]
    fn reproduces_the_existing_truth_table_example() {
        let table = table("~((~A + B) & (~B + C))", DisplayMode::Complete, true);
        assert_eq!(table.columns.len(), 6);
        let actual: Vec<Vec<u8>> = table.rows.iter().map(|row| row.outputs.clone()).collect();
        assert_eq!(
            actual,
            vec![
                vec![1, 1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 1, 0],
                vec![1, 1, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 1, 0],
                vec![0, 0, 1, 1, 0, 1],
                vec![0, 0, 1, 1, 0, 1],
                vec![0, 1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 1, 0],
            ]
        );
    }

    #[test]
    fn deduplicates_inputs_and_intermediate_expressions() {
        let table = table("(Z + A) & (Z + A)", DisplayMode::Complete, true);
        assert_eq!(table.inputs, ["A", "Z"]);
        assert_eq!(table.columns, ["Z + A", "(Z + A) & (Z + A)"]);
        assert_eq!(table.rows.len(), 4);
        for row in table.rows {
            assert_eq!(row.outputs[0], row.outputs[1]);
        }
    }

    #[test]
    fn handles_constants_and_single_variables() {
        for mode in [DisplayMode::Complete, DisplayMode::Reduced] {
            for value in [0, 1] {
                let table = table(&value.to_string(), mode, true);
                assert!(table.inputs.is_empty());
                assert_eq!(table.total_rows, 1);
                assert_eq!(
                    table.rows,
                    [TruthTableRow {
                        inputs: vec![],
                        outputs: vec![value]
                    }]
                );
            }
            let table = table("A", mode, true);
            assert_eq!(table.columns, ["A"]);
            assert_eq!(table.rows[0].outputs, [0]);
            assert_eq!(table.rows[1].outputs, [1]);
        }
    }

    #[test]
    fn preserves_inputs_that_cancel_out_of_the_result() {
        let complete = table("A + ~A", DisplayMode::Complete, false);
        assert_eq!(complete.inputs, ["A"]);
        assert_eq!(complete.rows.len(), 2);
        let reduced = table("A + ~A", DisplayMode::Reduced, false);
        assert_eq!(reduced.total_rows, 2);
        assert_eq!(
            reduced.rows,
            [TruthTableRow {
                inputs: vec![None],
                outputs: vec![1]
            }]
        );
    }

    #[test]
    fn compacts_or_into_disjoint_regions() {
        let table = table("A + B", DisplayMode::Reduced, false);
        assert_eq!(
            table.rows,
            [
                TruthTableRow {
                    inputs: vec![Some(0), Some(0)],
                    outputs: vec![0]
                },
                TruthTableRow {
                    inputs: vec![None, Some(1)],
                    outputs: vec![1]
                },
                TruthTableRow {
                    inputs: vec![Some(1), Some(0)],
                    outputs: vec![1]
                },
            ]
        );
    }

    fn assert_partition(full: &[Vec<u8>], rows: &[TruthTableRow], inputs: usize) {
        for (index, expected) in full.iter().enumerate() {
            let matches: Vec<_> = rows
                .iter()
                .filter(|row| {
                    row.inputs.iter().enumerate().all(|(column, value)| {
                        value.is_none_or(|value| {
                            value == u8::from(is_input_set(index, column, inputs))
                        })
                    })
                })
                .collect();
            assert_eq!(
                matches.len(),
                1,
                "combination {index} must be covered exactly once"
            );
            assert_eq!(&matches[0].outputs, expected);
        }
    }

    #[test]
    fn reduced_rows_partition_every_three_variable_boolean_function() {
        // All 256 possible truth tables: every combination must be covered
        // once, and expanding don't-cares must recover its original output.
        for function in 0..256 {
            let full: Vec<Vec<u8>> = (0..8)
                .map(|row| vec![((function >> row) & 1) as u8])
                .collect();
            let rows = compact(full.clone(), 3);
            assert_partition(&full, &rows, 3);
            assert!(rows.len() <= full.len());
            assert_eq!(rows, compact(full, 3));
        }
    }

    #[test]
    fn reduced_rows_preserve_all_displayed_intermediate_columns() {
        let complete = table("~((~A + B) & (~B + C))", DisplayMode::Complete, true);
        let reduced = table("~((~A + B) & (~B + C))", DisplayMode::Reduced, true);
        let full: Vec<_> = complete.rows.into_iter().map(|row| row.outputs).collect();
        assert_eq!(reduced.columns, complete.columns);
        assert_partition(&full, &reduced.rows, 3);
    }

    #[test]
    fn uses_the_requested_notation_for_headers() {
        let table = generate("A & ~B", Notation::ProgBools, DisplayMode::Complete, true).unwrap();
        assert_eq!(table.columns, ["!B", "A && !B"]);
    }

    #[test]
    fn reports_invalid_expressions_and_resource_limits() {
        for source in ["", "A &", "(A + B", "A + )"] {
            assert!(generate(source, Notation::Default, DisplayMode::Complete, false).is_err());
        }
        let too_many = (0..=MAX_INPUTS)
            .map(|i| format!("X{i}"))
            .collect::<Vec<_>>()
            .join(" + ");
        assert!(
            generate(&too_many, Notation::Default, DisplayMode::Complete, false)
                .unwrap_err()
                .contains("20 entradas")
        );
        let expensive = (0..=MAX_REDUCED_INPUTS)
            .map(|i| format!("X{i}"))
            .collect::<Vec<_>>()
            .join(" + ");
        assert!(
            generate(&expensive, Notation::Default, DisplayMode::Reduced, false)
                .unwrap_err()
                .contains("10 entradas")
        );
        // Intermediate columns would exceed the cell budget at 20 inputs.
        let wide = (0..MAX_INPUTS)
            .map(|i| format!("X{i}"))
            .collect::<Vec<_>>()
            .join(" + ");
        assert!(
            generate(&wide, Notation::Default, DisplayMode::Complete, true)
                .unwrap_err()
                .contains("células")
        );
    }

    #[test]
    fn missing_assignments_are_errors_even_in_short_circuit_branches() {
        let expression = Parser::parse("0 & A").unwrap();
        assert!(
            expression
                .evaluate(&HashMap::new())
                .unwrap_err()
                .contains("'A'")
        );
    }
}
