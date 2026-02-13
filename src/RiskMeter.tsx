import { useEffect, useState } from "react";
import "./RiskMeter.css";

interface SecurityEvent {
  event_type: string;
  severity: string;
  message: string;
  timestamp: number;
}

interface RiskMeterProps {
  events: SecurityEvent[];
}

const RiskMeter: React.FC<RiskMeterProps> = ({ events }) => {
  const [riskLevel, setRiskLevel] = useState(0);

  useEffect(() => {
    // Calculate risk level based on recent events
    const now = Date.now() / 1000;
    const recentEvents = events.filter((e) => now - e.timestamp < 3600); // Last hour

    let risk = 0;
    recentEvents.forEach((event) => {
      switch (event.severity) {
        case "critical":
          risk += 30;
          break;
        case "high":
          risk += 20;
          break;
        case "medium":
          risk += 10;
          break;
        default:
          risk += 5;
      }
    });

    // Cap at 100
    setRiskLevel(Math.min(risk, 100));
  }, [events]);

  const getRiskColor = () => {
    if (riskLevel < 20) return "#4ade80"; // Green
    if (riskLevel < 50) return "#fbbf24"; // Yellow
    if (riskLevel < 80) return "#fb923c"; // Orange
    return "#ef4444"; // Red
  };

  const getRiskLabel = () => {
    if (riskLevel < 20) return "Low";
    if (riskLevel < 50) return "Moderate";
    if (riskLevel < 80) return "High";
    return "Critical";
  };

  return (
    <div className="risk-meter">
      <h2>🛡️ Security Risk Meter</h2>
      <div className="meter-container">
        <svg width="200" height="200" viewBox="0 0 200 200">
          {/* Background circle */}
          <circle
            cx="100"
            cy="100"
            r="80"
            fill="none"
            stroke="#e5e7eb"
            strokeWidth="20"
          />
          {/* Risk level circle */}
          <circle
            cx="100"
            cy="100"
            r="80"
            fill="none"
            stroke={getRiskColor()}
            strokeWidth="20"
            strokeDasharray={`${(riskLevel / 100) * 502.65} 502.65`}
            strokeLinecap="round"
            transform="rotate(-90 100 100)"
            style={{ transition: "stroke-dasharray 0.5s ease" }}
          />
          {/* Center text */}
          <text
            x="100"
            y="95"
            textAnchor="middle"
            fontSize="36"
            fontWeight="bold"
            fill={getRiskColor()}
          >
            {riskLevel}
          </text>
          <text
            x="100"
            y="115"
            textAnchor="middle"
            fontSize="14"
            fill="#6b7280"
          >
            {getRiskLabel()}
          </text>
        </svg>
      </div>
      <div className="meter-info">
        <p>
          Active threats detected in the last hour:{" "}
          <strong>{events.filter((e) => Date.now() / 1000 - e.timestamp < 3600).length}</strong>
        </p>
      </div>
    </div>
  );
};

export default RiskMeter;
