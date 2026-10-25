import "./App.css";
import {useState} from "react";
import SignalGroup from "./components/SignalGroup.tsx";

function App() {
    const [selectedSignal, setSelectedSignal] =
        useState<string | null>(null);

    return (
        <main className="container">
            <header className="titlebar">
                <h3>frclog</h3>
            </header>
            <div className="app-content">
                <div className="sidebar">
                    <p className="section-title">EXPLORER</p>
                    <p className="sidebar-file">qm12.wpilog</p>
                    <div className="file-content">
                        <SignalGroup
                            name="Elevator"
                            signals={["Position", "Velocity", "Current"]}
                            selectedSignal={selectedSignal}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                        <SignalGroup
                            name="Swerve"
                            signals={["Speed", "Angle", "Distance"]}
                            selectedSignal={selectedSignal}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                        <SignalGroup
                            name="Battery"
                            signals={["Voltage", "Current"]}
                            selectedSignal={selectedSignal}
                            onSelect={signal => setSelectedSignal(signal)}
                        />
                    </div>
                </div>
                <div className="workspace">
                    <p className="section-title">INSPECTOR</p>
                    <p> {selectedSignal
                        ? selectedSignal
                        : "Select a signal to inspect"} </p>
                </div>
            </div>
        </main>
    );
}

export default App;
