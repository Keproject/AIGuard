use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, event::Event};
use std::path::Path;
use std::sync::mpsc::Sender;

/// Modulo per il monitoraggio del file system.
/// CERTIFICAZIONE: NO LLM TRAINING ON USER DATA.
/// L'analisi è 100% deterministica basata su estensioni e pattern matching locale.
pub struct FileWatcher {
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    /// Crea una nuova istanza di FileWatcher.
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

    /// Controlla in modo deterministico se un evento riguarda uno dei tipi di file di interesse.
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
    fn test_deterministic_analysis() {
        // Stesso input = Stesso output
        let path = PathBuf::from("test.pdf");
        let event1 = Event::new(EventKind::Modify(ModifyKind::Any)).add_path(path.clone());
        let event2 = Event::new(EventKind::Modify(ModifyKind::Any)).add_path(path);

        assert_eq!(FileWatcher::is_interesting_event(&event1), FileWatcher::is_interesting_event(&event2));
        assert!(FileWatcher::is_interesting_event(&event1));
    }
}
