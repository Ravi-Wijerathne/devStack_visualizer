import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import ProjectPicker from '../../components/ProjectPicker';
import { open } from '@tauri-apps/plugin-dialog';

vi.mock('@tauri-apps/plugin-dialog');

const mockOnProjectSelected = vi.fn();

describe('ProjectPicker', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders welcome message and button', () => {
    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    expect(screen.getByText('DevStack Visualizer')).toBeInTheDocument();
    expect(screen.getByText(/Analyze your project/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /open project/i })).toBeInTheDocument();
  });

  it('renders folder emoji', () => {
    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );
    expect(screen.getByText('📁')).toBeInTheDocument();
  });

  it('button is disabled when disabled prop is true', () => {
    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={true} />
      </MemoryRouter>
    );
    expect(screen.getByRole('button', { name: /open project/i })).toBeDisabled();
  });

  it('button is enabled when disabled prop is false', () => {
    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );
    expect(screen.getByRole('button', { name: /open project/i })).not.toBeDisabled();
  });

  it('calls open dialog when button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    await user.click(screen.getByRole('button', { name: /open project/i }));

    expect(open).toHaveBeenCalledWith({
      directory: true,
      multiple: false,
      title: 'Select Project Directory',
    });
  });

  it('does not call onProjectSelected when dialog returns null', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue(null);

    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    await user.click(screen.getByRole('button', { name: /open project/i }));

    expect(mockOnProjectSelected).not.toHaveBeenCalled();
  });

  it('calls onProjectSelected with path when dialog returns a path', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/path/to/project');

    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    await user.click(screen.getByRole('button', { name: /open project/i }));

    expect(mockOnProjectSelected).toHaveBeenCalledWith('/path/to/project');
  });

  it('does not call onProjectSelected when dialog returns array (multiple selection)', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue(['/path/a', '/path/b']);

    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    await user.click(screen.getByRole('button', { name: /open project/i }));

    expect(mockOnProjectSelected).not.toHaveBeenCalled();
  });

  it('handles dialog errors gracefully', async () => {
    const user = userEvent.setup();
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.mocked(open).mockRejectedValue(new Error('Dialog error'));

    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={false} />
      </MemoryRouter>
    );

    await user.click(screen.getByRole('button', { name: /open project/i }));

    expect(consoleError).toHaveBeenCalledWith('Failed to open folder dialog:', expect.any(Error));
    expect(mockOnProjectSelected).not.toHaveBeenCalled();
    consoleError.mockRestore();
  });

  it('button is not clickable when disabled even if clicked programmatically', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/path/to/project');

    render(
      <MemoryRouter>
        <ProjectPicker onProjectSelected={mockOnProjectSelected} disabled={true} />
      </MemoryRouter>
    );

    const button = screen.getByRole('button', { name: /open project/i });
    await user.click(button);

    expect(open).not.toHaveBeenCalled();
  });
});
