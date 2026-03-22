import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Toolbar from '../../components/Toolbar';
import { open } from '@tauri-apps/plugin-dialog';

vi.mock('@tauri-apps/plugin-dialog');

const mockOnProjectSelected = vi.fn();
const mockOnReanalyze = vi.fn();
const mockOnExport = vi.fn();
const mockOnToggleSettings = vi.fn();

describe('Toolbar Snapshots', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders logo and title', () => {
    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText('⚡ DevStack')).toBeInTheDocument();
  });

  it('renders Open button', () => {
    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByRole('button', { name: /open/i })).toBeInTheDocument();
  });

  it('displays project name when path is provided', () => {
    render(
      <Toolbar
        projectPath="/path/to/my-project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText('my-project')).toBeInTheDocument();
  });

  it('displays analysis stats when project is loaded', () => {
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={25}
        totalNodes={10}
        totalEdges={15}
      />
    );

    expect(screen.getByText('25 files')).toBeInTheDocument();
    expect(screen.getByText('10 nodes')).toBeInTheDocument();
    expect(screen.getByText('15 edges')).toBeInTheDocument();
  });

  it('shows Refresh and Export buttons when project is loaded', () => {
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={10}
        totalNodes={5}
        totalEdges={8}
      />
    );

    expect(screen.getByRole('button', { name: /refresh/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /export/i })).toBeInTheDocument();
  });

  it('shows loading indicator when loading', () => {
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={true}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText(/analyzing/i)).toBeInTheDocument();
  });

  it('disables buttons when loading', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/new/path');

    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={true}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    const openButton = screen.getByRole('button', { name: /open/i });
    expect(openButton).toBeDisabled();

    await user.click(openButton);
    expect(open).not.toHaveBeenCalled();
  });

  it('renders Settings button', () => {
    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByRole('button', { name: '⚙️' })).toBeInTheDocument();
  });

  it('calls onToggleSettings when Settings button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    await user.click(screen.getByRole('button', { name: '⚙️' }));
    expect(mockOnToggleSettings).toHaveBeenCalledTimes(1);
  });

  it('calls onReanalyze when Refresh button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={10}
        totalNodes={5}
        totalEdges={8}
      />
    );

    await user.click(screen.getByRole('button', { name: /refresh/i }));
    expect(mockOnReanalyze).toHaveBeenCalledTimes(1);
  });

  it('calls onExport when Export button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={10}
        totalNodes={5}
        totalEdges={8}
      />
    );

    await user.click(screen.getByRole('button', { name: /export/i }));
    expect(mockOnExport).toHaveBeenCalledTimes(1);
  });

  it('calls open dialog when Open button is clicked', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/new/project');

    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    await user.click(screen.getByRole('button', { name: /open/i }));

    expect(open).toHaveBeenCalledWith({
      directory: true,
      multiple: false,
      title: 'Select Project Directory',
    });
  });

  it('calls onProjectSelected with path from dialog', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/selected/project');

    render(
      <Toolbar
        projectPath={null}
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    await user.click(screen.getByRole('button', { name: /open/i }));

    expect(mockOnProjectSelected).toHaveBeenCalledWith('/selected/project');
  });
});

describe('Toolbar Edge Cases', () => {
  it('handles project path with Windows backslashes', () => {
    render(
      <Toolbar
        projectPath="C:\\Users\\Developer\\Projects\\my-app"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText('my-app')).toBeInTheDocument();
  });

  it('handles project path with forward slashes', () => {
    render(
      <Toolbar
        projectPath="/home/developer/projects/my-app"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText('my-app')).toBeInTheDocument();
  });

  it('displays zero stats correctly', () => {
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={0}
        totalNodes={0}
        totalEdges={0}
      />
    );

    expect(screen.getByText('0 files')).toBeInTheDocument();
    expect(screen.getByText('0 nodes')).toBeInTheDocument();
    expect(screen.getByText('0 edges')).toBeInTheDocument();
  });

  it('displays large numbers correctly', () => {
    render(
      <Toolbar
        projectPath="/path/to/project"
        onProjectSelected={mockOnProjectSelected}
        onReanalyze={mockOnReanalyze}
        onExport={mockOnExport}
        onToggleSettings={mockOnToggleSettings}
        loading={false}
        filesParsed={10000}
        totalNodes={5000}
        totalEdges={7500}
      />
    );

    expect(screen.getByText('10000 files')).toBeInTheDocument();
    expect(screen.getByText('5000 nodes')).toBeInTheDocument();
    expect(screen.getByText('7500 edges')).toBeInTheDocument();
  });
});
