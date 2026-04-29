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

            // Controlla se il nome del processo contiene onnx o torch
            if name.contains("onnxruntime") || name.contains("torch") {
                detected.push(name);
                continue;
            }

            for arg in process.cmd() {
                let arg_low = arg.to_string_lossy().to_lowercase();
                if arg_low.contains("onnxruntime") || arg_low.contains("torch") {
                    detected.push(process.name().to_string_lossy().to_string());
                    break;
                }
            }
        }

        detected.sort();
        detected.dedup();
        detected
    }
}
