use std::time::{Instant, Duration};
use std::collections::HashMap;

/// Modulo per l'analisi comportamentale delle minacce IA.
/// Monitora anomalie come la frequenza di accesso ai file da parte di processi esterni.
pub struct BehavioralAnalysis {
    access_log: HashMap<String, Vec<Instant>>,
    threshold_count: usize,
    threshold_window: Duration,
}

impl BehavioralAnalysis {
    pub fn new() -> Self {
        Self {
            access_log: HashMap::new(),
            threshold_count: 5, // Massimo 5 accessi in 1 secondo
            threshold_window: Duration::from_secs(1),
        }
    }

    /// Registra un accesso e restituisce true se rileva un'anomalia comportamentale.
    pub fn record_access(&mut self, process_name: &str) -> bool {
        let now = Instant::now();
        let history = self.access_log.entry(process_name.to_string()).or_insert_with(|| Vec::new());

        history.push(now);

        // Mantieni solo gli accessi all'interno della finestra temporale
        history.retain(|&time| now.duration_since(time) < self.threshold_window);

        // Rileva anomalia
        history.len() > self.threshold_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavioral_anomaly_detection() {
        let mut analysis = BehavioralAnalysis::new();
        let proc = "AI_External_Agent";

        // Simuliamo 5 accessi (limite massimo incluso)
        for _ in 0..5 {
            assert!(!analysis.record_access(proc));
        }

        // Il 6° accesso nello stesso secondo deve triggerare l'anomalia
        assert!(analysis.record_access(proc));
    }
}
