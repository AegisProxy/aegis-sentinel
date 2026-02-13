# Aegis Sentinel

Native desktop guardian for Windows and macOS. Monitors system clipboard and local file uploads for AI data leaks.

![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)
![React](https://img.shields.io/badge/React-19.1-blue)
![Rust](https://img.shields.io/badge/Rust-1.93-orange)

## Features

- 🔍 **Real-time Clipboard Monitoring**: Automatically detects sensitive data patterns in clipboard content
- 🚨 **Security Alerts**: Instant notifications when API keys, passwords, or credentials are detected
- 📊 **Risk Meter**: Visual dashboard showing real-time security risk level
- 🛡️ **Pattern Detection**: Recognizes multiple sensitive data patterns including:
  - API keys and access tokens
  - AWS credentials
  - GitHub tokens
  - Password patterns
- 🎨 **Modern UI**: Clean, responsive React interface with dark mode support
- 🖥️ **Cross-platform**: Built with Tauri for Windows and macOS support

## Architecture

### Backend (Rust)
- **Clipboard Monitoring**: Continuous monitoring using the `clipboard` crate
- **Pattern Matching**: Regex-based detection of sensitive data using `regex` and `lazy_static`
- **Event System**: Real-time communication with frontend via Tauri's event system
- **Notifications**: Native OS notifications via `tauri-plugin-notification`

### Frontend (React + TypeScript)
- **Risk Meter Component**: Dynamic visualization of security risk level
- **Security Events Component**: Real-time display of detected security threats
- **IPC Communication**: Listens to events from Rust backend
- **Responsive Design**: Adapts to different screen sizes and color schemes

## Getting Started

### Prerequisites

- Node.js (v20+)
- npm or yarn
- Rust (v1.70+)
- System dependencies for Tauri:
  - **Windows**: Visual Studio C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: See [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/AegisProxy/aegis-sentinel.git
cd aegis-sentinel
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run tauri dev
```

### Building

To create a production build:

```bash
npm run tauri build
```

This will create installer packages in `src-tauri/target/release/bundle/`.

## Usage

1. Launch the application
2. The clipboard monitor starts automatically
3. Copy any text containing sensitive patterns (e.g., "api_key: abc123...")
4. Receive instant security warnings via:
   - In-app event log
   - System notifications
   - Risk meter updates

### Testing the Monitor

Try copying these examples to trigger alerts:

```
api_key: sk_test_51234567890abcdefghijklmnop
password: MySecurePassword123
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

## Security Patterns Detected

1. **API Keys**: Generic API key patterns with 20+ character tokens
2. **AWS Credentials**: AWS access keys and secret keys
3. **GitHub Tokens**: Personal access tokens (ghp_*)
4. **Passwords**: Generic password field patterns

## Development

### Project Structure

```
aegis-sentinel/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── RiskMeter.tsx      # Risk visualization component
│   └── SecurityEvents.tsx # Event display component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── lib.rs         # Tauri setup and commands
│   │   └── clipboard_monitor.rs  # Clipboard monitoring logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node.js dependencies
└── README.md
```

### Adding New Pattern Detectors

Edit `src-tauri/src/clipboard_monitor.rs` and add new patterns to the lazy_static block:

```rust
lazy_static! {
    static ref YOUR_PATTERN: Regex = Regex::new(
        r"your_regex_pattern_here"
    ).unwrap();
}
```

Then add detection logic in `check_security_patterns()` function.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the terms of the LICENSE file in the repository.

## Disclaimer

This tool is designed to help prevent accidental data leaks. It should not be considered a complete security solution. Always follow security best practices when handling sensitive data.
