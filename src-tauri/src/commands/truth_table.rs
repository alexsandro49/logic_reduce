use crate::models::{
    notation::{self, Notation},
    truth_table::{DisplayMode, TruthTable},
};
use crate::services::truth_table;

#[tauri::command]
pub fn truth_table_helper(
    source: &str,
    notation: Option<String>,
    display: Option<DisplayMode>,
    include_intermediate: Option<bool>,
) -> Result<TruthTable, String> {
    let notation = notation
        .as_deref()
        .map(notation::parse_notation)
        .transpose()?
        .unwrap_or(Notation::Default);
    truth_table::generate(
        source,
        notation,
        display.unwrap_or_default(),
        include_intermediate.unwrap_or(false),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_complete_with_only_the_final_output() {
        let table = truth_table_helper("A + ~B", None, None, None).unwrap();
        assert_eq!(table.rows.len(), 4);
        assert_eq!(table.columns, ["A + ~B"]);
    }

    #[test]
    fn rejects_invalid_notation() {
        assert!(truth_table_helper("A", Some("unknown".into()), None, None).is_err());
    }

    #[test]
    fn serializes_reduced_inputs_as_numbers_or_null() {
        let mode = serde_json::from_str::<DisplayMode>("\"reduced\"").unwrap();
        let table = truth_table_helper("A + ~A", None, Some(mode), None).unwrap();
        assert_eq!(
            serde_json::to_value(table).unwrap(),
            serde_json::json!({
                "inputs": ["A"],
                "columns": ["A + ~A"],
                "rows": [{ "inputs": [null], "outputs": [1] }],
                "total_rows": 2
            })
        );
        assert!(serde_json::from_str::<DisplayMode>("\"invalid\"").is_err());
    }
}
