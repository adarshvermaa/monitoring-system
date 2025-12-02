const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';
const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080';

export const config = {
    apiBaseUrl: API_BASE_URL,
    wsUrl: WS_URL,
    wsIngestUrl: `${WS_URL}/ingest`,
    healthUrl: `${API_BASE_URL}/health`,
    refreshInterval: 5000, // 5 seconds
    maxRetries: 3,
    retryDelay: 1000,
};
