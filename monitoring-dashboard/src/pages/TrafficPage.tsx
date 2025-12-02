import { useQuery } from '@tanstack/react-query';
import { apiService } from '../services/api';
import { formatDistanceToNow } from 'date-fns';

export const TrafficPage = () => {
  const { data: traffic, isLoading } = useQuery({
    queryKey: ['traffic'],
    queryFn: () => apiService.getTraffic(),
    refetchInterval: 5000,
  });

  return (
    <div className="traffic-page">
      <div className="page-header">
        <h1>Network Traffic</h1>
        <p className="page-description">Monitor network connections and traffic patterns</p>
      </div>

      {isLoading ? (
        <div className="loading-container">
          <div className="spinner"></div>
        </div>
      ) : (
        <div className="card">
          <div className="table-container">
            <table className="data-table">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Protocol</th>
                  <th>Source</th>
                  <th>Destination</th>
                  <th>Bytes</th>
                  <th>Packets</th>
                </tr>
              </thead>
              <tbody>
                {traffic?.map((t, i) => (
                  <tr key={i}>
                    <td>{formatDistanceToNow(t.timestamp, { addSuffix: true })}</td>
                    <td><span className="badge badge-info">{t.protocol}</span></td>
                    <td>{t.src_ip}:{t.src_port}</td>
                    <td>{t.dst_ip}:{t.dst_port}</td>
                    <td>{(t.bytes / 1024).toFixed(2)} KB</td>
                    <td>{t.packets}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};
