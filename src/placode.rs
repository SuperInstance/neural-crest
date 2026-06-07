//! Destination cluster for migrating cells.

use crate::crest_cell::{CrestCell, Position};

/// A placode (destination cluster).
pub struct Placode {
    pub id: usize,
    pub center: Position,
    pub radius: f64,
    pub capacity: usize,
    pub accepted: Vec<usize>,
    pub target_type: String,
}

impl Placode {
    pub fn new(id: usize, center: Position, radius: f64, capacity: usize) -> Self {
        Placode { id, center, radius, capacity, accepted: Vec::new(), target_type: "any".to_string() }
    }

    pub fn with_type(mut self, t: &str) -> Self { self.target_type = t.to_string(); self }

    pub fn contains(&self, pos: &Position) -> bool {
        self.center.distance_to(pos) <= self.radius
    }

    pub fn is_full(&self) -> bool {
        self.accepted.len() >= self.capacity
    }

    pub fn accept(&mut self, cell_id: usize) -> bool {
        if self.is_full() { return false; }
        self.accepted.push(cell_id);
        true
    }

    pub fn occupancy(&self) -> f64 {
        if self.capacity == 0 { return 0.0; }
        self.accepted.len() as f64 / self.capacity as f64
    }

    pub fn remaining_capacity(&self) -> usize {
        self.capacity.saturating_sub(self.accepted.len())
    }
}

/// Find which placode (if any) a cell has arrived at.
pub fn find_arrival(cell: &CrestCell, placodes: &[Placode]) -> Option<usize> {
    placodes.iter().find(|p| p.contains(&cell.position)).map(|p| p.id)
}

/// Assign cells to the nearest placode.
pub fn assign_to_nearest(cells: &[CrestCell], placodes: &[Placode]) -> Vec<(usize, usize)> {
    cells.iter().filter_map(|c| {
        if !c.alive { return None; }
        let nearest = placodes.iter().min_by(|a, b| {
            c.position.distance_to(&a.center).partial_cmp(&c.position.distance_to(&b.center)).unwrap()
        })?;
        Some((c.id, nearest.id))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placode_contains_inside() {
        let p = Placode::new(0, Position::new(5.0, 5.0), 3.0, 10);
        assert!(p.contains(&Position::new(6.0, 5.0)));
    }

    #[test]
    fn test_placode_contains_outside() {
        let p = Placode::new(0, Position::new(5.0, 5.0), 1.0, 10);
        assert!(!p.contains(&Position::new(10.0, 5.0)));
    }

    #[test]
    fn test_placode_accept() {
        let mut p = Placode::new(0, Position::origin(), 5.0, 2);
        assert!(p.accept(1));
        assert!(p.accept(2));
        assert!(!p.accept(3)); // full
    }

    #[test]
    fn test_placode_is_full() {
        let mut p = Placode::new(0, Position::origin(), 5.0, 1);
        p.accept(1);
        assert!(p.is_full());
    }

    #[test]
    fn test_placode_occupancy() {
        let mut p = Placode::new(0, Position::origin(), 5.0, 4);
        p.accept(1);
        assert!((p.occupancy() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_placode_remaining() {
        let mut p = Placode::new(0, Position::origin(), 5.0, 3);
        p.accept(1);
        assert_eq!(p.remaining_capacity(), 2);
    }

    #[test]
    fn test_placode_with_type() {
        let p = Placode::new(0, Position::origin(), 5.0, 10).with_type("Neuron");
        assert_eq!(p.target_type, "Neuron");
    }

    #[test]
    fn test_find_arrival() {
        let cell = CrestCell::new(0, Position::new(5.0, 5.0));
        let placodes = vec![Placode::new(0, Position::new(5.0, 5.0), 1.0, 10)];
        assert_eq!(find_arrival(&cell, &placodes), Some(0));
    }

    #[test]
    fn test_find_arrival_none() {
        let cell = CrestCell::new(0, Position::new(0.0, 0.0));
        let placodes = vec![Placode::new(0, Position::new(50.0, 50.0), 1.0, 10)];
        assert_eq!(find_arrival(&cell, &placodes), None);
    }

    #[test]
    fn test_assign_to_nearest() {
        let cells = vec![
            CrestCell::new(0, Position::new(1.0, 1.0)),
            CrestCell::new(1, Position::new(9.0, 9.0)),
        ];
        let placodes = vec![
            Placode::new(0, Position::new(0.0, 0.0), 5.0, 10),
            Placode::new(1, Position::new(10.0, 10.0), 5.0, 10),
        ];
        let assignments = assign_to_nearest(&cells, &placodes);
        assert_eq!(assignments.len(), 2);
    }
}
