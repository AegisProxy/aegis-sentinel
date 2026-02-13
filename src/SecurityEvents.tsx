import "./SecurityEvents.css";

interface SecurityEvent {
  event_type: string;
  severity: string;
  message: string;
  timestamp: number;
}

interface SecurityEventsProps {
  events: SecurityEvent[];
}

const SecurityEvents: React.FC<SecurityEventsProps> = ({ events }) => {
  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "critical":
        return "#ef4444";
      case "high":
        return "#fb923c";
      case "medium":
        return "#fbbf24";
      default:
        return "#4ade80";
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case "critical":
        return "🚨";
      case "high":
        return "⚠️";
      case "medium":
        return "⚡";
      default:
        return "ℹ️";
    }
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleTimeString();
  };

  // Show only the last 10 events
  const recentEvents = [...events].reverse().slice(0, 10);

  return (
    <div className="security-events">
      <h2>📋 Recent Security Events</h2>
      {recentEvents.length === 0 ? (
        <div className="no-events">
          <p>No security events detected yet.</p>
          <p className="hint">
            Try copying text containing "api_key" or "password" to test the
            clipboard monitor.
          </p>
        </div>
      ) : (
        <div className="events-list">
          {recentEvents.map((event, index) => (
            <div
              key={`${event.timestamp}-${index}`}
              className="event-item"
              style={{ borderLeftColor: getSeverityColor(event.severity) }}
            >
              <div className="event-header">
                <span className="event-icon">
                  {getSeverityIcon(event.severity)}
                </span>
                <span className="event-severity" style={{ color: getSeverityColor(event.severity) }}>
                  {event.severity.toUpperCase()}
                </span>
                <span className="event-time">{formatTimestamp(event.timestamp)}</span>
              </div>
              <div className="event-message">{event.message}</div>
              <div className="event-type">{event.event_type.replace(/_/g, " ")}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default SecurityEvents;
