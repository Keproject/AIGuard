use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, event::Event};
use std::path::Path;
use std::sync::mpsc::Sender;

/// Modulo per il monitoraggio del file system.
/// Monitora la creazione e la modifica di file con estensioni specifiche:
/// .pdf, .docx, .pem, .kdbx.
pub struct FileWatcher {
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    /// Crea una nuova istanza di FileWatcher.
    /// Invia eventi tramite il trasmettitore fornito quando vengono rilevati file di interesse.
    pub fn new(tx: Sender<Event>) -> notify::Result<Self> {
        let watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    tx.send(event).unwrap();
                }
            },
            Config::default(),
        )?;

        Ok(Self { watcher })
    }

    /// Inizia a monitorare una directory specifica.
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)
    }

    /// Controlla se un evento riguarda uno dei tipi di file che ci interessano.
    pub fn is_interesting_event(event: &Event) -> bool {
        for path in &event.paths {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                match ext_str.as_str() {
                    "pdf" | "docx" | "pem" | "kdbx" => return true,
                    _ => {}
                }
            }
        }
        false
    }
}
