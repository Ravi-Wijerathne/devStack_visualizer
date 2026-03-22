import {
  React,
  useState,
  useEffect,
  useCallback,
  useMemo,
  useRef,
  createContext,
  useContext,
  useReducer,
  useLayoutEffect,
  forwardRef,
  useImperativeHandle,
  createElement,
  Fragment,
  Suspense,
  lazy,
  memo,
  Component,
  PureComponent,
  createRef,
} from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter, Routes, Route, Link, useParams, useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';
import { create } from 'zustand';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar, PieChart, Pie, Cell } from 'recharts';
import { format, parseISO, differenceInDays, addDays } from 'date-fns';

interface User {
  id: number;
  name: string;
  email: string;
  role: 'admin' | 'user' | 'guest';
  avatar?: string;
  createdAt: string;
}

interface DashboardStats {
  totalUsers: number;
  activeUsers: number;
  totalRevenue: number;
  revenueChange: number;
}

interface ChartDataPoint {
  date: string;
  value: number;
  label?: string;
}

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
}

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary' | 'danger';
  disabled?: boolean;
}

interface InputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: 'text' | 'email' | 'password' | 'number';
  error?: string;
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884d8'];

export const App: React.FC = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [chartData, setChartData] = useState<ChartDataPoint[]>([]);
  const [selectedPeriod, setSelectedPeriod] = useState<'week' | 'month' | 'year'>('month');

  useEffect(() => {
    const fetchData = async () => {
      try {
        setIsLoading(true);
        const [statsResponse, chartResponse] = await Promise.all([
          axios.get<DashboardStats>('/api/dashboard/stats'),
          axios.get<ChartDataPoint[]>('/api/dashboard/chart'),
        ]);
        setStats(statsResponse.data);
        setChartData(chartResponse.data);
        setError(null);
      } catch (err) {
        setError('Failed to load dashboard data');
        console.error('Dashboard error:', err);
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, [selectedPeriod]);

  const handlePeriodChange = useCallback((period: 'week' | 'month' | 'year') => {
    setSelectedPeriod(period);
  }, []);

  const renderContent = () => {
    if (isLoading) {
      return <div className="loading-spinner">Loading...</div>;
    }

    if (error) {
      return <div className="error-message">{error}</div>;
    }

    return (
      <div className="dashboard-content">
        <StatsCards stats={stats} />
        <ChartSection data={chartData} />
        <UserTable users={[]} />
      </div>
    );
  };

  return (
    <div className="app-container">
      <Header user={user} />
      <main className="main-content">
        {renderContent()}
      </main>
      <Footer />
    </div>
  );
};

const StatsCards: React.FC<{ stats: DashboardStats | null }> = ({ stats }) => {
  const cards = [
    { label: 'Total Users', value: stats?.totalUsers ?? 0, change: '+12%' },
    { label: 'Active Users', value: stats?.activeUsers ?? 0, change: '+8%' },
    { label: 'Revenue', value: stats?.totalRevenue ?? 0, change: `${stats?.revenueChange ?? 0}%` },
  ];

  return (
    <div className="stats-grid">
      {cards.map((card, index) => (
        <StatCard key={index} {...card} />
      ))}
    </div>
  );
};

const StatCard: React.FC<{ label: string; value: number; change: string }> = memo(({ label, value, change }) => {
  const isPositive = change.startsWith('+');
  
  return (
    <div className="stat-card">
      <h3>{label}</h3>
      <div className="stat-value">{value.toLocaleString()}</div>
      <div className={`stat-change ${isPositive ? 'positive' : 'negative'}`}>
        {change}
      </div>
    </div>
  );
});

const ChartSection: React.FC<{ data: ChartDataPoint[] }> = ({ data }) => {
  return (
    <div className="chart-section">
      <h2>Analytics Overview</h2>
      <ResponsiveContainer width="100%" height={400}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="date" />
          <YAxis />
          <Tooltip />
          <Line type="monotone" dataKey="value" stroke="#8884d8" />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

class UserTable extends Component<{ users: User[] }> {
  constructor(props: { users: User[] }) {
    super(props);
    this.state = { selectedRows: new Set<number>() };
  }

  handleRowClick = (userId: number) => {
    this.setState((prevState) => {
      const newSelected = new Set(prevState.selectedRows);
      if (newSelected.has(userId)) {
        newSelected.delete(userId);
      } else {
        newSelected.add(userId);
      }
      return { selectedRows: newSelected };
    });
  };

  render() {
    const { users } = this.props;
    const { selectedRows } = this.state;

    return (
      <table className="user-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Role</th>
            <th>Created</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <tr
              key={user.id}
              className={selectedRows.has(user.id) ? 'selected' : ''}
              onClick={() => this.handleRowClick(user.id)}
            >
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
              <td>{user.role}</td>
              <td>{format(parseISO(user.createdAt), 'MMM dd, yyyy')}</td>
            </tr>
          ))}
        </tbody>
      </table>
    );
  }
}

const Header: React.FC<{ user: User | null }> = ({ user }) => (
  <header className="app-header">
    <h1>Dashboard</h1>
    <nav>
      <Link to="/">Home</Link>
      <Link to="/users">Users</Link>
      <Link to="/settings">Settings</Link>
    </nav>
    {user && <span className="user-name">{user.name}</span>}
  </header>
);

const Footer: React.FC = () => (
  <footer className="app-footer">
    <p>2024 Company Name. All rights reserved.</p>
  </footer>
);

export default App;
