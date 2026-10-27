use crate::services::equivalence::{self, EquivalenceResult};

#[tauri::command]
pub async fn verify_equivalence(
    original: String,
    candidate: String,
) -> Result<EquivalenceResult, String> {
    tauri::async_runtime::spawn_blocking(move || equivalence::verify(&original, &candidate))
        .await
        .map_err(|error| format!("Não foi possível verificar as expressões: {error}"))?
}
