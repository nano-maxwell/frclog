import {useState} from "react";

interface SignalGroupProps {
    name: string;
    signals: string[];
    selectedSignal: string | null;
    onSelect: (signal: string) => void;
}

function SignalGroup({name, signals, selectedSignal, onSelect}: SignalGroupProps) {
    const [isOpen, setIsOpen] = useState(false);

    return <>
        <button className="file-signal-group" aria-expanded={isOpen}
                onClick={() => setIsOpen(!isOpen)}>{isOpen ? "▾" : "▸"} {name}</button>
        {isOpen && <div className="signal-group-content">
            {signals.map(signal => (
                <button key={signal} className={selectedSignal === `${name}/${signal}` ? "group-signal selected" :"group-signal"}
                        onClick={() => onSelect(`${name}/${signal}`)}>{signal}</button>
            ))}
        </div>}
    </>;
}

export default SignalGroup;