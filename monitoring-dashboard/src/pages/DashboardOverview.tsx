import { useEffect, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { 
  TrendingUp, 
  Server, 
  AlertTriangle, 
  Activity,
  ArrowUp,
  ArrowDown 
} from 'lucide-react';
import { Line, Area, AreaChart, LineChart, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { apiService } from '../services/api';
import type { DashboardStats } from '../types';
import './DashboardOverview.css';

const StatCard = ({ title, value, change, icon: Icon, trend }: any) => (
  <div className="stat-card card fade-in">
    <div className="stat-header">
      <div className="stat-icon" style={{ background: `rgba(${trend > 0 ? '16, 185, 129' : '239, 68, 68'}, 0.1)` }}>
        <Icon size={24} style={{ color: trend > 0 ? 'var(--accent)' : 'var(--danger)' }} />
      </div>
      <span className="stat-change" style={{ color: trend > 0 ? 'var(--accent)' : 'var(--danger)' }}>
        {trend > 0 ? <ArrowUp size={16} /> : <ArrowDown size={16} />}
        {change}%
      </span>
    </div>
    <h3 className="stat-title">{title}</h3>
    <p className="stat-value">{value.toLocaleString()}</p>
  </div>
);

export const DashboardOverview = () => {
  const [chartData, setChartData] = useState<any[]>([]);

  const { data: stats } = useQuery({
    queryKey: ['dashboard-stats'],
    queryFn: () => apiService.getDashboardStats(),
    refetchInterval: 5000,
  });

  useEffect(() => {
    // Generate mock chart data
    const generateData = () => {
      const data = [];
      const now = Date.now();
      for (let i = 24; i >= 0; i--) {
        data.push({
          time: new Date(now - i * 60 * 60 * 1000).toLocaleTimeString('en-US', { hour: '2-digit' }),
          events: Math.floor(Math.random() * 200) + 100,
          cpu: Math.random() * 80 + 10,
          memory: Math.random() * 70 + 20,
        });
      }
      return data;
    };

    setChartData(generateData());
    const interval = setInterval(() => {
      setChartData(generateData());
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  if (!stats) {
    return (
      <div className="loading-container">
        <div className="spinner"></div>
        <p>Loading dashboard...</p>
      </div>
    );
  }

  return (
    <div className="dashboard-overview">
      <div className="page-header">
        <h1>Overview</h1>
        <p className="page-description">Real-time monitoring system metrics</p>
      </div>

      <div className="stats-grid">
        <StatCard
          title="Total Events"
          value={stats.total_events}
          change={12.5}
          trend={1}
          icon={Activity}
        />
        <StatCard
          title="Events/Second"
          value={stats.events_per_second}
          change={8.2}
          trend={1}
          icon={TrendingUp}
        />
        <StatCard
          title="Active Agents"
          value={`${stats.online_agents}/${stats.total_agents}`}
          change={-2.1}
          trend={-1}
          icon={Server}
        />
        <StatCard
          title="Errors"
          value={stats.error_count}
          change={15.3}
          trend={-1}
          icon={AlertTriangle}
        />
      </div>

      <div className="charts-container">
        <div className="chart-card card">
          <h3 className="chart-title">Events Over Time</h3>
          <ResponsiveContainer width="100%" height={300}>
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="colorEvents" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="var(--primary)" stopOpacity={0.8}/>
                  <stop offset="95%" stopColor="var(--primary)" stopOpacity={0}/>
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="var(--border)" />
              <XAxis dataKey="time" stroke="var(--text-muted)" />
              <YAxis stroke="var(--text-muted)" />
              <Tooltip
                contentStyle={{
                  background: 'var(--bg-secondary)',
                  border: '1px solid var(--border)',
                  borderRadius: 'var(--radius-md)',
                  color: 'var(--text-primary)'
                }}
              />
              <Area
                type="monotone"
                dataKey="events"
                stroke="var(--primary)"
                fillOpacity={1}
                fill="url(#colorEvents)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="chart-card card">
          <h3 className="chart-title">System Resources</h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="var(--border)" />
              <XAxis dataKey="time" stroke="var(--text-muted)" />
              <YAxis stroke="var(--text-muted)" />
              <Tooltip
                contentStyle={{
                  background: 'var(--bg-secondary)',
                  border: '1px solid var(--border)',
                  borderRadius: 'var(--radius-md)',
                  color: 'var(--text-primary)'
                }}
              />
              <Line
                type="monotone"
                dataKey="cpu"
                stroke="var(--accent)"
                strokeWidth={2}
                dot={false}
              />
              <Line
                type="monotone"
                dataKey="memory"
                stroke="var(--secondary)"
                strokeWidth={2}
                dot={false}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
};
