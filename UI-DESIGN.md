# UI Screenshots and Design

## Application Overview

Aegis Sentinel features a modern, gradient-based design with a clean interface for monitoring security events.

### Main Window

**Dimensions**: 900x700px

**Color Scheme**:
- Light mode: Purple gradient (from #667eea to #764ba2)
- Dark mode: Dark blue gradient (from #1a1a2e to #16213e)

### Components Layout

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│              🛡️  Aegis Sentinel                         │
│           Desktop Security Guardian                     │
│                                                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│     ┌───────────────────────────────────────┐         │
│     │   🛡️ Security Risk Meter              │         │
│     │                                       │         │
│     │           ┌─────────┐                │         │
│     │          │    50   │                │         │
│     │          │ Moderate│                │         │
│     │           └─────────┘                │         │
│     │                                       │         │
│     │  Active threats detected: 3           │         │
│     └───────────────────────────────────────┘         │
│                                                         │
│     ┌───────────────────────────────────────┐         │
│     │   📋 Recent Security Events            │         │
│     │                                       │         │
│     │  ┌──────────────────────────────┐    │         │
│     │  │ 🚨 CRITICAL      10:23 AM    │    │         │
│     │  │ AWS credentials detected!    │    │         │
│     │  │ aws key detected             │    │         │
│     │  └──────────────────────────────┘    │         │
│     │                                       │         │
│     │  ┌──────────────────────────────┐    │         │
│     │  │ ⚠️  HIGH         10:22 AM    │    │         │
│     │  │ Potential API key detected!  │    │         │
│     │  │ api key detected             │    │         │
│     │  └──────────────────────────────┘    │         │
│     │                                       │         │
│     │  ┌──────────────────────────────┐    │         │
│     │  │ ⚡ MEDIUM        10:20 AM    │    │         │
│     │  │ Potential password detected! │    │         │
│     │  │ password detected            │    │         │
│     │  └──────────────────────────────┘    │         │
│     └───────────────────────────────────────┘         │
│                                                         │
├─────────────────────────────────────────────────────────┤
│  Monitoring clipboard for sensitive data patterns •    │
│  Protecting your system from data leaks                │
└─────────────────────────────────────────────────────────┘
```

## Risk Meter Colors

The risk meter uses an SVG circular gauge with dynamic colors:

- **0-20**: Green (#4ade80) - Low Risk
- **20-50**: Yellow (#fbbf24) - Moderate Risk  
- **50-80**: Orange (#fb923c) - High Risk
- **80-100**: Red (#ef4444) - Critical Risk

The gauge fills clockwise from the top, with smooth transitions as the risk level changes.

## Security Event Cards

Each security event is displayed in a card with:

1. **Left Border**: Color-coded by severity
   - Critical: Red (#ef4444)
   - High: Orange (#fb923c)
   - Medium: Yellow (#fbbf24)
   - Low: Green (#4ade80)

2. **Header Row**:
   - Icon (emoji): Severity indicator
   - Severity text: Bold, uppercase
   - Timestamp: Right-aligned, gray text

3. **Message**: Main alert message in bold

4. **Event Type**: Capitalized, gray text

## Responsive Design

The application adapts to the window size and system color scheme:

- **Light Mode**: White cards on gradient background
- **Dark Mode**: Dark gray cards with adjusted text colors
- Cards use subtle shadows and hover effects
- Text is optimized for readability in both modes

## User Experience Features

1. **Real-time Updates**: Events appear immediately when threats are detected
2. **Smooth Animations**: Risk meter gauge animates when levels change
3. **Clear Visual Hierarchy**: Important information is prominent
4. **Accessibility**: High contrast ratios, clear icons, readable fonts
5. **No Sensitive Data Display**: Only patterns are shown, never actual clipboard content

## System Notifications

When a threat is detected (Windows/macOS):

```
┌─────────────────────────────┐
│  🔒 Security Warning         │
│                             │
│  Potential API key          │
│  detected in clipboard!     │
│                             │
│  Aegis Sentinel             │
└─────────────────────────────┘
```

Notifications appear in the system notification area and automatically dismiss after a few seconds.

## Empty State

When no events have been detected:

```
┌───────────────────────────────────┐
│  📋 Recent Security Events         │
│                                   │
│  No security events detected yet. │
│                                   │
│  Try copying text containing      │
│  "api_key" or "password" to test  │
│  the clipboard monitor.           │
│                                   │
└───────────────────────────────────┘
```

This helps users understand how to test the application.

## Technology Stack

- **Frontend Framework**: React 19.1 with TypeScript
- **Styling**: CSS with CSS Variables for theming
- **Icons**: Unicode emoji for cross-platform consistency
- **Graphics**: SVG for the risk meter gauge
- **Build Tool**: Vite for fast development and optimized builds
- **Desktop Framework**: Tauri 2.0 for native desktop integration
