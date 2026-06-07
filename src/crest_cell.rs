//! Migratory agent with position and velocity.

/// A 2D position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self { Position { x, y } }
    pub fn origin() -> Self { Position { x: 0.0, y: 0.0 } }
    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn direction_to(&self, other: &Position) -> (f64, f64) {
        let d = self.distance_to(other);
        if d == 0.0 { return (0.0, 0.0); }
        ((other.x - self.x) / d, (other.y - self.y) / d)
    }
    pub fn add(&self, dx: f64, dy: f64) -> Position {
        Position { x: self.x + dx, y: self.y + dy }
    }
}

/// A 2D velocity vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Self { Velocity { dx, dy } }
    pub fn zero() -> Self { Velocity { dx: 0.0, dy: 0.0 } }
    pub fn magnitude(&self) -> f64 {
        (self.dx.powi(2) + self.dy.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Velocity {
        let m = self.magnitude();
        if m == 0.0 { Velocity::zero() } else { Velocity { dx: self.dx / m, dy: self.dy / m } }
    }
    pub fn scale(&self, factor: f64) -> Velocity {
        Velocity { dx: self.dx * factor, dy: self.dy * factor }
    }
}

/// A migratory crest cell.
#[derive(Clone, Debug)]
pub struct CrestCell {
    pub id: usize,
    pub position: Position,
    pub velocity: Velocity,
    pub cell_type: String,
    pub age: usize,
    pub energy: f64,
    pub alive: bool,
}

impl CrestCell {
    pub fn new(id: usize, position: Position) -> Self {
        CrestCell { id, position, velocity: Velocity::zero(), cell_type: "undifferentiated".to_string(), age: 0, energy: 100.0, alive: true }
    }

    pub fn with_velocity(mut self, v: Velocity) -> Self { self.velocity = v; self }
    pub fn with_type(mut self, t: &str) -> Self { self.cell_type = t.to_string(); self }

    pub fn step(&mut self) {
        if !self.alive { return; }
        self.position = self.position.add(self.velocity.dx, self.velocity.dy);
        self.age += 1;
        self.energy -= self.velocity.magnitude();
        if self.energy <= 0.0 { self.alive = false; }
    }

    pub fn step_towards(&mut self, target: &Position, speed: f64) {
        if !self.alive { return; }
        let (dx, dy) = self.position.direction_to(target);
        self.velocity = Velocity::new(dx * speed, dy * speed);
        self.step();
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        self.position.distance_to(other)
    }

    pub fn recharge(&mut self, amount: f64) {
        self.energy += amount;
        if self.energy > 100.0 { self.energy = 100.0; }
    }

    pub fn is_differentiated(&self) -> bool {
        self.cell_type != "undifferentiated"
    }

    pub fn die(&mut self) { self.alive = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_distance() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_position_direction() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(0.0, 5.0);
        let (dx, dy) = a.direction_to(&b);
        assert!((dx - 0.0).abs() < 1e-10);
        assert!((dy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_position_direction_same() {
        let a = Position::new(1.0, 1.0);
        let (dx, dy) = a.direction_to(&a);
        assert_eq!((dx, dy), (0.0, 0.0));
    }

    #[test]
    fn test_position_add() {
        let p = Position::new(1.0, 2.0);
        let q = p.add(3.0, 4.0);
        assert!((q.x - 4.0).abs() < 1e-10);
        assert!((q.y - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_magnitude() {
        let v = Velocity::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_normalize() {
        let v = Velocity::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_normalize_zero() {
        let v = Velocity::zero();
        assert_eq!(v.normalize(), Velocity::zero());
    }

    #[test]
    fn test_velocity_scale() {
        let v = Velocity::new(1.0, 0.0);
        let s = v.scale(3.0);
        assert!((s.dx - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_crest_cell_new() {
        let c = CrestCell::new(0, Position::origin());
        assert_eq!(c.id, 0);
        assert!(!c.is_differentiated());
        assert!(c.alive);
    }

    #[test]
    fn test_crest_cell_step() {
        let mut c = CrestCell::new(0, Position::origin()).with_velocity(Velocity::new(1.0, 0.0));
        c.step();
        assert!((c.position.x - 1.0).abs() < 1e-10);
        assert_eq!(c.age, 1);
    }

    #[test]
    fn test_crest_cell_step_towards() {
        let mut c = CrestCell::new(0, Position::origin());
        let target = Position::new(10.0, 0.0);
        c.step_towards(&target, 2.0);
        assert!(c.position.x > 0.0);
    }

    #[test]
    fn test_crest_cell_energy_depletion() {
        let mut c = CrestCell::new(0, Position::origin())
            .with_velocity(Velocity::new(100.0, 0.0));
        c.energy = 50.0;
        c.step();
        assert!(!c.alive);
    }

    #[test]
    fn test_crest_cell_recharge() {
        let mut c = CrestCell::new(0, Position::origin());
        c.energy = 50.0;
        c.recharge(30.0);
        assert!((c.energy - 80.0).abs() < 1e-10);
    }

    #[test]
    fn test_crest_cell_recharge_cap() {
        let mut c = CrestCell::new(0, Position::origin());
        c.recharge(50.0);
        assert!((c.energy - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_crest_cell_die() {
        let mut c = CrestCell::new(0, Position::origin());
        c.die();
        assert!(!c.alive);
    }

    #[test]
    fn test_crest_cell_step_dead() {
        let mut c = CrestCell::new(0, Position::origin()).with_velocity(Velocity::new(1.0, 0.0));
        c.die();
        c.step();
        assert!((c.position.x - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_crest_cell_differentiated() {
        let c = CrestCell::new(0, Position::origin()).with_type("neuron");
        assert!(c.is_differentiated());
    }
}
