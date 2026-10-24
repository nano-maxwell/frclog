import "./App.css";
import {useState} from "react";

interface SignalGroupProps {
    name: string;
    signals: string[];
    onSelect: (signal: string) => void;
}

function SignalGroup({ name, signals }: SignalGroupProps) {
    const [isOpen, setIsOpen] = useState(false);

    return <>
        <button className="file-signal-group" aria-expanded={isOpen} onClick={() => setIsOpen(!isOpen)}>{isOpen ? "▾" : "▸"} {name}</button>
        {isOpen && <div className="signal-group-content">
            {signals.map(signal => (
                <p key={signal} className="group-signal">{signal}</p>
            ))}
        </div>}
    </>;
}

function App() {
    const [, setSelectedSignal] =
        useState<string | null>(null);

    return (
        <main className="container">
            <header className="titlebar">
                <h3>frclog</h3>
            </header>
            <div className="app-content">
                <div className="sidebar">
                    <p className="sidebar-title">EXPLORER</p>
                    <p className="sidebar-file">qm12.wpilog</p>
                    <div className="file-content">
                        <SignalGroup
                            name="Elevator"
                            signals={["Position", "Velocity", "Current"]}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                        <SignalGroup
                            name="Swerve"
                            signals={["Speed", "Angle", "Distance"]}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                        <SignalGroup
                            name="Battery"
                            signals={["Voltage", "Current"]}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                    </div>
                </div>
                <div className="workspace">
                    workspace
                </div>
            </div>
        </main>
    );
}

export default App;
