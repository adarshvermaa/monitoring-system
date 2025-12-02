#!/bin/bash
# Quick start script for local development

set -e

echo "🚀 Starting Monitoring System..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust installation found"

# Build all components
echo "📦 Building all components..."
cargo build --release --all

echo ""
echo "✅ Build complete!"
echo ""

# Set up environment
export JWT_SECRET="dev-secret-$(date +%s)"
export MONITORING_AUTH_TOKEN="dev-token-$(date +%s)"

echo "🔑 Generated temporary credentials:"
echo "   JWT_SECRET=$JWT_SECRET"
echo "   MONITORING_AUTH_TOKEN=$MONITORING_AUTH_TOKEN"
echo ""

# Create log directory
mkdir -p logs

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Shutting down..."
    pkill -P $$ || true
    exit 0
}

trap cleanup SIGINT SIGTERM

# Start collector in background
echo "🌐 Starting collector server on ws://localhost:8080..."
JWT_SECRET=$JWT_SECRET \
    ./target/release/monitoring-collector \
    --config config/collector.toml \
    > logs/collector.log 2>&1 &

COLLECTOR_PID=$!
echo "   Collector PID: $COLLECTOR_PID"

# Wait for collector to start
sleep 2

# Check if collector is running
if ! kill -0 $COLLECTOR_PID 2>/dev/null; then
    echo "❌ Collector failed to start. Check logs/collector.log"
    exit 1
fi

echo "✅ Collector started successfully"
echo ""

# Start agent
echo "📡 Starting monitoring agent..."
MONITORING_AUTH_TOKEN=$MONITORING_AUTH_TOKEN \
    ./target/release/monitoring-agent \
    --config config/agent.toml \
    > logs/agent.log 2>&1 &

AGENT_PID=$!
echo "   Agent PID: $AGENT_PID"

# Wait for agent to start
sleep 2

# Check if agent is running
if ! kill -0 $AGENT_PID 2>/dev/null; then
    echo "❌ Agent failed to start. Check logs/agent.log"
    kill $COLLECTOR_PID 2>/dev/null || true
    exit 1
fi

echo "✅ Agent started successfully"
echo ""

# Display status
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ Monitoring System is Running!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Collector:  ws://localhost:8080/ingest"
echo "🏥 Health:     http://localhost:8080/health"
echo ""
echo "📝 Logs:"
echo "   Collector:  tail -f logs/collector.log"
echo "   Agent:      tail -f logs/agent.log"
echo ""
echo "Press Ctrl+C to stop all services"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test health endpoint
echo "🔍 Testing collector health..."
if curl -s http://localhost:8080/health > /dev/null; then
    echo "✅ Collector is healthy!"
else
    echo "⚠️  Collector health check failed"
fi

echo ""
echo "📡 Monitoring events..."
echo ""

# Tail both logs
tail -f logs/collector.log logs/agent.log
