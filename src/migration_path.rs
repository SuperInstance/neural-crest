//! Chemotaxis gradient following.

use crate::crest_cell::{CrestCell, Position};

/// A chemotactic gradient field.
pub struct GradientField {
    width: usize,
    height: usize,
    concentrations: Vec<f64>,
}

impl GradientField {
    pub fn new(width: usize, height: usize) -> Self {
        GradientField { width, height, concentrations: vec![0.0; width * height] }
    }

    pub fn set(&mut self, x: usize, y: usize, value: f64) {
        if x < self.width && y < self.height {
            self.concentrations[y * self.width + x] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        if x < self.width && y < self.height {
            self.concentrations[y * self.width + x]
        } else { 0.0 }
    }

    pub fn gradient_at(&self, x: usize, y: usize) -> (f64, f64) {
        let cx = self.get(x, y);
        let left = if x > 0 { self.get(x - 1, y) } else { cx };
        let right = if x + 1 < self.width { self.get(x + 1, y) } else { cx };
        let up = if y > 0 { self.get(x, y - 1) } else { cx };
        let down = if y + 1 < self.height { self.get(x, y + 1) } else { cx };
        ((right - left) / 2.0, (down - up) / 2.0)
    }

    /// Place a source that diffuses outward.
    pub fn add_source(&mut self, cx: usize, cy: usize, strength: f64, radius: usize) {
        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                let nx = (cx as isize + dx) as usize;
                let ny = (cy as isize + dy) as usize;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();
                if dist <= radius as f64 {
                    let val = strength * (1.0 - dist / (radius as f64 + 1.0));
                    if val > self.get(nx, ny) {
                        self.set(nx, ny, val);
                    }
                }
            }
        }
    }

    pub fn max_concentration(&self) -> f64 {
        self.concentrations.iter().cloned().fold(0.0_f64, f64::max)
    }

    pub fn total_concentration(&self) -> f64 {
        self.concentrations.iter().sum()
    }
}

/// A migration path with waypoints.
pub struct MigrationPath {
    pub waypoints: Vec<Position>,
    pub current: usize,
}

impl MigrationPath {
    pub fn new(waypoints: Vec<Position>) -> Self {
        MigrationPath { waypoints, current: 0 }
    }

    pub fn current_target(&self) -> Option<&Position> {
        self.waypoints.get(self.current)
    }

    pub fn advance(&mut self) -> bool {
        if self.current + 1 < self.waypoints.len() {
            self.current += 1;
            true
        } else { false }
    }

    pub fn is_complete(&self) -> bool {
        self.current >= self.waypoints.len()
    }

    pub fn total_length(&self) -> f64 {
        self.waypoints.windows(2).map(|w| w[0].distance_to(&w[1])).sum()
    }

    pub fn remaining_length(&self, pos: &Position) -> f64 {
        if self.is_complete() { return 0.0; }
        let mut total = pos.distance_to(&self.waypoints[self.current]);
        for i in self.current..self.waypoints.len().saturating_sub(1) {
            total += self.waypoints[i].distance_to(&self.waypoints[i + 1]);
        }
        total
    }
}

/// Move a cell along the gradient.
pub fn follow_gradient(cell: &mut CrestCell, field: &GradientField, speed: f64) {
    let gx = cell.position.x.max(0.0) as usize;
    let gy = cell.position.y.max(0.0) as usize;
    let (dx, dy) = field.gradient_at(gx.min(field.width - 1), gy.min(field.height - 1));
    let mag = (dx * dx + dy * dy).sqrt();
    if mag > 0.0 {
        cell.velocity = crate::crest_cell::Velocity::new(dx / mag * speed, dy / mag * speed);
    }
    cell.step();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_field_new() {
        let f = GradientField::new(10, 10);
        assert_eq!(f.get(5, 5), 0.0);
    }

    #[test]
    fn test_gradient_field_set_get() {
        let mut f = GradientField::new(10, 10);
        f.set(5, 5, 1.0);
        assert!((f.get(5, 5) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gradient_field_out_of_bounds() {
        let f = GradientField::new(5, 5);
        assert_eq!(f.get(10, 10), 0.0);
    }

    #[test]
    fn test_gradient_at_flat() {
        let f = GradientField::new(10, 10);
        let (gx, gy) = f.gradient_at(5, 5);
        assert_eq!((gx, gy), (0.0, 0.0));
    }

    #[test]
    fn test_gradient_at_slope() {
        let mut f = GradientField::new(10, 10);
        f.set(4, 5, 0.0);
        f.set(5, 5, 0.5);
        f.set(6, 5, 1.0);
        let (gx, _) = f.gradient_at(5, 5);
        assert!(gx > 0.0);
    }

    #[test]
    fn test_add_source() {
        let mut f = GradientField::new(20, 20);
        f.add_source(10, 10, 10.0, 3);
        assert!(f.get(10, 10) > 0.0);
    }

    #[test]
    fn test_max_concentration() {
        let mut f = GradientField::new(5, 5);
        f.set(2, 2, 5.0);
        assert!((f.max_concentration() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_total_concentration() {
        let mut f = GradientField::new(5, 5);
        f.set(0, 0, 1.0);
        f.set(1, 1, 2.0);
        assert!((f.total_concentration() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_migration_path_current() {
        let path = MigrationPath::new(vec![Position::new(0.0, 0.0), Position::new(10.0, 0.0)]);
        assert_eq!(path.current_target(), Some(&Position::new(0.0, 0.0)));
    }

    #[test]
    fn test_migration_path_advance() {
        let mut path = MigrationPath::new(vec![Position::new(0.0, 0.0), Position::new(10.0, 0.0)]);
        assert!(path.advance());
        assert_eq!(path.current, 1);
    }

    #[test]
    fn test_migration_path_not_complete_at_last_waypoint() {
        let mut path = MigrationPath::new(vec![Position::new(0.0, 0.0)]);
        assert!(!path.is_complete()); // still has a target
    }

    #[test]
    fn test_migration_path_complete_when_empty() {
        let path = MigrationPath::new(vec![]);
        assert!(path.is_complete());
    }

    #[test]
    fn test_migration_path_total_length() {
        let path = MigrationPath::new(vec![Position::new(0.0, 0.0), Position::new(3.0, 4.0)]);
        assert!((path.total_length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_migration_path_remaining() {
        let mut path = MigrationPath::new(vec![Position::origin(), Position::new(10.0, 0.0)]);
        path.advance();
        let remaining = path.remaining_length(&Position::new(5.0, 0.0));
        assert!((remaining - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_follow_gradient() {
        let mut field = GradientField::new(20, 20);
        field.set(15, 10, 10.0);
        let mut cell = CrestCell::new(0, Position::new(10.0, 10.0));
        follow_gradient(&mut cell, &field, 1.0);
        assert!(cell.age == 1);
    }
}
