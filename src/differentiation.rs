//! Cell type specialization.

use crate::crest_cell::CrestCell;

/// Cell type identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    Undifferentiated,
    Neuron,
    GlialCell,
    Melanocyte,
    Cartilage,
    Bone,
    SmoothMuscle,
    ConnectiveTissue,
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Signal molecule concentration.
#[derive(Clone, Debug)]
pub struct DifferentiationSignal {
    pub signal_type: String,
    pub concentration: f64,
    pub threshold: f64,
}

impl DifferentiationSignal {
    pub fn new(signal_type: &str, concentration: f64, threshold: f64) -> Self {
        DifferentiationSignal { signal_type: signal_type.to_string(), concentration, threshold }
    }

    pub fn is_active(&self) -> bool {
        self.concentration >= self.threshold
    }
}

/// Differentiation rule: maps signal combinations to cell types.
pub struct DifferentiationRule {
    pub required_signals: Vec<String>,
    pub result_type: CellType,
}

impl DifferentiationRule {
    pub fn new(signals: Vec<&str>, result: CellType) -> Self {
        DifferentiationRule { required_signals: signals.into_iter().map(String::from).collect(), result_type: result }
    }

    pub fn matches(&self, active_signals: &[DifferentiationSignal]) -> bool {
        self.required_signals.iter().all(|req| {
            active_signals.iter().any(|s| s.signal_type == *req && s.is_active())
        })
    }
}

/// Apply differentiation rules to a cell.
pub fn differentiate_cell(cell: &mut CrestCell, rules: &[DifferentiationRule], signals: &[DifferentiationSignal]) -> bool {
    for rule in rules {
        if rule.matches(signals) {
            cell.cell_type = rule.result_type.to_string();
            return true;
        }
    }
    false
}

/// Check what cell types are possible given signals.
pub fn possible_types(rules: &[DifferentiationRule], signals: &[DifferentiationSignal]) -> Vec<CellType> {
    rules.iter().filter(|r| r.matches(signals)).map(|r| r.result_type.clone()).collect()
}

/// Count how many cells of each type exist.
pub fn count_by_type(cells: &[CrestCell]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for cell in cells {
        *counts.entry(cell.cell_type.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_type_display() {
        assert_eq!(CellType::Neuron.to_string(), "Neuron");
    }

    #[test]
    fn test_signal_active() {
        let s = DifferentiationSignal::new("wnt", 1.0, 0.5);
        assert!(s.is_active());
    }

    #[test]
    fn test_signal_inactive() {
        let s = DifferentiationSignal::new("wnt", 0.3, 0.5);
        assert!(!s.is_active());
    }

    #[test]
    fn test_signal_at_threshold() {
        let s = DifferentiationSignal::new("wnt", 0.5, 0.5);
        assert!(s.is_active());
    }

    #[test]
    fn test_rule_matches() {
        let rule = DifferentiationRule::new(vec!["bmp", "wnt"], CellType::Neuron);
        let signals = vec![
            DifferentiationSignal::new("bmp", 1.0, 0.5),
            DifferentiationSignal::new("wnt", 0.8, 0.5),
        ];
        assert!(rule.matches(&signals));
    }

    #[test]
    fn test_rule_no_match() {
        let rule = DifferentiationRule::new(vec!["bmp", "wnt"], CellType::Neuron);
        let signals = vec![
            DifferentiationSignal::new("bmp", 1.0, 0.5),
        ];
        assert!(!rule.matches(&signals));
    }

    #[test]
    fn test_rule_signal_inactive() {
        let rule = DifferentiationRule::new(vec!["bmp"], CellType::Neuron);
        let signals = vec![
            DifferentiationSignal::new("bmp", 0.1, 0.5),
        ];
        assert!(!rule.matches(&signals));
    }

    #[test]
    fn test_differentiate_cell_success() {
        let mut cell = CrestCell::new(0, crate::crest_cell::Position::origin());
        let rules = vec![DifferentiationRule::new(vec!["ntn"], CellType::Neuron)];
        let signals = vec![DifferentiationSignal::new("ntn", 1.0, 0.5)];
        assert!(differentiate_cell(&mut cell, &rules, &signals));
        assert_eq!(cell.cell_type, "Neuron");
    }

    #[test]
    fn test_differentiate_cell_no_match() {
        let mut cell = CrestCell::new(0, crate::crest_cell::Position::origin());
        let rules = vec![DifferentiationRule::new(vec!["bmp"], CellType::Cartilage)];
        let signals = vec![DifferentiationSignal::new("ntn", 1.0, 0.5)];
        assert!(!differentiate_cell(&mut cell, &rules, &signals));
    }

    #[test]
    fn test_possible_types() {
        let rules = vec![
            DifferentiationRule::new(vec!["a"], CellType::Neuron),
            DifferentiationRule::new(vec!["b"], CellType::GlialCell),
        ];
        let signals = vec![DifferentiationSignal::new("a", 1.0, 0.5)];
        let types = possible_types(&rules, &signals);
        assert_eq!(types, vec![CellType::Neuron]);
    }

    #[test]
    fn test_count_by_type() {
        use crate::crest_cell::Position;
        let cells = vec![
            CrestCell::new(0, Position::origin()).with_type("Neuron"),
            CrestCell::new(1, Position::origin()).with_type("Neuron"),
            CrestCell::new(2, Position::origin()).with_type("Melanocyte"),
        ];
        let counts = count_by_type(&cells);
        assert_eq!(*counts.get("Neuron").unwrap(), 2);
        assert_eq!(*counts.get("Melanocyte").unwrap(), 1);
    }
}
