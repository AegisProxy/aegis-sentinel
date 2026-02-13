# Development Guide

## Quick Start

### Development Mode

To run the application in development mode:

```bash
npm run tauri dev
```

This will:
1. Start the Vite development server for hot-reloading the React frontend
2. Build and launch the Tauri desktop application
3. Open a window showing the application

### Production Build

To create a production build:

```bash
npm run tauri build
```

This creates optimized bundles in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image and `.app` bundle
- **Linux**: `.deb`, `.rpm`, and `.AppImage` packages

## Application Structure

### Clipboard Monitor (Rust)

The clipboard monitoring system runs in the background and:

1. **Polls the clipboard** every 500ms for new content
2. **Checks content** against security patterns using regex
3. **Emits events** to the frontend when threats are detected
4. **Sends notifications** via the OS notification system

**Location**: `src-tauri/src/clipboard_monitor.rs`

**Key Components**:
- `ClipboardMonitor` struct: Manages the monitoring lifecycle
- `SecurityEvent` struct: Represents a detected security threat
- Pattern regex: Defined using `lazy_static` for efficiency

### Frontend Components

#### RiskMeter Component

**Location**: `src/RiskMeter.tsx`

Displays a visual gauge showing the current security risk level:
- **Green (0-20)**: Low risk
- **Yellow (20-50)**: Moderate risk
- **Orange (50-80)**: High risk
- **Red (80-100)**: Critical risk

Risk level is calculated based on:
- Number of events in the last hour
- Severity of each event (critical=30, high=20, medium=10, low=5)

#### SecurityEvents Component

**Location**: `src/SecurityEvents.tsx`

Shows a list of recent security events with:
- Event severity and icon
- Timestamp
- Descriptive message
- Event type

Displays the 10 most recent events in reverse chronological order.

## Security Patterns

The application detects the following patterns:

### 1. Generic API Keys
```regex
(?i)(api[_-]?key|apikey|access[_-]?token|secret[_-]?key|private[_-]?key)[\s:=]+[a-zA-Z0-9_\-]{20,}
```
**Example**: `api_key: sk_test_1234567890abcdefghij`

### 2. AWS Credentials
```regex
(?i)(AKIA[0-9A-Z]{16}|aws_secret_access_key)
```
**Example**: `AKIA1234567890ABCDEF`

### 3. GitHub Tokens
```regex
(?i)(ghp_[a-zA-Z0-9]{36}|github_token)
```
**Example**: `ghp_1234567890abcdefghijklmnopqrstuvwxyz`

### 4. Password Patterns
```regex
(?i)(password|passwd|pwd)[\s:=]+.{6,}
```
**Example**: `password: MySecretPassword123`

## Adding New Patterns

To add a new security pattern:

1. **Define the regex** in `src-tauri/src/clipboard_monitor.rs`:

```rust
lazy_static! {
    static ref NEW_PATTERN: Regex = Regex::new(
        r"your_pattern_here"
    ).unwrap();
}
```

2. **Add detection logic** in `check_security_patterns()`:

```rust
if NEW_PATTERN.is_match(content) {
    return Some(SecurityEvent {
        event_type: "new_pattern_detected".to_string(),
        severity: "high".to_string(),
        message: "New pattern detected in clipboard!".to_string(),
        timestamp,
    });
}
```

3. **Rebuild** the application to test your changes

## Testing

### Manual Testing

1. Launch the app: `npm run tauri dev`
2. Copy test strings to clipboard:
   - `api_key: test_key_12345678901234567890`
   - `password: TestPassword123`
   - `AKIA1234567890ABCDEF`
3. Verify:
   - Event appears in the Security Events list
   - Risk meter updates
   - System notification appears (Windows/macOS)

### Platform-Specific Notes

**Linux**: Clipboard monitoring has limited functionality on Linux due to X11/Wayland differences. The application includes a demo mode that triggers sample events.

**Windows/macOS**: Full clipboard monitoring functionality is available.

## Configuration

### Tauri Configuration

Edit `src-tauri/tauri.conf.json` to configure:
- Window size and title
- Application identifier
- Build settings
- Permissions and capabilities

### Frontend Configuration

Edit `vite.config.ts` to configure:
- Build optimization
- Dev server settings
- Plugin options

## Troubleshooting

### Build Errors

**Missing system libraries (Linux)**:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Rust compilation errors**:
```bash
cd src-tauri
cargo clean
cargo build
```

### Runtime Issues

**Clipboard not working**:
- Check OS permissions for clipboard access
- On Linux, ensure X11/Wayland clipboard support is available

**No notifications**:
- Check OS notification permissions
- Verify notification plugin is enabled in Tauri config

## Performance Considerations

- **Polling Interval**: Currently set to 500ms. Adjust in `clipboard_monitor.rs` if needed
- **Event History**: Frontend stores all events in memory. Consider adding a limit for long-running sessions
- **Regex Performance**: Patterns are compiled once using `lazy_static` for efficiency

## Security Best Practices

1. **Never log sensitive data**: The clipboard content itself is never logged or stored
2. **Pattern matching only**: Only pattern matches trigger events, not the actual content
3. **Local processing**: All detection happens locally; no data is sent to external servers
4. **User privacy**: The application does not track user behavior or collect telemetry

## Contributing

When contributing, please:
1. Test on multiple platforms if possible
2. Add appropriate error handling
3. Update documentation for new features
4. Follow the existing code style
5. Add tests for new patterns or features

## Resources

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Regex Testing Tool](https://regex101.com/)
