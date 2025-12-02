import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { Search, Filter, Download, RefreshCw } from 'lucide-react';
import { apiService } from '../services/api';
import type { LogEvent } from '../types';
import { formatDistanceToNow } from 'date-fns';
import './LogsPage.css';

const LogLevelBadge = ({ level }: { level: LogEvent['level'] }) => {
  const colors = {
    trace: 'badge-info',
    debug: 'badge-info',
    info: 'badge-success',
    warning: 'badge-warning',
    error: 'badge-danger',
    critical: 'badge-danger',
  };

  return <span className={`badge ${colors[level]}`}>{level.toUpperCase()}</span>;
};

export const LogsPage = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [levelFilter, setLevelFilter] = useState<string>('all');

  const { data: logs, isLoading, refetch } = useQuery({
    queryKey: ['logs'],
    queryFn: () => apiService.getLogs(),
    refetchInterval: 5000,
  });

  const filteredLogs = logs?.filter(log => {
    const matchesSearch = log.message.toLowerCase().includes(searchTerm.toLowerCase()) ||
                          log.source.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesLevel = levelFilter === 'all' || log.level === levelFilter;
    return matchesSearch && matchesLevel;
  });

  return (
    <div className="logs-page">
      <div className="page-header">
        <div>
          <h1>Logs</h1>
          <p className="page-description">View and search system logs in real-time</p>
        </div>
        <button className="btn btn-primary" onClick={() => refetch()}>
          <RefreshCw size={18} />
          Refresh
        </button>
      </div>

      <div className="logs-controls card">
        <div className="search-box">
          <Search size={20} />
          <input
            type="text"
            placeholder="Search logs..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>

        <select
          value={levelFilter}
          onChange={(e) => setLevelFilter(e.target.value)}
          className="level-filter"
        >
          <option value="all">All Levels</option>
          <option value="info">Info</option>
          <option value="warning">Warning</option>
          <option value="error">Error</option>
          <option value="critical">Critical</option>
        </select>

        <button className="btn btn-secondary">
          <Download size={18} />
          Export
        </button>
      </div>

      <div className="logs-container card">
        {isLoading ? (
          <div className="loading-container">
            <div className="spinner"></div>
            <p>Loading logs...</p>
          </div>
        ) : (
          <div className="logs-list">
            {filteredLogs?.map((log, index) => (
              <div key={index} className="log-entry">
                <div className="log-header">
                  <LogLevelBadge level={log.level} />
                  <span className="log-time">
                    {formatDistanceToNow(log.timestamp, { addSuffix: true })}
                  </span>
                  <span className="log-source">{log.source}</span>
                </div>
                <p className="log-message">{log.message}</p>
                {Object.keys(log.fields).length > 0 && (
                  <div className="log-fields">
                    {Object.entries(log.fields).map(([key, value]) => (
                      <span key={key} className="field-tag">
                        {key}: {value}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
