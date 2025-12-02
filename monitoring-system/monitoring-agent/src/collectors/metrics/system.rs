use crate::buffer::RingBuffer;
use anyhow::Result;
use monitoring_common::{Event, MetricEvent, MetricType};
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, ProcessExt};
use tracing::{debug, warn};

pub struct SystemMetrics {
    interval_secs: u64,
    include_process_metrics: bool,
    buffer: Arc<RingBuffer>,
    sys: System,
}

impl SystemMetrics {
    pub fn new(
        interval_secs: u64,
        include_process_metrics: bool,
        buffer: Arc<RingBuffer>,
    ) -> Self {
        Self {
            interval_secs,
            include_process_metrics,
            buffer,
            sys: System::new_all(),
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.interval_secs)
        );

        loop {
            interval.tick().await;
            
            // Refresh system information
            self.sys.refresh_all();

            // Collect CPU metrics
            self.collect_cpu_metrics();

            // Collect memory metrics
            self.collect_memory_metrics();

            // Collect disk metrics
            self.collect_disk_metrics();

            // Collect network metrics
            self.collect_network_metrics();

            // Collect process metrics (optional)
            if self.include_process_metrics {
                self.collect_process_metrics();
            }
        }
    }

    fn collect_cpu_metrics(&self) {
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Overall CPU usage
        let global_cpu = self.sys.global_cpu_info();
        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.cpu.usage".to_string(),
            value: global_cpu.cpu_usage() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "percent".to_string()),
            ]),
            unit: Some("%".to_string()),
        });

        // Per-CPU usage
        for (idx, cpu) in self.sys.cpus().iter().enumerate() {
            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.cpu.usage_per_core".to_string(),
                value: cpu.cpu_usage() as f64,
                metric_type: MetricType::Gauge,
                tags: HashMap::from([
                    ("cpu".to_string(), idx.to_string()),
                    ("unit".to_string(), "percent".to_string()),
                ]),
                unit: Some("%".to_string()),
            });
        }

        // Load average (on supported platforms)
        if let Some(load_avg) = sysinfo::System::load_average() {
            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.load.1".to_string(),
                value: load_avg.one,
                metric_type: MetricType::Gauge,
                tags: HashMap::new(),
                unit: None,
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.load.5".to_string(),
                value: load_avg.five,
                metric_type: MetricType::Gauge,
                tags: HashMap::new(),
                unit: None,
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.load.15".to_string(),
                value: load_avg.fifteen,
                metric_type: MetricType::Gauge,
                tags: HashMap::new(),
                unit: None,
            });
        }
    }

    fn collect_memory_metrics(&self) {
        let timestamp = chrono::Utc::now().timestamp_millis();

        // RAM metrics
        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.mem.total".to_string(),
            value: self.sys.total_memory() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "bytes".to_string()),
            ]),
            unit: Some("bytes".to_string()),
        });

        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.mem.used".to_string(),
            value: self.sys.used_memory() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "bytes".to_string()),
            ]),
            unit: Some("bytes".to_string()),
        });

        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.mem.free".to_string(),
            value: self.sys.free_memory() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "bytes".to_string()),
            ]),
            unit: Some("bytes".to_string()),
        });

        let mem_usage = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100.0;
        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.mem.usage".to_string(),
            value: mem_usage,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "percent".to_string()),
            ]),
            unit: Some("%".to_string()),
        });

        // Swap metrics
        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.swap.total".to_string(),
            value: self.sys.total_swap() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "bytes".to_string()),
            ]),
            unit: Some("bytes".to_string()),
        });

        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.swap.used".to_string(),
            value: self.sys.used_swap() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("unit".to_string(), "bytes".to_string()),
            ]),
            unit: Some("bytes".to_string()),
        });
    }

    fn collect_disk_metrics(&self) {
        let timestamp = chrono::Utc::now().timestamp_millis();

        for disk in self.sys.disks() {
            let mount_point = disk.mount_point().to_string_lossy().to_string();
            let tags = HashMap::from([
                ("device".to_string(), disk.name().to_string_lossy().to_string()),
                ("mount_point".to_string(), mount_point.clone()),
            ]);

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.disk.total".to_string(),
                value: disk.total_space() as f64,
                metric_type: MetricType::Gauge,
                tags: tags.clone(),
                unit: Some("bytes".to_string()),
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.disk.free".to_string(),
                value: disk.available_space() as f64,
                metric_type: MetricType::Gauge,
                tags: tags.clone(),
                unit: Some("bytes".to_string()),
            });

            let used = disk.total_space() - disk.available_space();
            let usage = (used as f64 / disk.total_space() as f64) * 100.0;

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.disk.used".to_string(),
                value: used as f64,
                metric_type: MetricType::Gauge,
                tags: tags.clone(),
                unit: Some("bytes".to_string()),
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.disk.usage".to_string(),
                value: usage,
                metric_type: MetricType::Gauge,
                tags,
                unit: Some("%".to_string()),
            });
        }
    }

    fn collect_network_metrics(&self) {
        let timestamp = chrono::Utc::now().timestamp_millis();

        for (interface_name, data) in self.sys.networks() {
            let tags = HashMap::from([
                ("interface".to_string(), interface_name.to_string()),
            ]);

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.bytes_recv".to_string(),
                value: data.total_received() as f64,
                metric_type: MetricType::Counter,
                tags: tags.clone(),
                unit: Some("bytes".to_string()),
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.bytes_sent".to_string(),
                value: data.total_transmitted() as f64,
                metric_type: MetricType::Counter,
                tags: tags.clone(),
                unit: Some("bytes".to_string()),
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.packets_recv".to_string(),
                value: data.total_packets_received() as f64,
                metric_type: MetricType::Counter,
                tags: tags.clone(),
                unit: None,
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.packets_sent".to_string(),
                value: data.total_packets_transmitted() as f64,
                metric_type: MetricType::Counter,
                tags: tags.clone(),
                unit: None,
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.errors_recv".to_string(),
                value: data.total_errors_on_received() as f64,
                metric_type: MetricType::Counter,
                tags: tags.clone(),
                unit: None,
            });

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.net.errors_sent".to_string(),
                value: data.total_errors_on_transmitted() as f64,
                metric_type: MetricType::Counter,
                tags,
                unit: None,
            });
        }
    }

    fn collect_process_metrics(&self) {
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Top N processes by CPU/memory
        let mut processes_cpu: Vec<_> = self.sys.processes().values().collect();
        proc.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

        // Top 10 by CPU
        for (rank, process) in processes_cpu.iter().take(10).enumerate() {
            let tags = HashMap::from([
                ("pid".to_string(), process.pid().to_string()),
                ("name".to_string(), process.name().to_string()),
                ("rank".to_string(), (rank + 1).to_string()),
            ]);

            self.emit_metric(MetricEvent {
                timestamp,
                name: "system.process.cpu".to_string(),
                value: process.cpu_usage() as f64,
                metric_type: MetricType::Gauge,
                tags,
                unit: Some("%".to_string()),
            });
        }

        // Total process count
        self.emit_metric(MetricEvent {
            timestamp,
            name: "system.process.count".to_string(),
            value: self.sys.processes().len() as f64,
            metric_type: MetricType::Gauge,
            tags: HashMap::new(),
            unit: None,
        });
    }

    fn emit_metric(&self, metric: MetricEvent) {
        let event = Event::Metric(metric);
        if let Err(e) = self.buffer.push(event) {
            warn!("Buffer full, dropping metric: {}", e);
        }
    }
}
