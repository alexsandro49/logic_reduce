use crate::models::notation;
use crate::models::parser;
use crate::services::boolean_algebra;

#[tauri::command]
pub fn simplify_helper(source: &str) -> String {
    match parser::Parser::parse(&source) {
        Ok(parsed) => {
            let (result, _) = boolean_algebra::simplify(parsed.clone());
            return boolean_algebra::format(&result, None, false, notation::Notation::Default);
        }
        Err(e) => {
            return format!("Erro: {}", e);
        }
    }
}
