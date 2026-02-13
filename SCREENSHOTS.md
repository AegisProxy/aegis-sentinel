# Application Screenshots

Since this is a desktop application that requires a display environment, here are mockup representations of what the application looks like when running.

## Main Application Window

The application features a modern interface with a purple gradient background (or dark blue in dark mode).

### Initial State (No Events)

```
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                        🛡️  Aegis Sentinel                            ║
║                     Desktop Security Guardian                         ║
║                                                                       ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║   ┌─────────────────────────────────────────────────────────────┐   ║
║   │           🛡️ Security Risk Meter                            │   ║
║   │                                                             │   ║
║   │                     ◯                                       │   ║
║   │                    ╱ ╲                                      │   ║
║   │                   │ 0 │  ← Green circular gauge             │   ║
║   │                    ╲ ╱                                      │   ║
║   │                   Low                                       │   ║
║   │                                                             │   ║
║   │         Active threats detected: 0                          │   ║
║   └─────────────────────────────────────────────────────────────┘   ║
║                                                                       ║
║   ┌─────────────────────────────────────────────────────────────┐   ║
║   │           📋 Recent Security Events                          │   ║
║   │                                                             │   ║
║   │         No security events detected yet.                     │   ║
║   │                                                             │   ║
║   │         Try copying text containing                          │   ║
║   │         "api_key" or "password" to test                      │   ║
║   │         the clipboard monitor.                               │   ║
║   │                                                             │   ║
║   └─────────────────────────────────────────────────────────────┘   ║
║                                                                       ║
╠═══════════════════════════════════════════════════════════════════════╣
║  Monitoring clipboard for sensitive data patterns •                  ║
║  Protecting your system from data leaks                              ║
╚═══════════════════════════════════════════════════════════════════════╝
```

### Active State (With Detected Threats)

```
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                        🛡️  Aegis Sentinel                            ║
║                     Desktop Security Guardian                         ║
║                                                                       ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║   ┌─────────────────────────────────────────────────────────────┐   ║
║   │           🛡️ Security Risk Meter                            │   ║
║   │                                                             │   ║
║   │                     ◯                                       │   ║
║   │                   ▓▓╱ ╲                                     │   ║
║   │                 ▓▓│ 65 │  ← Orange circular gauge (65%)     │   ║
║   │                   ▓▓╲ ╱                                     │   ║
║   │                   High                                      │   ║
║   │                                                             │   ║
║   │         Active threats detected: 3                          │   ║
║   └─────────────────────────────────────────────────────────────┘   ║
║                                                                       ║
║   ┌─────────────────────────────────────────────────────────────┐   ║
║   │           📋 Recent Security Events                          │   ║
║   │                                                             │   ║
║   │   ┌──────────────────────────────────────────────────┐     │   ║
║   │ ║ │ 🚨 CRITICAL                       2:34 PM      │     │   ║
║   │ ║ │ AWS credentials detected in clipboard!         │     │   ║
║   │ ║ │ aws key detected                                │     │   ║
║   │   └──────────────────────────────────────────────────┘     │   ║
║   │                                                             │   ║
║   │   ┌──────────────────────────────────────────────────┐     │   ║
║   │ ║ │ ⚠️  HIGH                          2:33 PM      │     │   ║
║   │ ║ │ Potential API key detected in clipboard!       │     │   ║
║   │ ║ │ api key detected                                │     │   ║
║   │   └──────────────────────────────────────────────────┘     │   ║
║   │                                                             │   ║
║   │   ┌──────────────────────────────────────────────────┐     │   ║
║   │ ║ │ ⚡ MEDIUM                         2:30 PM      │     │   ║
║   │ ║ │ Potential password detected in clipboard!      │     │   ║
║   │ ║ │ password detected                               │     │   ║
║   │   └──────────────────────────────────────────────────┘     │   ║
║   │                                                             │   ║
║   └─────────────────────────────────────────────────────────────┘   ║
║                                                                       ║
╠═══════════════════════════════════════════════════════════════════════╣
║  Monitoring clipboard for sensitive data patterns •                  ║
║  Protecting your system from data leaks                              ║
╚═══════════════════════════════════════════════════════════════════════╝
```

## System Notification (Windows/macOS)

When a threat is detected, a native system notification appears:

```
┌────────────────────────────────────┐
│  🔒 Security Warning                │
│                                    │
│  Potential API key detected        │
│  in clipboard!                     │
│                                    │
│  Aegis Sentinel                    │
└────────────────────────────────────┘
```

## Color Scheme

### Light Mode
- Background: Purple gradient (#667eea → #764ba2)
- Cards: White with subtle shadows
- Text: Dark gray (#1f2937)

### Dark Mode  
- Background: Dark blue gradient (#1a1a2e → #16213e)
- Cards: Dark gray (#1f2937)
- Text: Light gray (#f3f4f6)

## Risk Meter Color Coding

The circular gauge changes color based on risk level:

- **0-20**: 🟢 Green (#4ade80) - Low Risk
- **20-50**: 🟡 Yellow (#fbbf24) - Moderate Risk
- **50-80**: 🟠 Orange (#fb923c) - High Risk
- **80-100**: 🔴 Red (#ef4444) - Critical Risk

## Event Severity Indicators

Each event card has a colored left border:

- 🚨 **Critical** - Red border
- ⚠️  **High** - Orange border
- ⚡ **Medium** - Yellow border
- ℹ️  **Low** - Green border

## Window Properties

- **Size**: 900px × 700px
- **Resizable**: Yes
- **Title**: Aegis Sentinel
- **Icon**: Shield icon (included in src-tauri/icons/)

## Real-Time Updates

The UI updates in real-time as events are detected:

1. Event appears in the Security Events list (newest first)
2. Risk Meter gauge animates to new level
3. Threat counter increments
4. System notification appears (Windows/macOS)

## Responsive Design

The application adapts to:
- Window resizing
- System color scheme (light/dark)
- Different screen densities
- Various monitor sizes

## Testing the UI

To see the application in action:

1. Run `npm run tauri dev`
2. Copy test strings from `test-clipboard.sh` or `test-clipboard.bat`
3. Watch events appear in real-time
4. Observe risk meter changes
5. Check system notifications

## Technologies Used

- **SVG**: For the circular risk meter gauge
- **CSS Gradients**: For the background
- **Flexbox**: For responsive layout
- **CSS Variables**: For theming
- **React Hooks**: For state management
- **Tauri Events**: For backend communication
