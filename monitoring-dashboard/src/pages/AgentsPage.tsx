import { useQuery } from '@tanstack/react-query';
import { Server, Circle } from 'lucide-react';
import { apiService } from '../services/api';
import { formatDistanceToNow } from 'date-fns';
import './AgentsPage.css';

export const AgentsPage = () => {
  const { data: agents, isLoading } = useQuery({
    queryKey: ['agents'],
    queryFn: () => apiService.getAgents(),
    refetchInterval: 5000,
  });

  return (
    <div className="agents-page">
      <div className="page-header">
        <h1>Agents</h1>
        <p className="page-description">Monitor connected monitoring agents</p>
      </div>

      {isLoading ? (
        <div className="loading-container">
          <div className="spinner"></div>
        </div>
      ) : (
        <div className="agents-grid">
          {agents?.map((agent) => (
            <div key={agent.id} className="agent-card card">
              <div className="agent-header">
                <Server size={24} className="agent-icon" />
                <div className="agent-status">
                  <Circle
                    size={12}
                    fill={agent.status === 'online' ? 'var(--accent)' : 'var(--danger)'}
                    color={agent.status === 'online' ? 'var(--accent)' : 'var(--danger)'}
                  />
                  <span className={agent.status}>{agent.status}</span>
                </div>
              </div>

              <h3 className="agent-hostname">{agent.hostname}</h3>
              <p className="agent-id">{agent.id}</p>

              <div className="agent-tags">
                {agent.tags.map((tag) => (
                  <span key={tag} className="badge badge-info">{tag}</span>
                ))}
              </div>

              <div className="agent-stats">
                <div className="stat-item">
                  <span className="stat-label">Events Sent</span>
                  <span className="stat-value">{agent.events_sent.toLocaleString()}</span>
                </div>
                <div className="stat-item">
                  <span className="stat-label">Last Seen</span>
                  <span className="stat-value">
                    {formatDistanceToNow(agent.last_seen, { addSuffix: true })}
                  </span>
                </div>
              </div>

              {agent.cpu_usage !== undefined && (
                <div className="resource-meters">
                  <div className="meter">
                    <div className="meter-label">
                      <span>CPU</span>
                      <span>{agent.cpu_usage.toFixed(1)}%</span>
                    </div>
                    <div className="meter-bar">
                      <div
                        className="meter-fill cpu"
                        style={{ width: `${agent.cpu_usage}%` }}
                      ></div>
                    </div>
                  </div>
                  <div className="meter">
                    <div className="meter-label">
                      <span>Memory</span>
                      <span>{agent.memory_usage?.toFixed(1)}%</span>
                    </div>
                    <div className="meter-bar">
                      <div
                        className="meter-fill memory"
                        style={{ width: `${agent.memory_usage}%` }}
                      ></div>
                    </div>
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
