import { useState } from 'react';
import IntentConsole from './components/IntentConsole';
import AgentMatrix from './components/AgentMatrix';
import Studio from './components/Studio';
import Planetary from './components/Planetary';

type Tab = 'console' | 'agents' | 'studio' | 'planetary';

const TABS: { id: Tab; label: string }[] = [
  { id: 'console', label: 'Intent Console' },
  { id: 'agents',  label: 'Agents' },
  { id: 'studio',  label: 'Studio' },
  { id: 'planetary', label: 'Planetary' },
];

export default function App() {
  const [active, setActive] = useState<Tab>('console');

  return (
    <div className="app">
      <header className="app-header">
        <span className="app-logo" aria-label="GAIA">
          <svg width="28" height="28" viewBox="0 0 28 28" fill="none" aria-hidden="true">
            <circle cx="14" cy="14" r="12" stroke="currentColor" strokeWidth="2" />
            <path d="M14 6 C14 6 20 10 20 14 C20 18 14 22 14 22 C14 22 8 18 8 14 C8 10 14 6 14 6Z"
                  stroke="currentColor" strokeWidth="1.5" fill="none" />
            <line x1="6" y1="14" x2="22" y2="14" stroke="currentColor" strokeWidth="1.5" />
          </svg>
          GAIA Studio
        </span>
        <nav className="app-nav" role="navigation" aria-label="Main navigation">
          {TABS.map((tab) => (
            <button
              key={tab.id}
              className={`nav-tab${active === tab.id ? ' nav-tab--active' : ''}`}
              onClick={() => setActive(tab.id)}
              aria-current={active === tab.id ? 'page' : undefined}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </header>

      <main className="app-main" id="main-content">
        {active === 'console' && <IntentConsole />}
        {active === 'agents'  && <AgentMatrix />}
        {active === 'studio'  && <Studio />}
      </main>
    </div>
  );
}
