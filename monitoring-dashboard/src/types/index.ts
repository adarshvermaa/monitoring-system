// API Types matching Rust backend
export interface LogEvent {
    timestamp: number;
    source: string;
    level: 'trace' | 'debug' | 'info' | 'warning' | 'error' | 'critical';
    message: string;
    fields: Record<string, string>;
    tags: string[];
}

export interface MetricEvent {
    timestamp: number;
    name: string;
    value: number;
    metric_type: 'counter' | 'gauge' | 'histogram' | 'summary';
    tags: Record<string, string>;
    unit?: string;
}

export interface TrafficEvent {
    timestamp: number;
    protocol: 'HTTP' | 'HTTPS' | 'TCP' | 'UDP' | 'ICMP' | string;
    src_ip: string;
    dst_ip: string;
    src_port: number;
    dst_port: number;
    bytes: number;
    packets: number;
    metadata: Record<string, string>;
}

export type Event =
    | { type: 'log'; data: LogEvent; }
    | { type: 'metric'; data: MetricEvent; }
    | { type: 'traffic'; data: TrafficEvent; };

export interface Batch {
    batch_id: string;
    agent_id: string;
    hostname: string;
    timestamp: number;
    event_count: number;
    compression: 'none' | 'snappy' | 'lz4' | 'gzip';
    compressed_data: number[];
    checksum: string;
}

export interface IngestResponse {
    batch_id: string;
    status: 'success' | 'partial_success' | 'failed' | 'rejected';
    error_message?: string;
    received_at: number;
}

// Dashboard Types
export interface Agent {
    id: string;
    hostname: string;
    tags: string[];
    status: 'online' | 'offline' | 'warning';
    last_seen: number;
    events_sent: number;
    cpu_usage?: number;
    memory_usage?: number;
}

export interface DashboardStats {
    total_events: number;
    events_per_second: number;
    total_agents: number;
    online_agents: number;
    error_count: number;
    warning_count: number;
}

export interface LogQuery {
    level?: LogEvent['level'][];
    source?: string[];
    search?: string;
    start_time?: number;
    end_time?: number;
    limit?: number;
}

export interface MetricQuery {
    metric_names?: string[];
    tags?: Record<string, string>;
    start_time?: number;
    end_time?: number;
    aggregation?: 'avg' | 'sum' | 'min' | 'max';
}

// Auth Types
export interface User {
    id: string;
    username: string;
    email: string;
    role: 'admin' | 'viewer';
}

export interface LoginCredentials {
    username: string;
    password: string;
}

export interface AuthResponse {
    token: string;
    user: User;
}
