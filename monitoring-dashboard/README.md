# Monitoring Dashboard

Beautiful, production-grade React TypeScript dashboard for the Rust monitoring system.

## ✨ Features

- 🔐 **Authentication** - Secure login with JWT
- 🛡️ **Protected Routes** - Route guards for authenticated users
- 📊 **Real-time Dashboard** - Live metrics and statistics
- 📝 **Logs Viewer** - Search and filter system logs
- 📈 **Metrics Charts** - Beautiful visualizations with Recharts
- 🌐 **Traffic Monitor** - Network connection tracking
- 🖥️ **Agent Management** - Monitor connected agents
- 🎨 **Modern UI** - Dark theme with smooth animations
- 📱 **Responsive Design** - Works on all devices

## 🚀 Quick Start

### Prerequisites

- Node.js 18+ and npm
- Rust monitoring collector running on `localhost:8080`

### Installation

```bash
cd d:\cli\monitoring-dashboard

# Install dependencies
npm install

# Start development server
npm run dev
```

The dashboard will open at `http://localhost:5173`

### Default Credentials

```
Username: admin
Password: password
```

## 🏗️ Project Structure

```
monitoring-dashboard/
├── src/
│   ├── components/          # Reusable components
│   │   ├── Header.tsx
│   │   ├── Sidebar.tsx
│   │   └── ProtectedRoute.tsx
│   ├── layouts/            # Layout components
│   │   └── DashboardLayout.tsx
│   ├── pages/              # Page components
│   │   ├── Login.tsx
│   │   ├── DashboardOverview.tsx
│   │   ├── LogsPage.tsx
│   │   ├── MetricsPage.tsx
│   │   ├── TrafficPage.tsx
│   │   ├── AgentsPage.tsx
│   │   └── SettingsPage.tsx
│   ├── services/           # API services
│   │   └── api.ts
│   ├── store/              # State management
│   │   └── authStore.ts
│   ├── types/              # TypeScript types
│   │   └── index.ts
│   ├── config/             # Configuration
│   │   └── index.ts
│   ├── App.tsx            # Main app component
│   ├── main.tsx           # Entry point
│   └── index.css          # Global styles
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 📦 Tech Stack

- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool
- **React Router** - Routing
- **TanStack Query** - Data fetching
- **Zustand** - State management
- **Recharts** - Charts and graphs
- **Lucide React** - Beautiful icons
- **date-fns** - Date utilities

## 🎨 Pages

### Overview Dashboard
- Real-time statistics
- Event trends chart
- System resource graphs
- Quick stats cards

### Logs
- Real-time log streaming
- Search and filter
- Level filtering (info, warning, error)
- Export functionality

### Metrics
- System performance charts
- CPU, memory, disk metrics
- Historical data visualization

### Traffic
- Network connection table
- Protocol breakdown
- Source/destination tracking

### Agents
- Connected agent cards
- Status indicators
- Resource usage meters
- Event counts

### Settings
- Configuration management
- API endpoint settings
- Refresh interval control

## 🔧 Configuration

Create a `.env` file:

```bash
cp .env.example .env
```

Edit `.env`:

```env
VITE_API_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080
```

## 🚢 Building for Production

```bash
# Build for production
npm run build

# Preview production build
npm run preview
```

The build output will be in the `dist/` directory.

## 🌐 Integration with Rust Backend

The dashboard connects to the Rust monitoring collector via:

1. **WebSocket** - Real-time event streaming at `/ingest`
2. **HTTP** - REST API for queries and configuration

Ensure the collector is running:

```bash
cd d:\cli\monitoring-system\monitoring-collector
cargo run -- --config ../config/collector.toml
```

## 🎯 Features In Detail

### Authentication
- JWT-based authentication
- Token stored in localStorage
- Auto-redirect on logout
- Protected routes

### Real-time Updates
- Auto-refresh every 5 seconds
- WebSocket connection for live data
- React Query for efficient caching

### Responsive Design
- Mobile-friendly sidebar
- Adaptive grid layouts
- Touch-friendly controls

### Dark Theme
- Modern dark color scheme
- Smooth transitions
- Glassmorphism effects
- Vibrant accent colors

##📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- Built with React and TypeScript
- Icons by Lucide
- Charts by Recharts
- Integrates with Rust monitoring system
