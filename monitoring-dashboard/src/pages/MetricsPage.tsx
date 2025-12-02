import { useQuery } from '@tanstack/react-query';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { apiService } from '../services/api';

export const MetricsPage = () => {
  const { data: metrics, isLoading } = useQuery({
    queryKey: ['metrics'],
    queryFn: () => apiService.getMetrics(),
    refetchInterval: 5000,
  });

  const chartData = metrics?.map(m => ({
    time: new Date(m.timestamp).toLocaleTimeString(),
    [m.name]: m.value,
  })) || [];
console.log(chartData)
  return (
    <div className="metrics-page">
      <div className="page-header">
        <h1>Metrics</h1>
        <p className="page-description">System performance metrics and trends</p>
      </div>

      {isLoading ? (
        <div className="loading-container">
          <div className="spinner"></div>
        </div>
      ) : (
        <div className="card" style={{ padding: '2rem' }}>
          <h3 style={{ marginBottom: '1.5rem' }}>CPU Usage Over Time</h3>
          <ResponsiveContainer width="100%" height={400}>
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="var(--border)" />
              <XAxis dataKey="time" stroke="var(--text-muted)" />
              <YAxis stroke="var(--text-muted)" />
              <Tooltip
                contentStyle={{
                  background: 'var(--bg-secondary)',
                  border: '1px solid var(--border)',
                  borderRadius: 'var(--radius-md)',
                }}
              />
              <Legend />
              <Line
                type="monotone"
                dataKey="system.cpu.usage"
                stroke="var(--primary)"
                strokeWidth={2}
                dot={false}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      )}
    </div>
  );
};
