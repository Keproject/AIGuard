use sysinfo::System;

/// Modulo per la scansione dei processi di sistema.
/// Rileva la presenza di librerie o processi legati a 'onnxruntime' e 'torch'.
pub struct ProcessScanner {
    sys: System,
}

impl ProcessScanner {
    /// Crea una nuova istanza di ProcessScanner.
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    /// Aggiorna le informazioni sul sistema e controlla se i processi target sono in esecuzione.
    /// Restituisce una lista di nomi di processi che corrispondono ai criteri.
    pub fn scan_for_ai_libraries(&mut self) -> Vec<String> {
        self.sys.refresh_all();
        let mut detected = Vec::new();

        for process in self.sys.processes().values() {
            let name = process.name().to_string_lossy().to_lowercase();

            if Self::is_ai_related(&name) {
                detected.push(name);
                continue;
            }

            for arg in process.cmd() {
                let arg_low = arg.to_string_lossy().to_lowercase();
                if Self::is_ai_related(&arg_low) {
                    detected.push(process.name().to_string_lossy().to_string());
                    break;
                }
            }
        }

        detected.sort();
        detected.dedup();
        detected
    }

    /// Funzione interna per determinare se una stringa è legata all'IA.
    fn is_ai_related(s: &str) -> bool {
        let low = s.to_lowercase();
        low.contains("onnxruntime") || low.contains("torch")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ai_related_positive() {
        assert!(ProcessScanner::is_ai_related("libonnxruntime.so"));
        assert!(ProcessScanner::is_ai_related("python_torch_executor"));
        assert!(ProcessScanner::is_ai_related("TORCH_MODEL_SERVER"));
    }

    #[test]
    fn test_is_ai_related_negative() {
        assert!(!ProcessScanner::is_ai_related("chrome.exe"));
        assert!(!ProcessScanner::is_ai_related("system_idle_process"));
    }
}
