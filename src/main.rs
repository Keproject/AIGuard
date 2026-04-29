use aiguard_windows_core::watcher::FileWatcher;
use aiguard_windows_core::process_scanner::ProcessScanner;
use aiguard_windows_core::tray::{TrayManager, GuardStatus};
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::time;
use winit::event_loop::{ControlFlow, EventLoop};

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("AIGuard Core avviato...");

    // Canale per gli eventi del file watcher
    let (tx, rx) = channel();
    let mut fw = FileWatcher::new(tx).expect("Errore nell'inizializzazione del FileWatcher");

    // Monitoriamo la directory corrente per demo
    if let Ok(current_dir) = std::env::current_dir() {
        fw.watch(&current_dir).expect("Errore nel monitoraggio della cartella corrente");
        println!("Monitoraggio avviato su: {:?}", current_dir);
    }

    let mut scanner = ProcessScanner::new();
    let mut tray_manager = TrayManager::new();
    tray_manager.setup();

    // Loop principale dell'interfaccia (necessario per tray-icon su Windows/macOS)
    let event_loop = EventLoop::builder().build().unwrap();

    // Task asincrono per monitoraggio e scansione
    let (status_tx, mut status_rx) = tokio::sync::mpsc::channel::<GuardStatus>(10);

    tokio::spawn(async move {
        let mut current_status = GuardStatus::Verde;

        loop {
            let detected_ai = scanner.scan_for_ai_libraries();
            let mut new_status = GuardStatus::Verde;

            if !detected_ai.is_empty() {
                new_status = GuardStatus::Rosso;
            } else {
                // Se non ci sono processi AI, controlliamo se ci sono stati eventi file
                let mut found_file_event = false;
                while let Ok(event) = rx.try_recv() {
                    if FileWatcher::is_interesting_event(&event) {
                        found_file_event = true;
                        println!("Evento file sensibile rilevato: {:?}", event);
                        break;
                    }
                }

                if found_file_event {
                    new_status = GuardStatus::Giallo;
                }
            }

            // Aggiorna lo stato se è cambiato
            if std::mem::discriminant(&new_status) != std::mem::discriminant(&current_status) {
                current_status = new_status;
                let _ = status_tx.send(new_status).await;
            }

            time::sleep(Duration::from_secs(2)).await;
        }
    });

    // Gestione degli eventi dell'event loop
    #[allow(deprecated)]
    let _ = event_loop.run(move |_event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + Duration::from_millis(100),
        ));

        // Ricezione degli aggiornamenti di stato dal task asincrono
        while let Ok(new_status) = status_rx.try_recv() {
            tray_manager.update_status(new_status);
        }

        // Gestione dell'evento di uscita
        if tray_manager.check_quit_event() {
            println!("Uscita in corso...");
            event_loop_window_target.exit();
        }
    });
}
