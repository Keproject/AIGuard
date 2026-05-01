use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Locales {
    pub quit: String,
    pub secure: String,
    pub warning: String,
    pub alert: String,
    pub protected: String,
    pub started: String,
    pub monitoring: String,
    pub ai_detected: String,
    pub file_event: String,
    pub exit_msg: String,
}

impl Locales {
    pub fn load(lang: &str) -> Self {
        let content = match lang {
            "en" => include_str!("locales/en.toml"),
            "de" => include_str!("locales/de.toml"),
            "it" => include_str!("locales/it.toml"),
            "fr" => include_str!("locales/fr.toml"),
            "es" => include_str!("locales/es.toml"),
            _ => include_str!("locales/en.toml"),
        };
        toml::from_str(content).unwrap_or_else(|_| Self::default_en())
    }

    fn default_en() -> Self {
        Self {
            quit: "Exit".to_string(),
            secure: "AIGuard - Secure".to_string(),
            warning: "AIGuard - Warning: Sensitive files detected".to_string(),
            alert: "AIGuard - ALERT: AI Libraries detected".to_string(),
            protected: "AIGuard - Protected".to_string(),
            started: "AIGuard Core started...".to_string(),
            monitoring: "Monitoring started on: ".to_string(),
            ai_detected: "WARNING: AI processes detected: ".to_string(),
            file_event: "Sensitive file event detected: ".to_string(),
            exit_msg: "Exiting...".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all() {
        assert_eq!(Locales::load("it").quit, "Esci");
        assert_eq!(Locales::load("en").quit, "Exit");
        assert_eq!(Locales::load("de").quit, "Beenden");
        assert_eq!(Locales::load("fr").quit, "Quitter");
        assert_eq!(Locales::load("es").quit, "Salir");
    }
}
