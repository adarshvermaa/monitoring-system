import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LogIn, Activity } from 'lucide-react';
import { useAuthStore } from '../store/authStore';
import { apiService } from '../services/api';
import './Login.css';

export const Login = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  const login = useAuthStore((state) => state.login);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      const response = await apiService.login(username, password);
      login(response.token, response.user);
      navigate('/dashboard');
    } catch (err) {
      setError('Invalid credentials. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="login-container">
      <div className="login-card slide-up">
        <div className="login-header">
          <div className="logo">
            <Activity size={40} className="logo-icon" />
            <h1>Monitoring System</h1>
          </div>
          <p className="subtitle">Sign in to access your dashboard</p>
        </div>

        <form onSubmit={handleSubmit} className="login-form">
          {error && (
            <div className="error-message fade-in">
              {error}
            </div>
          )}

          <div className="form-group">
            <label htmlFor="username">Username</label>
            <input
              id="username"
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Enter your username"
              required
              autoFocus
            />
          </div>

          <div className="form-group">
            <label htmlFor="password">Password</label>
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter your password"
              required
            />
          </div>

          <button
            type="submit"
            className="btn btn-primary btn-login"
            disabled={loading}
          >
            {loading ? (
              <>
                <div className="spinner-small"></div>
                Signing in...
              </>
            ) : (
              <>
                <LogIn size={20} />
                Sign In
              </>
            )}
          </button>
        </form>

        <div className="login-footer">
          <p className="demo-hint">
            Demo credentials: <code>admin</code> / <code>password</code>
          </p>
        </div>
      </div>
    </div>
  );
};
