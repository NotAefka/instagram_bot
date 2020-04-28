use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct Timeouts {
    pub script: Option<usize>,
    pub page_load: usize,
    pub implicit: usize,
}
