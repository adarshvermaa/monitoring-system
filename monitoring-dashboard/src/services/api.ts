import { config } from '../config';
import type { LogEvent, MetricEvent, TrafficEvent, DashboardStats, Agent } from '../types';

class WebSocketService {
    private ws: WebSocket | null = null;
    private reconnectAttempts = 0;
    private maxReconnectAttempts = 5;
    private reconnectDelay = 1000;
    private listeners: Map<string, Set<(data: any) => void>> = new Map();

    connect(token: string) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            return;
        }

        const wsUrl = `${config.wsIngestUrl}?token=${token}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.reconnectAttempts = 0;
            this.emit('connected', true);
        };

        this.ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                this.handleMessage(data);
            } catch (error) {
                console.error('Failed to parse WebSocket message:', error);
            }
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.emit('error', error);
        };

        this.ws.onclose = () => {
            console.log('WebSocket disconnected');
            this.emit('disconnected', true);
            this.attemptReconnect(token);
        };
    }

    private attemptReconnect(token: string) {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
            console.log(`Attempting to reconnect in ${delay}ms...`);

            setTimeout(() => {
                this.connect(token);
            }, delay);
        }
    }

    private handleMessage(message: any) {
        // Handle different message types
        if (message.type === 'event') {
            this.emit('event', message.data);
        } else if (message.type === 'batch') {
            this.emit('batch', message.data);
        } else if (message.type === 'stats') {
            this.emit('stats', message.data);
        }
    }

    on(event: string, callback: (data: any) => void) {
        if (!this.listeners.has(event)) {
            this.listeners.set(event, new Set());
        }
        this.listeners.get(event)!.add(callback);
    }

    off(event: string, callback: (data: any) => void) {
        if (this.listeners.has(event)) {
            this.listeners.get(event)!.delete(callback);
        }
    }

    private emit(event: string, data: any) {
        if (this.listeners.has(event)) {
            this.listeners.get(event)!.forEach(callback => callback(data));
        }
    }

    send(data: any) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(data));
        }
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }
}

export const wsService = new WebSocketService();

// API Service for HTTP requests
class ApiService {
    async checkHealth(): Promise<{ status: string; }> {
        const response = await fetch(config.healthUrl);
        return response.text().then(status => ({ status }));
    }

    async login(username: string, _password: string) {
        // Mock login - replace with actual API call
        return {
            token: 'mock-jwt-token',
            user: {
                id: '1',
                username,
                email: `${username}@example.com`,
                role: 'admin' as const,
            },
        };
    }

    async getDashboardStats(): Promise<DashboardStats> {
        // Mock data - would call actual API
        return {
            total_events: 1250000,
            events_per_second: 152,
            total_agents: 12,
            online_agents: 10,
            error_count: 45,
            warning_count: 128,
        };
    }

    async getAgents(): Promise<Agent[]> {
        // Mock data
        return [
            {
                id: 'agent-001',
                hostname: 'web-server-01',
                tags: ['env:prod', 'region:us-east'],
                status: 'online',
                last_seen: Date.now(),
                events_sent: 45230,
                cpu_usage: 23.5,
                memory_usage: 67.2,
            },
            {
                id: 'agent-002',
                hostname: 'web-server-02',
                tags: ['env:prod', 'region:us-east'],
                status: 'online',
                last_seen: Date.now() - 2000,
                events_sent: 42180,
                cpu_usage: 19.8,
                memory_usage: 71.5,
            },
            {
                id: 'agent-003',
                hostname: 'db-server-01',
                tags: ['env:prod', 'region:us-west'],
                status: 'warning',
                last_seen: Date.now() - 45000,
                events_sent: 38950,
                cpu_usage: 78.2,
                memory_usage: 89.3,
            },
        ];
    }

    async getLogs(_params?: any): Promise<LogEvent[]> {
        // Mock data
        return Array.from({ length: 50 }, (_, i) => ({
            timestamp: Date.now() - i * 60000,
            source: '/var/log/nginx/access.log',
            level: ['info', 'warning', 'error'][Math.floor(Math.random() * 3)] as any,
            message: `Sample log message ${i}`,
            fields: { request_id: `req_${i}` },
            tags: ['env:prod'],
        }));
    }

    async getMetrics(_params?: any): Promise<MetricEvent[]> {
        // Mock data
        return Array.from({ length: 50 }, (_, i) => ({
            timestamp: Date.now() - i * 10000,
            name: 'system.cpu.usage',
            value: Math.random() * 100,
            metric_type: 'gauge' as const,
            tags: { host: 'web-01' },
            unit: '%',
        }));
    }

    async getTraffic(_params?: any): Promise<TrafficEvent[]> {
        // Mock data
        return Array.from({ length: 30 }, (_, i) => ({
            timestamp: Date.now() - i * 5000,
            protocol: 'HTTP',
            src_ip: `192.168.1.${100 + i}`,
            dst_ip: '10.0.0.1',
            src_port: 50000 + i,
            dst_port: 80,
            bytes: Math.floor(Math.random() * 100000),
            packets: Math.floor(Math.random() * 100),
            metadata: {},
        }));
    }
}

export const apiService = new ApiService();
