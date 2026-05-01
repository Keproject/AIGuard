use std::sync::mpsc::channel;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use aiguard_windows_core::watcher::FileWatcher;

// Per rendere il modulo accessibile ai test di integrazione,
// dobbiamo assicurarci che sia pubblico nel crate, ma per semplicità
// qui simuliamo la logica o assumiamo la visibilità.

#[test]
fn test_attacco_a_raffica_reazione_rapida() {
    let dir = tempdir().unwrap();
    let (tx, rx) = channel();
    let mut watcher = FileWatcher::new(tx).unwrap();
    watcher.watch(dir.path()).unwrap();

    let start = Instant::now();

    // Simuliamo un "Attacco a Raffica": creazione rapida di 10 file sensibili
    for i in 0..10 {
        let file_path = dir.path().join(format!("attacco_{}.pdf", i));
        let mut f = File::create(file_path).unwrap();
        f.write_all(b"attacco").unwrap();
    }

    // Verifichiamo la reazione
    let mut detected = false;
    // Timeout di 50ms come richiesto
    while start.elapsed() < Duration::from_millis(50) {
        if let Ok(event) = rx.try_recv() {
            if FileWatcher::is_interesting_event(&event) {
                detected = true;
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Tempo di reazione: {:?}", elapsed);

    assert!(detected, "Il sistema non ha rilevato l'attacco a raffica in tempo");
    assert!(elapsed < Duration::from_millis(50), "Reazione troppo lenta: {:?}", elapsed);
}
