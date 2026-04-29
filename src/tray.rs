use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIcon, TrayIconBuilder, Icon,
    menu::MenuEvent,
};
use crate::i18n::Locales;

/// Stati possibili per l'applicazione, che determinano il colore dell'icona.
#[derive(Clone, Copy)]
pub enum GuardStatus {
    Verde,   // Tutto sicuro
    Giallo,  // Attività sospetta (file monitorati toccati)
    Rosso,   // Allarme (Librerie AI rilevate)
}

pub struct TrayManager {
    tray_icon: Option<TrayIcon>,
    quit_id: String,
    locales: Locales,
}

impl TrayManager {
    pub fn new(locales: Locales) -> Self {
        Self {
            tray_icon: None,
            quit_id: String::new(),
            locales,
        }
    }

    /// Inizializza la system tray con l'icona iniziale (Verde).
    pub fn setup(&mut self) {
        let icon = Self::create_icon_stateless(GuardStatus::Verde);

        let menu = Menu::new();
        let quit_item = MenuItem::new(&self.locales.quit, true, None);
        self.quit_id = quit_item.id().0.clone();
        menu.append(&quit_item).unwrap();

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip(&self.locales.protected)
            .with_icon(icon)
            .build()
            .unwrap();

        self.tray_icon = Some(tray_icon);
    }

    /// Aggiorna l'icona della tray in base allo stato.
    pub fn update_status(&mut self, status: GuardStatus) {
        let icon = Self::create_icon_stateless(status);
        if let Some(ref mut tray) = self.tray_icon {
            let _ = tray.set_icon(Some(icon));

            match status {
                GuardStatus::Verde => {
                    let _ = tray.set_tooltip(Some(&self.locales.secure));
                },
                GuardStatus::Giallo => {
                    let _ = tray.set_tooltip(Some(&self.locales.warning));
                },
                GuardStatus::Rosso => {
                    let _ = tray.set_tooltip(Some(&self.locales.alert));
                }
            }
        }
    }

    /// Controlla se è stato cliccato il tasto Esci.
    pub fn check_quit_event(&self) -> bool {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            return event.id.0 == self.quit_id;
        }
        false
    }

    /// Crea un'icona colorata programmatica 32x32.
    fn create_icon_stateless(status: GuardStatus) -> Icon {
        let color = match status {
            GuardStatus::Verde => [0, 255, 0, 255],
            GuardStatus::Giallo => [255, 255, 0, 255],
            GuardStatus::Rosso => [255, 0, 0, 255],
        };

        let mut rgba = Vec::with_capacity(32 * 32 * 4);
        for _ in 0..(32 * 32) {
            rgba.extend_from_slice(&color);
        }

        Icon::from_rgba(rgba, 32, 32).unwrap()
    }
}
