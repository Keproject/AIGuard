use aiguard_windows_core::watcher::FileWatcher;
use aiguard_windows_core::process_scanner::ProcessScanner;
use aiguard_windows_core::tray::{TrayManager, GuardStatus};
use aiguard_windows_core::i18n::Locales;
use aiguard_windows_core::behavioral_analysis::BehavioralAnalysis;
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::time;
use winit::event_loop::{ControlFlow, EventLoop};

#[tokio::main]
async fn main() {
    env_logger::init();

    let lang = std::env::var("LANG").unwrap_or_else(|_| "en".to_string());
    let lang_code = if lang.starts_with("it") { "it" }
                   else if lang.starts_with("de") { "de" }
                   else if lang.starts_with("fr") { "fr" }
                   else if lang.starts_with("es") { "es" }
                   else { "en" };
    let locales = Locales::load(lang_code);

    println!("{}", locales.started);

    let (tx, rx) = channel();
    let mut fw = FileWatcher::new(tx).expect("Errore nell'inizializzazione del FileWatcher");

    if let Ok(current_dir) = std::env::current_dir() {
        fw.watch(&current_dir).expect("Errore nel monitoraggio della cartella corrente");
        println!("{} {:?}", locales.monitoring, current_dir);
    }

    let mut scanner = ProcessScanner::new();
    let mut behavioral = BehavioralAnalysis::new();
    let mut tray_manager = TrayManager::new(locales.clone());
    tray_manager.setup();

    let event_loop = EventLoop::new().unwrap();
    let (status_tx, mut status_rx) = tokio::sync::mpsc::channel::<GuardStatus>(10);

    let locales_task = locales.clone();
    tokio::spawn(async move {
        let mut current_status = GuardStatus::Verde;

        loop {
            let detected_ai = scanner.scan_for_ai_libraries();
            let mut new_status = GuardStatus::Verde;

            // Logica Comportamentale su processi IA rilevati
            let mut behavioral_anomaly = false;
            for proc in &detected_ai {
                if behavioral.record_access(proc) {
                    behavioral_anomaly = true;
                    break;
                }
            }

            if !detected_ai.is_empty() {
                // Se rilevato ma non anomalo -> Giallo
                // Se anomalo (troppi accessi) -> Rosso
                new_status = if behavioral_anomaly { GuardStatus::Rosso } else { GuardStatus::Giallo };
            } else {
                let mut found_file_event = false;
                while let Ok(event) = rx.try_recv() {
                    if FileWatcher::is_interesting_event(&event) {
                        found_file_event = true;
                        println!("{} {:?}", locales_task.file_event, event);
                        break;
                    }
                }

                if found_file_event {
                    new_status = GuardStatus::Giallo;
                }
            }

            if std::mem::discriminant(&new_status) != std::mem::discriminant(&current_status) {
                current_status = new_status;
                let _ = status_tx.send(new_status).await;
            }

            time::sleep(Duration::from_millis(50)).await;
        }
    });

    let locales_loop = locales.clone();
    let _ = event_loop.run(move |_event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + Duration::from_millis(10),
        ));

        while let Ok(new_status) = status_rx.try_recv() {
            tray_manager.update_status(new_status);
        }

        if tray_manager.check_quit_event() {
            println!("{}", locales_loop.exit_msg);
            event_loop_window_target.exit();
        }
    });
}
