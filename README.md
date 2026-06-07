# Neural Crest

[![crates.io](https://img.shields.io/crates/v/neural-crest.svg)](https://crates.io/crates/neural-crest)
[![docs.rs](https://docs.rs/neural-crest/badge.svg)](https://docs.rs/neural-crest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Neural crest cell migration patterns for agent deployment and differentiation.**

---

## The Problem

Deploying agents into a multi-agent environment is like releasing cells into a developing embryo — they need to migrate to the right locations, differentiate into the right specializations, and form the right connections. Without a structured migration model, agents cluster randomly or fail to find their niche.

## Why This Exists

Inspired by biological **neural crest cells** — the remarkable embryonic cells that migrate throughout the body and differentiate into neurons, cartilage, bone, and pigment cells — this crate provides:
- **CrestCell**: Migratory agents with position, velocity, and differentiation potential
- **GradientField**: Chemotactic gradient fields for guided migration
- **Differentiation**: Cell type specialization based on environmental cues
- **NeuralTube**: Source structures that emit cells
- **Placode**: Destination structures that receive cells

## Architecture

```
  Neural Tube (source)          Gradient Field
  ┌────────────────┐           ┌───────────────────┐
  │ ╔══╗ ╔══╗      │    ┌────→│ ↑  ↑  ↑  ↑  ↑  ↑  │
  │ ║C1║ ║C2║ ...  │────┘     │ →  ·  ·  ·  ·  ←  │
  │ ╚══╝ ╚══╝      │          │ ↓  ↓  ↓  ↓  ↓  ↓  │
  └────────────────┘           └───────────────────┘
         │                              │
         ▼                              ▼
  Migration ──→ Differentiation ──→ Placode (destination)
                  │
         ┌───────┼───────┐
         │       │       │
      Neuron  Melanocyte Cartilage
      Glial   Bone      Muscle
```

## Installation

```toml
[dependencies]
neural-crest = "0.1"
```

## API Reference

### `CrestCell`, `Position`, `Velocity`

Migratory agent with 2D physics:

```rust
use neural_crest::crest_cell::*;

let pos = Position::new(10.0, 20.0);
let vel = Velocity::new(1.0, 0.5);
let mut cell = CrestCell::new(0, pos, vel);
cell.migrate(1.0); // advance one time step
```

### `GradientField`

Chemotactic concentration field:

```rust
use neural_crest::migration_path::GradientField;

let mut field = GradientField::new(100, 100);
field.set(50, 50, 1.0); // concentration source
let gradient = field.gradient_at(45, 45); // direction to source
```

### `CellType` & Differentiation

```rust
use neural_crest::differentiation::CellType;

let types = vec![
    CellType::Neuron,
    CellType::GlialCell,
    CellType::Melanocyte,
    CellType::Cartilage,
    CellType::Bone,
    CellType::SmoothMuscle,
];
```

### `NeuralTube`

Source of crest cells:

```rust
use neural_crest::neural_tube::NeuralTube;
use neural_crest::crest_cell::Position;

let tube = NeuralTube::new(Position::new(0.0, 50.0), Position::new(0.0, 50.0))
    .with_rate(3)
    .with_max(100);
```

## Usage Examples

### Example 1: Guided Cell Migration

```rust
use neural_crest::crest_cell::*;
use neural_crest::migration_path::GradientField;

let mut field = GradientField::new(100, 100);
field.set(80, 50, 1.0); // destination concentration

let pos = Position::new(20.0, 50.0);
let cell = CrestCell::new(0, pos, Velocity::zero());

// Cell follows gradient toward destination
```

### Example 2: Cell Differentiation

```rust
use neural_crest::differentiation::CellType;
use neural_crest::crest_cell::CrestCell;

// Cells differentiate based on their destination environment
let cell_type = CellType::Neuron;
println!("Differentiated into: {}", cell_type);
```

## Performance

| Operation | Complexity |
|-----------|-----------|
| Cell migration | O(1) |
| Gradient lookup | O(1) |
| Field construction | O(W × H) |
| Cell emission | O(rate) |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Push and open a Pull Request

## Mathematical Background

**Chemotaxis**: Cells follow concentration gradients using the equation:

```
∇C = (∂C/∂x, ∂C/∂y)
```

The cell's velocity is biased toward higher concentration:

```
v = v₀ + α × ∇C / |∇C|
```

Where α is the chemotactic sensitivity.

**Neural Crest Migration** follows a random walk with drift:

```
Δx = μdt + σdW
```

Where μ is the drift (from chemotaxis) and σdW is the random component (Brownian motion).

**Differentiation** is modeled as a function of positional information:

```
cell_type = f(position, gradient_exposure, time)
```

Cells that reach different destinations differentiate into different types, following the French Flag Model (Wolpert, 1969).

## Performance Characteristics

| Operation | Complexity |
|-----------|-----------|
| Cell migration | O(1) per cell |
| Gradient computation | O(1) lookup |
| Differentiation | O(1) |
| Tube emission | O(rate) |
| Full simulation step | O(cells × fields) |

## Comparison with Alternatives

| Feature | neural-crest | boids | particle-sim |
|---------|-------------|-------|-------------|
| Gradient following | ✅ | ❌ | ✅ |
| Differentiation | ✅ | ❌ | ❌ |
| Source emission | ✅ | ❌ | ✅ |
| Biological fidelity | ✅ | ❌ | ❌ |
| Agent deployment model | ✅ | ❌ | ❌ |

## API Reference

### `crest_cell`

Migratory agent with 2D physics:

```rust
use neural_crest::crest_cell::*;

let pos = Position::new(10.0, 20.0);
let vel = Velocity::new(1.0, 0.5);

let mut cell = CrestCell::new(0, pos, vel);
cell.migrate(1.0);

let distance = pos.distance_to(&Position::new(15.0, 20.0));
let direction = pos.direction_to(&Position::new(15.0, 20.0));
```

### `migration_path`

Chemotactic gradient fields for guided migration:

```rust
use neural_crest::migration_path::GradientField;

let mut field = GradientField::new(100, 100);
field.set(50, 50, 1.0); // highest concentration at center
let gradient = field.gradient_at(45, 45);
```

### `differentiation`

Cell type specialization based on environment:

```rust
use neural_crest::differentiation::CellType;

// Types: Undifferentiated, Neuron, GlialCell, Melanocyte,
//        Cartilage, Bone, SmoothMuscle, ConnectiveTissue
```

### `neural_tube`

Source structure that emits crest cells:

```rust
use neural_crest::neural_tube::NeuralTube;
use neural_crest::crest_cell::Position;

let tube = NeuralTube::new(
    Position::new(0.0, 0.0),
    Position::new(0.0, 100.0)
).with_rate(3).with_max(50);
```

### `placode`

Destination structures that receive differentiated cells.
