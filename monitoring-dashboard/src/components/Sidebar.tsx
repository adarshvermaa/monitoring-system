import { NavLink } from 'react-router-dom';
import {
  LayoutDashboard,
  FileText,
  Activity,
  Network,
  Server,
  Settings,
  BarChart3,
} from 'lucide-react';
import './Sidebar.css';

const navItems = [
  { to: '/dashboard', icon: LayoutDashboard, label: 'Overview' },
  { to: '/dashboard/logs', icon: FileText, label: 'Logs' },
  { to: '/dashboard/metrics', icon: BarChart3, label: 'Metrics' },
  { to: '/dashboard/traffic', icon: Network, label: 'Traffic' },
  { to: '/dashboard/agents', icon: Server, label: 'Agents' },
  { to: '/dashboard/settings', icon: Settings, label: 'Settings' },
];

export const Sidebar = () => {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <Activity className="sidebar-logo" size={32} />
        <h2>Monitoring</h2>
      </div>

      <nav className="sidebar-nav">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              `nav-item ${isActive ? 'active' : ''}`
            }
            end={item.to === '/dashboard'}
          >
            <item.icon size={20} />
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>

      <div className="sidebar-footer">
        <div className="status-indicator">
          <div className="status-dot online"></div>
          <span>System Online</span>
        </div>
      </div>
    </aside>
  );
};
