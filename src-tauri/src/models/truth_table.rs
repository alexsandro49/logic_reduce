use serde::{Deserialize, Serialize};

/// Logisim's AnalyzerModel.MAX_INPUTS limit.
pub const MAX_INPUTS: usize = 20;
/// Compacting enumerates implicants and is more expensive than evaluation.
pub const MAX_REDUCED_INPUTS: usize = 10;
pub const MAX_TABLE_CELLS: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    #[default]
    Complete,
    Reduced,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct TruthTableRow {
    /// 0 or 1; None is a don't-care input (serialized as null).
    pub inputs: Vec<Option<u8>>,
    /// Values follow TruthTable.columns; the last value is the final result.
    pub outputs: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct TruthTable {
    /// Distinct input names in lexicographical order.
    pub inputs: Vec<String>,
    /// Output expression labels, including any requested intermediate columns.
    pub columns: Vec<String>,
    pub rows: Vec<TruthTableRow>,
    /// Number of full input combinations, even when rows are compacted.
    pub total_rows: usize,
}
