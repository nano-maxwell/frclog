import "./App.css";
import {useState} from "react";
import SignalGroup from "./components/SignalGroup.tsx";

const signalGroups = [
    {
        name: "Elevator",
        signals: ["Position", "Velocity", "Current"]
    },
    {
        name: "Swerve",
        signals: ["Speed", "Angle", "Distance"]
    },
    {
        name: "Flywheel",
        signals: ["Velocity", "Target", "Current"]
    },
    {
        name: "Battery",
        signals: ["Voltage", "Current"]
    }
];

function App() {
    const [selectedSignal, setSelectedSignal] =
        useState<string | null>(null);
    const [searchQuery, setSearchQuery] = useState("");

    const query = searchQuery.trim().toLowerCase();

    const filteredGroups =
        signalGroups
            .filter(
                group => group.name.toLowerCase().includes(query)
                    || group.signals.some(signal => signal.toLowerCase().includes(query)))
            .map(
                group => ({
                    ...group,
                    signals: group.name.toLowerCase().includes(query)
                        ? group.signals
                        : group.signals.filter(signal => signal.toLowerCase().includes(query))
                }));

    return (
        <main className="container">
            <header className="titlebar" data-tauri-drag-region>
                <div className="searchbar" data-tauri-drag-region>
                    <input
                        type="text"
                        placeholder="⌘ K to search..."
                        aria-label="Search signals"
                        onChange={e => setSearchQuery(e.target.value)}
                        value={searchQuery}
                    />
                </div>
            </header>
            <div className="app-content">
                <div className="sidebar">
                    <p className="section-title">EXPLORER</p>
                    <p className="sidebar-file">qm12.wpilog</p>
                    <div className="file-content">
                        {filteredGroups.map(group => (
                            <SignalGroup
                                key={group.name}
                                name={group.name}
                                signals={group.signals}
                                selectedSignal={selectedSignal}
                                onSelect={setSelectedSignal}
                                isSearching={query.length > 0}
                            />))}
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
