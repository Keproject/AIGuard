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
                    let _ = tx.send(event);
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

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{Event, EventKind, ModifyKind};
    use std::path::PathBuf;

    #[test]
    fn test_is_interesting_event_pdf() {
        let event = Event::new(EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Content)))
            .add_path(PathBuf::from("test.pdf"));
        assert!(FileWatcher::is_interesting_event(&event));
    }

    #[test]
    fn test_is_interesting_event_kdbx() {
        let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
            .add_path(PathBuf::from("secret.KDBX")); // Case insensitive check
        assert!(FileWatcher::is_interesting_event(&event));
    }

    #[test]
    fn test_is_not_interesting_event_txt() {
        let event = Event::new(EventKind::Modify(ModifyKind::Any))
            .add_path(PathBuf::from("readme.txt"));
        assert!(!FileWatcher::is_interesting_event(&event));
    }
}
