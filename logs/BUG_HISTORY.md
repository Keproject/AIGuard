# BUG HISTORY - AIGuard.io

| Data | Versione | Bug Descrizione | Fix / Azione | Status |
|------|----------|-----------------|--------------|--------|
| 2024-10-25 | v0.1.0 | Case-sensitivity nel Process Scanner | Implementato .to_lowercase() | RISOLTO |
| 2024-10-25 | v0.1.1 | Linker error libxdo mancante | Aggiunto libxdo-dev a CI/CD e docs | RISOLTO |
| 2024-10-25 | v0.1.2 | Alto CPU usage (refresh_all) | Sostituito con refresh_processes() | RISOLTO |
