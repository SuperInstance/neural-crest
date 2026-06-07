//! Source structure (neural tube) from which cells emigrate.

use crate::crest_cell::{CrestCell, Position, Velocity};

/// Neural tube: a source of crest cells.
pub struct NeuralTube {
    pub start: Position,
    pub end: Position,
    pub emit_rate: usize,
    pub cells_emitted: usize,
    pub max_cells: usize,
}

impl NeuralTube {
    pub fn new(start: Position, end: Position) -> Self {
        NeuralTube { start, end, emit_rate: 1, cells_emitted: 0, max_cells: usize::MAX }
    }

    pub fn with_rate(mut self, rate: usize) -> Self { self.emit_rate = rate; self }
    pub fn with_max(mut self, max: usize) -> Self { self.max_cells = max; self }

    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }

    pub fn midpoint(&self) -> Position {
        Position::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }

    /// Point along the tube at parameter t in [0, 1].
    pub fn point_at(&self, t: f64) -> Position {
        Position::new(
            self.start.x + t * (self.end.x - self.start.x),
            self.start.y + t * (self.end.y - self.start.y),
        )
    }

    /// Emit cells from the tube.
    pub fn emit(&mut self, count: usize) -> Vec<CrestCell> {
        let actual = count.min(self.emit_rate).min(self.max_cells - self.cells_emitted);
        let mut cells = Vec::new();
        for i in 0..actual {
            let t = if actual > 1 { i as f64 / (actual - 1) as f64 } else { 0.5 };
            let pos = self.point_at(t);
            let cell = CrestCell::new(self.cells_emitted + i, pos);
            cells.push(cell);
        }
        self.cells_emitted += actual;
        cells
    }

    pub fn is_exhausted(&self) -> bool {
        self.cells_emitted >= self.max_cells
    }

    pub fn remaining(&self) -> usize {
        self.max_cells.saturating_sub(self.cells_emitted)
    }
}

/// Dorsal-ventral axis direction.
pub fn dorsal_direction(tube: &NeuralTube) -> Velocity {
    // Perpendicular to the tube axis, pointing "outward"
    let dx = tube.end.x - tube.start.x;
    let dy = tube.end.y - tube.start.y;
    Velocity::new(-dy, dx) // perpendicular
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tube_length() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(3.0, 4.0));
        assert!((t.length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_midpoint() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 10.0));
        let mid = t.midpoint();
        assert!((mid.x - 5.0).abs() < 1e-10);
        assert!((mid.y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_point_at_start() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 0.0));
        let p = t.point_at(0.0);
        assert!((p.x - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_point_at_end() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 0.0));
        let p = t.point_at(1.0);
        assert!((p.x - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_point_at_mid() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 0.0));
        let p = t.point_at(0.5);
        assert!((p.x - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_emit() {
        let mut t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 0.0))
            .with_rate(3);
        let cells = t.emit(3);
        assert_eq!(cells.len(), 3);
        assert_eq!(t.cells_emitted, 3);
    }

    #[test]
    fn test_tube_emit_with_max() {
        let mut t = NeuralTube::new(Position::origin(), Position::new(10.0, 0.0))
            .with_rate(5).with_max(2);
        let cells = t.emit(5);
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn test_tube_exhausted() {
        let mut t = NeuralTube::new(Position::origin(), Position::new(10.0, 0.0))
            .with_rate(5).with_max(5);
        t.emit(5);
        assert!(t.is_exhausted());
    }

    #[test]
    fn test_tube_remaining() {
        let mut t = NeuralTube::new(Position::origin(), Position::new(10.0, 0.0))
            .with_rate(5).with_max(10);
        t.emit(3);
        assert_eq!(t.remaining(), 7);
    }

    #[test]
    fn test_dorsal_direction() {
        let t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(1.0, 0.0));
        let v = dorsal_direction(&t);
        // Perpendicular to (1,0) is (0,1)
        assert!((v.dx - 0.0).abs() < 1e-10);
        assert!((v.dy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tube_emit_single() {
        let mut t = NeuralTube::new(Position::new(0.0, 0.0), Position::new(10.0, 0.0))
            .with_rate(1);
        let cells = t.emit(1);
        assert_eq!(cells.len(), 1);
        // Single cell should be at midpoint
        assert!((cells[0].position.x - 5.0).abs() < 1e-10);
    }
}
