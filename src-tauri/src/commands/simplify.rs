use crate::models::notation;
use crate::models::parser;
use crate::services::boolean_algebra;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Step {
    rule_name: String,
    after_rule: String
}



#[tauri::command]
pub fn simplify_helper(source: &str, notation: Option<String>) -> Result<(String, Vec<Step>), String> {
    let n = notation
        .as_deref()
        .map(notation::parse_notation)
        .transpose()?
        .unwrap_or(notation::Notation::Default);
    let mut steps_result: Vec<Step> = Vec::new();
    
    match parser::Parser::parse(&source) {
        Ok(parsed) => {
            let (result, steps) = boolean_algebra::simplify(parsed);
            
            for (_, after, _, desc) in steps {
                steps_result.push(Step { rule_name: String::from(desc), after_rule: boolean_algebra::format(&after, None, false, n)});
            }
            Ok((boolean_algebra::format(&result, None, false, n), steps_result))
        }
        Err(error) => Err(format!("Expressão inválida: {error}")),
    }
}
