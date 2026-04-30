# Rust Monitoring Engine: Blind-Shield Core

## Deterministic Watcher
- **Scope**: Monitors specific high-value file extensions: `.pdf`, `.docx`, `.pem`, `.kdbx`.
- **Implementation**: Uses the `notify` crate for low-level OS event hooking.
- **Latency**: Sub-10ms event detection.

## AI Process Scanner
- **Target**: Detects runtime ingestion by AI libraries.
- **Signatures**:
  - `onnxruntime` (Generic AI Ingestion)
  - `torch` (PyTorch models)
  - `webnn` (Web-based AI acceleration)
- **Method**: Iterates over active Windows processes and inspects loaded modules/symbols.

## Behavioral & Forensics
- **Anomaly Detection**: Tracks frequency of file access by unauthorized processes.
- **Forensics**: Analyzes file entropy and watermarks to identify AI-generated content or ingestion attempts.
- **Status States**:
  - `Verde` (Green): Monitoring active, no threats.
  - `Giallo` (Yellow): AI process detected or file event observed.
  - `Rosso` (Red): High-risk behavioral anomaly or AI watermark detected.

## System Integration
- **Tray Management**: Native Windows system tray integration with color-coded status icons.
- **IPC**: Uses asynchronous channels (mpsc) for non-blocking communication between the scanner and the UI thread.
