use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::Serialize;

use super::truth_table::collect_inputs;
use crate::models::{parser::Parser, truth_table::MAX_INPUTS};

#[derive(Debug, Serialize)]
pub struct Counterexample {
    inputs: BTreeMap<String, bool>,
    original: bool,
    candidate: bool,
}

#[derive(Debug, Serialize)]
pub struct EquivalenceResult {
    equivalent: bool,
    counterexample: Option<Counterexample>,
}

pub fn verify(original: &str, candidate: &str) -> Result<EquivalenceResult, String> {
    let original =
        Parser::parse(original).map_err(|error| format!("Expressão original inválida: {error}"))?;
    let candidate = Parser::parse(candidate)
        .map_err(|error| format!("Tentativa de simplificação inválida: {error}"))?;

    let mut names = BTreeSet::new();
    collect_inputs(&original, &mut names);
    collect_inputs(&candidate, &mut names);
    if names.len() > MAX_INPUTS {
        return Err(format!(
            "a verificação permite no máximo {MAX_INPUTS} variáveis entre as duas expressões"
        ));
    }
    let names: Vec<_> = names.into_iter().collect();
    let mut assignments: HashMap<String, bool> = names
        .iter()
        .map(|name| ((*name).to_owned(), false))
        .collect();

    for index in 0..(1usize << names.len()) {
        for (column, name) in names.iter().enumerate() {
            *assignments.get_mut(*name).expect("collected input") =
                index & (1usize << (names.len() - column - 1)) != 0;
        }
        let original_value = original.evaluate(&assignments)?;
        let candidate_value = candidate.evaluate(&assignments)?;
        if original_value != candidate_value {
            return Ok(EquivalenceResult {
                equivalent: false,
                counterexample: Some(Counterexample {
                    inputs: assignments.into_iter().collect(),
                    original: original_value,
                    candidate: candidate_value,
                }),
            });
        }
    }
    Ok(EquivalenceResult {
        equivalent: true,
        counterexample: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_equivalence_with_different_variables_and_structure() {
        for (original, candidate) in [
            ("A & (B + ~B)", "A"),
            ("A", "A & (B + ~B)"),
            ("~(A + B)", "~A & ~B"),
            ("A + ~A", "1"),
            ("0", "0"),
        ] {
            let result = verify(original, candidate).unwrap();
            assert!(result.equivalent, "{original} vs {candidate}");
            assert!(result.counterexample.is_none());
        }
    }

    #[test]
    fn returns_a_reproducible_counterexample_for_different_input_sets() {
        let result = verify("A", "B").unwrap();
        assert!(!result.equivalent);
        let example = result.counterexample.unwrap();
        assert_eq!(
            example.inputs,
            BTreeMap::from([("A".into(), false), ("B".into(), true)])
        );
        assert!(!example.original);
        assert!(example.candidate);
    }

    #[test]
    fn evaluates_constants_without_inputs() {
        let example = verify("0", "1").unwrap().counterexample.unwrap();
        assert!(example.inputs.is_empty());
        assert_ne!(example.original, example.candidate);
    }

    #[test]
    fn identifies_which_expression_is_invalid() {
        assert!(verify("", "A").unwrap_err().contains("original inválida"));
        assert!(
            verify("A", "A &")
                .unwrap_err()
                .contains("simplificação inválida")
        );
    }

    #[test]
    fn limits_the_union_of_inputs() {
        let original = (b'A'..=b'T')
            .map(|name| char::from(name).to_string())
            .collect::<Vec<_>>()
            .join(" + ");
        assert!(verify(&original, "U").unwrap_err().contains("20 variáveis"));
    }

    #[test]
    fn serializes_the_frontend_contract() {
        let value = serde_json::to_value(verify("0", "1").unwrap()).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "equivalent": false,
                "counterexample": { "inputs": {}, "original": false, "candidate": true }
            })
        );
    }
}
