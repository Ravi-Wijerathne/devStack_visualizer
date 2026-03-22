import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import SettingsPanel from '../../components/SettingsPanel';

const mockOnClose = vi.fn();

describe('SettingsPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders nothing when show is false', () => {
    const { container } = render(<SettingsPanel show={false} onClose={mockOnClose} />);
    expect(container.firstChild).toBeNull();
  });

  it('renders modal when show is true', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });

  it('renders language filter checkboxes', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    expect(screen.getByLabelText('Rust')).toBeInTheDocument();
    expect(screen.getByLabelText('Python')).toBeInTheDocument();
    expect(screen.getByLabelText('JavaScript/TypeScript')).toBeInTheDocument();
  });

  it('renders graph layout dropdown', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    const dropdowns = screen.getAllByRole('combobox');
    expect(dropdowns.length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText('Left → Right')).toBeInTheDocument();
    expect(screen.getByText('Top → Bottom')).toBeInTheDocument();
  });

  it('renders graph layout options', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    expect(screen.getByText('Left → Right')).toBeInTheDocument();
    expect(screen.getByText('Top → Bottom')).toBeInTheDocument();
    expect(screen.getByText('Right → Left')).toBeInTheDocument();
    expect(screen.getByText('Bottom → Top')).toBeInTheDocument();
  });

  it('renders theme dropdown', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    const dropdowns = screen.getAllByRole('combobox');
    expect(dropdowns.length).toBe(2);
  });

  it('renders done button', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    expect(screen.getByRole('button', { name: 'Done' })).toBeInTheDocument();
  });

  it('calls onClose when done button is clicked', async () => {
    const user = userEvent.setup();
    render(<SettingsPanel show={true} onClose={mockOnClose} />);

    await user.click(screen.getByRole('button', { name: 'Done' }));

    expect(mockOnClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when close button (×) is clicked', async () => {
    const user = userEvent.setup();
    render(<SettingsPanel show={true} onClose={mockOnClose} />);

    const closeButton = screen.getByRole('button', { name: '×' });
    await user.click(closeButton);

    expect(mockOnClose).toHaveBeenCalledTimes(1);
  });

  it('renders all section labels', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    expect(screen.getByText('Language Filter')).toBeInTheDocument();
    expect(screen.getByText('Graph Layout Direction')).toBeInTheDocument();
    expect(screen.getByText('Theme')).toBeInTheDocument();
  });

  it('has modal structure with overlay', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    const overlay = document.body.querySelector('.fixed.inset-0');
    expect(overlay).toBeInTheDocument();
  });

  it('has two select elements for graph layout and theme', () => {
    render(<SettingsPanel show={true} onClose={mockOnClose} />);
    const selects = screen.getAllByRole('combobox');
    expect(selects).toHaveLength(2);
  });
});
