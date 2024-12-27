use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAttributes {
    pub color: Option<String>,    // Tailwind class or hex TBD
    pub intensity: Option<f64>,   // Prob use for opacity
    pub size: Option<f64>,        // TBD if needed
    pub group: Option<String>,    // TBD if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualElement {
    pub position: Point,
    pub attributes: VisualAttributes,
    pub timestamp: u64,
    // pub metadata: Option<serde_json::Value>,  // For task-specific data but TBD if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualisationFrame {
    pub elements: Vec<VisualElement>,
    pub frame_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationProgress {
    pub current_value: u64,
    pub items_processed: u32,
    pub progress_percentage: f32,
    pub memory_usage: u64,
    pub elapsed_time_ms: u64,
}