use crate::models::{law::Law, parser::Parser, rewrite::Rewrite};
use crate::services::boolean_algebra;
use crate::services::equivalence::{self, EquivalenceResult};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ManualStepValidation {
    pub equivalence: EquivalenceResult,
    pub law_valid: bool,
    pub hint: String,
}

/// The law belongs to the source row and must explain the transition to the next row.
pub fn verify_step(
    original: &str,
    previous: &str,
    candidate: &str,
    selected_rule: &str,
) -> Result<ManualStepValidation, String> {
    let equivalence = equivalence::verify(original, candidate)?;
    let before = Parser::parse(previous)
        .map_err(|error| format!("Expressão do passo anterior inválida: {error}"))?;
    let after = Parser::parse(candidate)?;
    let selected = Law::from_id(selected_rule);
    let law_valid = selected.is_some_and(|law| Rewrite::explains_step(&before, &after, law));

    let hint = if !equivalence.equivalent {
        for_step(previous, selected_rule)?
    } else if law_valid {
        String::new()
    } else if let Some(correct) = Law::ALL
        .into_iter()
        .find(|law| Rewrite::explains_step(&before, &after, *law))
    {
        format!(
            "A expressão está correta, mas a lei selecionada está incorreta. Use a lei {}.",
            correct.label()
        )
    } else {
        "A expressão é equivalente, mas a lei selecionada não justifica essa transformação em um passo. Revise a lei e os passos intermediários."
            .into()
    };

    Ok(ManualStepValidation {
        equivalence,
        law_valid,
        hint,
    })
}

#[derive(Debug, Serialize)]
pub struct ManualCompletion {
    pub complete: bool,
    pub next_rule: Option<&'static str>,
}

pub fn completion(source: &str) -> Result<ManualCompletion, String> {
    let expression =
        Parser::parse(source).map_err(|error| format!("Expressão final inválida: {error}"))?;
    let (_, steps) = boolean_algebra::simplify(expression);
    // Operand order and grouping alone do not make a valid result unfinished.
    // Inspect the whole simplification, since reordering may expose a reduction.
    let next_rule = steps
        .iter()
        .find(|(_, _, law, _)| !matches!(law, Law::Commutativity | Law::Associativity))
        .map(|(_, _, law, _)| law.label());

    Ok(ManualCompletion {
        complete: next_rule.is_none(),
        next_rule,
    })
}

/// Suggests a law for an already identified, non-equivalent manual attempt.
pub fn for_step(previous: &str, selected_rule: &str) -> Result<String, String> {
    let expression = Parser::parse(previous)
        .map_err(|error| format!("Expressão do passo anterior inválida: {error}"))?;

    if let Some(selected) = Law::from_id(selected_rule)
        && Rewrite::rewrite_using(&expression, selected).is_some()
    {
        return Ok(format!(
            "A lei {} está correta, mas a expressão está incorreta. Revise a aplicação da lei.",
            selected.label()
        ));
    }

    Ok(match Rewrite::rewrite_once(&expression) {
        Some(rewrite) => format!("Use a lei {}.", rewrite.law.label()),
        None => "O motor não encontrou uma lei aplicável ao passo anterior. Revise a expressão informada."
            .into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_the_wrong_law_on_row_five_of_the_screenshot() {
        let rows = [
            ("~((~A + B) & (~B + C))", "commutative"),
            ("~((B + ~A) & (~B + C))", "commutative"),
            ("~((B + ~A) & (C + ~B))", "de_morgan"),
            ("~(B + ~A) + ~(C + ~B)", "de_morgan"),
            ("(~B & ~~A) + ~(C + ~B)", "identity"),
            ("(~B & A) + ~(C + ~B)", "commutative"),
            ("(A & ~B) + ~(C + ~B)", "de_morgan"),
            ("(A & ~B) + (~C & ~~B)", "double_negative"),
            ("(A & ~B) + (~C & B)", "commutative"),
            ("(A & ~B) + (B & ~C)", "final"),
        ];
        for (index, pair) in rows.windows(2).enumerate() {
            let result = verify_step(rows[0].0, pair[0].0, pair[1].0, pair[0].1).unwrap();
            assert!(result.equivalence.equivalent, "row {}", index + 1);
            assert_eq!(result.law_valid, index != 4, "row {}", index + 1);
            if index == 4 {
                assert!(result.hint.contains("lei selecionada está incorreta"));
                assert!(result.hint.contains("Use a lei D. Negação"));
                let corrected =
                    verify_step(rows[0].0, pair[0].0, pair[1].0, "double_negative").unwrap();
                assert!(corrected.law_valid);
                assert!(corrected.hint.is_empty());
            }
        }
        assert!(completion(rows.last().unwrap().0).unwrap().complete);
    }

    #[test]
    fn an_applicable_law_must_explain_the_actual_change() {
        let before = "(A & 1) + ~~B";
        let after = "(A & 1) + B";
        let result = verify_step(before, before, after, "identity").unwrap();
        assert!(result.equivalence.equivalent);
        assert!(!result.law_valid);
        assert!(result.hint.contains("Use a lei D. Negação"));

        let result = verify_step("A + (A & B)", "A + (A & B)", "A & B", "absorption").unwrap();
        assert!(!result.equivalence.equivalent);
        assert!(!result.law_valid);
        assert!(result.hint.contains("lei Absorção está correta"));
        assert!(result.hint.contains("expressão está incorreta"));
    }

    #[test]
    fn rejects_skipping_intermediate_laws_and_premature_final_labels() {
        let before = "A & (B + ~B)";
        let result = verify_step(before, before, "A", "complement").unwrap();
        assert!(result.equivalence.equivalent);
        assert!(!result.law_valid);
        assert!(result.hint.contains("passos intermediários"));

        let result = verify_step("~~A", "~~A", "A", "final").unwrap();
        assert!(result.equivalence.equivalent);
        assert!(!result.law_valid);
        assert!(result.hint.contains("Use a lei D. Negação"));
    }

    #[test]
    fn equivalent_partial_results_still_require_simplification() {
        for (source, next_rule) in [
            ("A & (B + ~B)", "Complemento"),
            ("A & 1", "Identidade"),
            ("(A & B) + (A & ~B)", "Distributiva"),
            ("~((B + ~A) & (C + ~B))", "De Morgan"),
        ] {
            let status = completion(source).unwrap();
            assert!(!status.complete, "{source}");
            assert_eq!(status.next_rule, Some(next_rule));
        }
    }

    #[test]
    fn accepts_finished_results_without_requiring_the_automatic_order() {
        for source in ["A", "0", "1", "B + A", "A + (B + C)", "(A & ~B) + (B & ~C)"] {
            let status = completion(source).unwrap();
            assert!(status.complete, "{source}");
            assert!(status.next_rule.is_none());
        }
        assert!(
            completion("A &")
                .unwrap_err()
                .contains("Expressão final inválida")
        );
    }

    #[test]
    fn suggests_a_law_from_the_previous_expression() {
        assert_eq!(
            for_step("A + (A & B)", "identity").unwrap(),
            "Use a lei Absorção."
        );
        assert_eq!(for_step("A + 0", "").unwrap(), "Use a lei Identidade.");
        assert_eq!(
            for_step("A & 0", "identity").unwrap(),
            "Use a lei Dominação."
        );
    }

    #[test]
    fn distinguishes_a_correct_law_from_an_incorrect_expression() {
        let message = for_step("A + (A & B)", "absorption").unwrap();
        assert!(message.contains("lei Absorção está correta"));
        assert!(message.contains("expressão está incorreta"));
        assert!(
            for_step("A & 0", "null")
                .unwrap()
                .contains("lei Dominação está correta")
        );
    }

    #[test]
    fn recognizes_valid_laws_beyond_the_automatic_first_choice() {
        // The automatic engine would reorder B + ~A before applying De Morgan.
        for (previous, selected_rule) in [
            ("~(B + ~A)", "de_morgan"),
            ("(B + A) & 1", "identity"),
            ("A + B", "commutative"),
            ("(A + B) + C", "associative"),
            ("A & (B + C)", "distributive"),
        ] {
            assert!(
                for_step(previous, selected_rule)
                    .unwrap()
                    .contains("está correta"),
                "{previous}"
            );
        }
    }

    #[test]
    fn does_not_invent_a_rule_for_an_irreducible_expression() {
        assert!(
            for_step("A", "absorption")
                .unwrap()
                .contains("não encontrou uma lei")
        );
        assert!(
            for_step("A ^ A", "complement")
                .unwrap()
                .contains("não encontrou uma lei")
        );
        assert!(
            for_step("A &", "identity")
                .unwrap_err()
                .contains("passo anterior inválida")
        );
    }
}
