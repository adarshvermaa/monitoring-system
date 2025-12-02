import { Bell, User, LogOut } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/authStore';
import './Header.css';

export const Header = () => {
  const { user, logout } = useAuthStore();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <header className="header">
      <div className="header-left">
        <h1 className="page-title">Monitoring Dashboard</h1>
      </div>

      <div className="header-right">
        <button className="icon-btn" aria-label="Notifications">
          <Bell size={20} />
          <span className="notification-badge">3</span>
        </button>

        <div className="user-menu">
          <div className="user-info">
            <User size={18} />
            <span>{user?.username}</span>
          </div>
          <button onClick={handleLogout} className="logout-btn">
            <LogOut size={18} />
          </button>
        </div>
      </div>
    </header>
  );
};
