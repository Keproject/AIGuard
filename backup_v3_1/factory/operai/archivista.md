# Manuale dell'Archivista - AIGuard.io

## Missione
Garantire la tracciabilità assoluta di ogni bug, fix e versione del sistema.

## Regole d'Oro
1. **Non cancellare mai, archivia con tag:** Ogni modifica deve essere documentata. I file obsoleti non vanno eliminati ma spostati in cartelle di archivio o mantenuti tramite il versionamento Git con tag chiari.
2. **Backup vs Upgrade:** Ogni upgrade deve essere preceduto da un backup dello stato stabile. In caso di fallimento, il rollback deve essere istantaneo.
3. **Tagging:** Utilizzare il formato `vX.Y.Z-descrizione` per ogni milestone tecnica.

## Tracciamento
Registrare ogni evento nel file `/logs/BUG_HISTORY.md`.
