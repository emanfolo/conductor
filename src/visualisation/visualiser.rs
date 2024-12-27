use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::visualisation::{ComputationProgress, Point, VisualAttributes, VisualElement, VisualisationFrame};


pub struct ComputationVisualiser;

impl ComputationVisualiser {
    pub fn generate_frame(&self, progress: &ComputationProgress) -> VisualisationFrame {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Only show points discovered in last 2 seconds
        let recent_points = vec![VisualElement {
            position: self.map_to_coordinates(progress.current_value),
            attributes: VisualAttributes {
                color: Some(self.calculate_color(progress)),
                // Fade out based on age
                intensity: Some(1.0),  // Start full intensity
                size: Some(1.0),
                group: None,
            },
            timestamp: now,
        }];

        VisualisationFrame {
            elements: recent_points,
            frame_index: progress.items_processed,
        }
    }

    fn map_to_coordinates(&self, value: u64) -> Point {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let angle = 2.0 * std::f64::consts::PI * value as f64 / phi;
        let radius = (value as f64).sqrt() * 2.0; // Scale factor for better spread
        
        let x = 400.0 + radius * angle.cos(); // Center at 400
        let y = 300.0 + radius * angle.sin(); // Center at 300
        
        Point { x, y, z: None }
    }

    fn calculate_color(&self, progress: &ComputationProgress) -> String {
        // HSL color scheme
        let hue = (progress.current_value % 360) as i32;
        let saturation = 70;
        let lightness = 50;
        
        format!("hsl({}, {}%, {}%)", hue, saturation, lightness)
    }
}