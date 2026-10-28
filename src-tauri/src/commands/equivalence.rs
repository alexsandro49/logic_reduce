use crate::services::equivalence::{self, EquivalenceResult};
use crate::services::manual_hint::{self, ManualStepValidation};

#[tauri::command]
pub async fn verify_manual_step(
    original: String,
    previous: String,
    candidate: String,
    selected_rule: String,
) -> Result<ManualStepValidation, String> {
    tauri::async_runtime::spawn_blocking(move || {
        manual_hint::verify_step(&original, &previous, &candidate, &selected_rule)
    })
    .await
    .map_err(|error| format!("Não foi possível verificar o passo: {error}"))?
}

#[tauri::command]
pub async fn verify_equivalence(
    original: String,
    candidate: String,
) -> Result<EquivalenceResult, String> {
    tauri::async_runtime::spawn_blocking(move || equivalence::verify(&original, &candidate))
        .await
        .map_err(|error| format!("Não foi possível verificar as expressões: {error}"))?
}
