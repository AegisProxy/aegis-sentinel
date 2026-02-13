import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import RiskMeter from "./RiskMeter";
import SecurityEvents from "./SecurityEvents";

interface SecurityEvent {
  event_type: string;
  severity: string;
  message: string;
  timestamp: number;
}

function App() {
  const [events, setEvents] = useState<SecurityEvent[]>([]);

  useEffect(() => {
    // Listen for security events from the Rust backend
    const unlisten = listen<SecurityEvent>("security-event", (event) => {
      console.log("Security event received:", event.payload);
      setEvents((prev) => [...prev, event.payload]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <main className="container">
      <header className="app-header">
        <h1>🛡️ Aegis Sentinel</h1>
        <p className="subtitle">Desktop Security Guardian</p>
      </header>

      <div className="content">
        <RiskMeter events={events} />
        <SecurityEvents events={events} />
      </div>

      <footer className="app-footer">
        <p>
          Monitoring clipboard for sensitive data patterns • Protecting your
          system from data leaks
        </p>
      </footer>
    </main>
  );
}

export default App;
