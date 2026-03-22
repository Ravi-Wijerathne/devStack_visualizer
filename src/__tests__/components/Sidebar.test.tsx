import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Sidebar from '../../components/Sidebar';
import type { FileAnalysis, AnalysisResult } from '../../types';

const mockOnClose = vi.fn();

const createMockFileAnalysis = (overrides: Partial<FileAnalysis> = {}): FileAnalysis => ({
  file: '/src/main.rs',
  imports: ['std::fmt', 'std::io'],
  functions: ['main', 'helper'],
  structs: ['MyStruct'],
  ...overrides,
});

const createMockAnalysisResult = (overrides: Partial<AnalysisResult> = {}): AnalysisResult => ({
  stack: { backend: 'rust', frontend: null, database: null, containerized: false, secondary_languages: [] },
  files_parsed: 10,
  total_nodes: 5,
  total_edges: 8,
  circular_dependencies: [],
  graph_data: { nodes: [], edges: [] },
  file_analyses: [],
  ...overrides,
});

describe('Sidebar Snapshots', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders nothing when both props are null', () => {
    const { container } = render(
      <Sidebar analysisResult={null} selectedFile={null} onClose={mockOnClose} />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders Project Overview when analysisResult is provided', () => {
    const result = createMockAnalysisResult();
    render(<Sidebar analysisResult={result} selectedFile={null} onClose={mockOnClose} />);
    
    expect(screen.getByText('Project Overview')).toBeInTheDocument();
    expect(screen.getByText('Stack')).toBeInTheDocument();
    expect(screen.getByText('Summary')).toBeInTheDocument();
  });

  it('renders File Details when selectedFile is provided', () => {
    const file = createMockFileAnalysis();
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText('Path')).toBeInTheDocument();
    expect(screen.getByText(/Imports/)).toBeInTheDocument();
    expect(screen.getByText(/Functions/)).toBeInTheDocument();
    expect(screen.getByText(/Structs/)).toBeInTheDocument();
  });

  it('prefers File Details over Project Overview when both provided', () => {
    const file = createMockFileAnalysis({ file: '/test/custom.rs' });
    const result = createMockAnalysisResult();
    
    render(<Sidebar analysisResult={result} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getAllByText(/custom\.rs/).length).toBeGreaterThan(0);
  });

  it('displays file path correctly', () => {
    const file = createMockFileAnalysis({ file: '/src/commands/mod.rs' });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/commands\/mod\.rs/)).toBeInTheDocument();
  });

  it('displays imports list', () => {
    const file = createMockFileAnalysis({
      imports: ['std::fmt', 'std::io', 'std::collections']
    });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Imports \(3\)/)).toBeInTheDocument();
    expect(screen.getByText('std::fmt')).toBeInTheDocument();
    expect(screen.getByText('std::io')).toBeInTheDocument();
  });

  it('displays empty imports message', () => {
    const file = createMockFileAnalysis({ imports: [] });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Imports \(0\)/)).toBeInTheDocument();
    expect(screen.getByText(/None/)).toBeInTheDocument();
  });

  it('displays functions list', () => {
    const file = createMockFileAnalysis({
      functions: ['main', 'helper', 'process']
    });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Functions \(3\)/)).toBeInTheDocument();
    expect(screen.getByText(/fn main\(\)/)).toBeInTheDocument();
    expect(screen.getByText(/fn helper\(\)/)).toBeInTheDocument();
  });

  it('displays structs list', () => {
    const file = createMockFileAnalysis({
      structs: ['MyStruct', 'AnotherStruct']
    });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Structs \/ Types \(2\)/)).toBeInTheDocument();
    expect(screen.getByText('MyStruct')).toBeInTheDocument();
    expect(screen.getByText('AnotherStruct')).toBeInTheDocument();
  });

  it('displays project stack information', () => {
    const result = createMockAnalysisResult({
      stack: {
        backend: 'Rust',
        frontend: 'React',
        database: 'PostgreSQL',
        containerized: true,
        secondary_languages: [],
      }
    });
    render(<Sidebar analysisResult={result} selectedFile={null} onClose={mockOnClose} />);
    
    expect(screen.getByText('Backend:')).toBeInTheDocument();
    expect(screen.getByText('Frontend:')).toBeInTheDocument();
    expect(screen.getByText('Database:')).toBeInTheDocument();
    expect(screen.getByText('Containerized:')).toBeInTheDocument();
  });

  it('displays project summary statistics', () => {
    const result = createMockAnalysisResult({
      files_parsed: 42,
      total_nodes: 15,
      total_edges: 25,
      circular_dependencies: [],
    });
    render(<Sidebar analysisResult={result} selectedFile={null} onClose={mockOnClose} />);
    
    expect(screen.getByText('42')).toBeInTheDocument();
    expect(screen.getByText('15')).toBeInTheDocument();
    expect(screen.getByText('25')).toBeInTheDocument();
  });

  it('displays circular dependencies when present', () => {
    const result = createMockAnalysisResult({
      circular_dependencies: [['a.rs', 'b.rs'], ['c.rs', 'd.rs']]
    });
    render(<Sidebar analysisResult={result} selectedFile={null} onClose={mockOnClose} />);
    
    expect(screen.getByText('Circular Dependencies')).toBeInTheDocument();
    expect(screen.getByText(/a\.rs ↔ b\.rs/)).toBeInTheDocument();
    expect(screen.getByText(/c\.rs ↔ d\.rs/)).toBeInTheDocument();
  });

  it('calls onClose when close button is clicked', async () => {
    const user = userEvent.setup();
    const file = createMockFileAnalysis();
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    const closeButton = screen.getByRole('button', { name: '×' });
    await user.click(closeButton);
    
    expect(mockOnClose).toHaveBeenCalledTimes(1);
  });
});

describe('Sidebar Edge Cases', () => {
  it('handles file with no functions', () => {
    const file = createMockFileAnalysis({ functions: [] });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Functions \(0\)/)).toBeInTheDocument();
    expect(screen.getByText(/None/)).toBeInTheDocument();
  });

  it('handles file with no structs', () => {
    const file = createMockFileAnalysis({ structs: [] });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Structs \/ Types \(0\)/)).toBeInTheDocument();
    expect(screen.getByText(/None/)).toBeInTheDocument();
  });

  it('handles empty project stack', () => {
    const result = createMockAnalysisResult({
      stack: { backend: null, frontend: null, database: null, containerized: false, secondary_languages: [] }
    });
    render(<Sidebar analysisResult={result} selectedFile={null} onClose={mockOnClose} />);
    
    expect(screen.getByText('Stack')).toBeInTheDocument();
    expect(screen.getByText('Containerized:')).toBeInTheDocument();
  });

  it('handles large number of imports', () => {
    const file = createMockFileAnalysis({
      imports: Array.from({ length: 50 }, (_, i) => `module_${i}`)
    });
    render(<Sidebar analysisResult={null} selectedFile={file} onClose={mockOnClose} />);
    
    expect(screen.getByText(/Imports \(50\)/)).toBeInTheDocument();
  });
});
