export const SettingsPage = () => {
  return (
    <div className="settings-page">
      <div className="page-header">
        <h1>Settings</h1>
        <p className="page-description">Configure your monitoring system</p>
      </div>

      <div className="settings-sections">
        <div className="card" style={{ padding: '2rem' }}>
          <h3 style={{ marginBottom: '1.5rem' }}>General Settings</h3>
          
          <div className="form-group">
            <label>Refresh Interval (seconds)</label>
            <input type="number" defaultValue={5} min={1} max={60} />
          </div>

          <div className="form-group">
            <label>Data Retention (days)</label>
            <input type="number" defaultValue={30} min={1} max={365} />
          </div>

          <div className="form-group" style={{ marginTop: '1.5rem' }}>
            <button className="btn btn-primary">Save Settings</button>
          </div>
        </div>

        <div className="card" style={{ padding: '2rem', marginTop: '1.5rem' }}>
          <h3 style={{ marginBottom: '1.5rem' }}>API Configuration</h3>
          
          <div className="form-group">
            <label>Collector URL</label>
            <input type="text" defaultValue="ws://localhost:8080/ingest" />
          </div>

          <div className="form-group">
            <label>Authentication Token</label>
            <input type="password" defaultValue="••••••••••••••••" />
          </div>

          <div className="form-group" style={{ marginTop: '1.5rem' }}>
            <button className="btn btn-primary">Update Configuration</button>
          </div>
        </div>
      </div>
    </div>
  );
};
