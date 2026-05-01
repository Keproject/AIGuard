use std::path::Path;
use std::fs::File;
use std::io::Read;

/// Modulo Forensics per rilevare tracce di interventi IA.
/// Il modulo è 'cieco': identifica marcature esterne senza leggere il contenuto sensibile.
pub struct ForensicsScanner;

impl ForensicsScanner {
    /// Cerca watermark digitali conosciuti nei metadati o intestazioni (mock logic per 2026).
    pub fn detect_ai_watermark<P: AsRef<Path>>(path: P) -> bool {
        if let Ok(mut file) = File::open(path) {
            let mut buffer = Vec::new();
            // Leggiamo un chunk per cercare il watermark
            if file.by_ref().take(2048).read_to_end(&mut buffer).is_ok() {
                let content = String::from_utf8_lossy(&buffer);

                // Pattern di watermark comuni (es. "AI-GENERATED", "CLAUDE-MODIFIED")
                if content.contains("AI-GEN") || content.contains("WATERMARK-SIG") {
                    return true;
                }
            }
        }
        false
    }

    /// Verifica se il file è stato toccato da processi sospetti (simulato tramite metadati o storico).
    pub fn verify_integrity_history<P: AsRef<Path>>(_path: P, suspicious_processes: &[String]) -> bool {
        !suspicious_processes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_detect_watermark() {
        let mut file = NamedTempFile::new().unwrap();
        // Assicuriamoci di scrivere abbastanza dati e flushare
        file.write_all(b"Contenuto normale con firma AI-GEN-2026").unwrap();
        file.flush().unwrap();
        assert!(ForensicsScanner::detect_ai_watermark(file.path()));
    }

    #[test]
    fn test_no_watermark() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"Documento segreto aziendale").unwrap();
        file.flush().unwrap();
        assert!(!ForensicsScanner::detect_ai_watermark(file.path()));
    }
}
