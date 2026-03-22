import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import GraphView from '../../components/GraphView';
import type { GraphData, GraphNode, GraphEdge } from '../../types';

const mockOnNodeClick = vi.fn();

const createMockGraphData = (
  nodes: Partial<GraphNode>[] = [],
  edges: Partial<GraphEdge>[] = []
): GraphData => ({
  nodes: nodes.map((n, i) => ({
    id: n.id || `node-${i}`,
    label: n.label || `Node ${i}`,
    file_path: n.file_path || `/path/node-${i}.rs`,
    node_type: n.node_type || 'rust',
    complexity: n.complexity || 'Low',
    functions_count: n.functions_count || 0,
    structs_count: n.structs_count || 0,
    ...n,
  })),
  edges: edges.map((e, i) => ({
    source: e.source || `node-${i}`,
    target: e.target || `node-${i + 1}`,
    is_circular: e.is_circular || false,
    ...e,
  })),
});

describe('GraphView Snapshots', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders empty state when graphData is null', () => {
    render(<GraphView graphData={null} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/No graph data/)).toBeInTheDocument();
  });

  it('renders empty state when graphData has no nodes', () => {
    const emptyGraph = createMockGraphData([], []);
    render(<GraphView graphData={emptyGraph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/No graph data/)).toBeInTheDocument();
  });

  it('renders single node correctly', () => {
    const singleNodeGraph = createMockGraphData([
      {
        id: 'main.rs',
        label: 'main',
        file_path: '/src/main.rs',
        node_type: 'rust',
        complexity: 'Low',
        functions_count: 1,
        structs_count: 0,
      },
    ]);
    
    render(<GraphView graphData={singleNodeGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('main')).toBeInTheDocument();
  });

  it('renders multiple nodes correctly', () => {
    const multiNodeGraph = createMockGraphData([
      { id: 'main.rs', label: 'main', node_type: 'rust', complexity: 'Low' },
      { id: 'lib.rs', label: 'lib', node_type: 'rust', complexity: 'Medium' },
      { id: 'utils.rs', label: 'utils', node_type: 'rust', complexity: 'High' },
    ]);
    
    render(<GraphView graphData={multiNodeGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('main')).toBeInTheDocument();
    expect(screen.getByText('lib')).toBeInTheDocument();
    expect(screen.getByText('utils')).toBeInTheDocument();
  });

  it('displays node complexity information', () => {
    const graph = createMockGraphData([
      {
        id: 'complex.rs',
        label: 'complex',
        functions_count: 5,
        structs_count: 3,
        complexity: 'Medium',
      },
    ]);
    
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText(/5fn/)).toBeInTheDocument();
    expect(screen.getByText(/3st/)).toBeInTheDocument();
    expect(screen.getByText(/Medium/)).toBeInTheDocument();
  });

  it('renders nodes with different types', () => {
    const mixedGraph = createMockGraphData([
      { id: 'main.rs', label: 'main', node_type: 'rust' },
      { id: 'app.py', label: 'app', node_type: 'python' },
      { id: 'index.ts', label: 'index', node_type: 'js' },
    ]);
    
    render(<GraphView graphData={mixedGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('main')).toBeInTheDocument();
    expect(screen.getByText('app')).toBeInTheDocument();
    expect(screen.getByText('index')).toBeInTheDocument();
  });

  it('renders graph with circular dependencies', () => {
    const circularGraph = createMockGraphData(
      [
        { id: 'a.rs', label: 'a' },
        { id: 'b.rs', label: 'b' },
      ],
      [
        { source: 'a.rs', target: 'b.rs', is_circular: true },
        { source: 'b.rs', target: 'a.rs', is_circular: true },
      ]
    );
    
    render(<GraphView graphData={circularGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('a')).toBeInTheDocument();
    expect(screen.getByText('b')).toBeInTheDocument();
  });

  it('renders graph with linear dependencies', () => {
    const linearGraph = createMockGraphData(
      [
        { id: 'main.rs', label: 'main' },
        { id: 'lib.rs', label: 'lib' },
        { id: 'utils.rs', label: 'utils' },
      ],
      [
        { source: 'main.rs', target: 'lib.rs', is_circular: false },
        { source: 'lib.rs', target: 'utils.rs', is_circular: false },
      ]
    );
    
    render(<GraphView graphData={linearGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('main')).toBeInTheDocument();
    expect(screen.getByText('lib')).toBeInTheDocument();
    expect(screen.getByText('utils')).toBeInTheDocument();
  });

  it('renders diamond dependency pattern', () => {
    const diamondGraph = createMockGraphData(
      [
        { id: 'a.rs', label: 'a' },
        { id: 'b.rs', label: 'b' },
        { id: 'c.rs', label: 'c' },
        { id: 'd.rs', label: 'd' },
      ],
      [
        { source: 'a.rs', target: 'b.rs', is_circular: false },
        { source: 'a.rs', target: 'c.rs', is_circular: false },
        { source: 'b.rs', target: 'd.rs', is_circular: false },
        { source: 'c.rs', target: 'd.rs', is_circular: false },
      ]
    );
    
    render(<GraphView graphData={diamondGraph} onNodeClick={mockOnNodeClick} />);
    
    expect(screen.getByText('a')).toBeInTheDocument();
    expect(screen.getByText('b')).toBeInTheDocument();
    expect(screen.getByText('c')).toBeInTheDocument();
    expect(screen.getByText('d')).toBeInTheDocument();
  });

  it('renders large graph with many nodes', () => {
    const largeGraph = createMockGraphData(
      Array.from({ length: 20 }, (_, i) => ({
        id: `module_${i}.rs`,
        label: `module_${i}`,
        complexity: i % 3 === 0 ? 'High' : i % 3 === 1 ? 'Medium' : 'Low',
        functions_count: Math.floor(Math.random() * 10),
        structs_count: Math.floor(Math.random() * 5),
      }))
    );
    
    render(<GraphView graphData={largeGraph} onNodeClick={mockOnNodeClick} />);
    
    for (let i = 0; i < 20; i++) {
      expect(screen.getByText(`module_${i}`)).toBeInTheDocument();
    }
  });
});

describe('GraphView Node Types', () => {
  it('displays rust nodes correctly', () => {
    const graph = createMockGraphData([{ id: 'main.rs', label: 'main', node_type: 'rust' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText('main')).toBeInTheDocument();
  });

  it('displays python nodes correctly', () => {
    const graph = createMockGraphData([{ id: 'app.py', label: 'app', node_type: 'python' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText('app')).toBeInTheDocument();
  });

  it('displays js/ts nodes correctly', () => {
    const graph = createMockGraphData([{ id: 'index.ts', label: 'index', node_type: 'js' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText('index')).toBeInTheDocument();
  });

  it('displays other file types correctly', () => {
    const graph = createMockGraphData([{ id: 'config.json', label: 'config', node_type: 'other' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText('config')).toBeInTheDocument();
  });
});

describe('GraphView Complexity Levels', () => {
  it('displays Low complexity nodes', () => {
    const graph = createMockGraphData([{ id: 'simple.rs', label: 'simple', complexity: 'Low' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/Low/)).toBeInTheDocument();
  });

  it('displays Medium complexity nodes', () => {
    const graph = createMockGraphData([{ id: 'medium.rs', label: 'medium', complexity: 'Medium' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/Medium/)).toBeInTheDocument();
  });

  it('displays High complexity nodes', () => {
    const graph = createMockGraphData([{ id: 'complex.rs', label: 'complex', complexity: 'High' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/High/)).toBeInTheDocument();
  });

  it('displays Unknown complexity nodes', () => {
    const graph = createMockGraphData([{ id: 'unknown.rs', label: 'unknown', complexity: 'Unknown' }]);
    render(<GraphView graphData={graph} onNodeClick={mockOnNodeClick} />);
    expect(screen.getByText(/Unknown/)).toBeInTheDocument();
  });
});
