import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ExportDialog from '../../components/ExportDialog';

const mockOnClose = vi.fn();
const mockOnExport = vi.fn().mockResolvedValue(undefined);

describe('ExportDialog', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockOnExport.mockResolvedValue(undefined);
  });

  it('renders nothing when show is false', () => {
    const { container } = render(
      <ExportDialog show={false} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders modal when show is true', () => {
    render(
      <ExportDialog show={true} projectPath="/test/path" onClose={mockOnClose} onExport={mockOnExport} />
    );
    expect(screen.getByText('Export Graph')).toBeInTheDocument();
  });

  it('displays project path', () => {
    render(
      <ExportDialog show={true} projectPath="/my/project" onClose={mockOnClose} onExport={mockOnExport} />
    );
    expect(screen.getByText(/Project: \/my\/project/)).toBeInTheDocument();
  });

  it('renders PNG, SVG, PDF format buttons', () => {
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );
    expect(screen.getByRole('button', { name: 'PNG' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'SVG' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'PDF' })).toBeInTheDocument();
  });

  it('defaults to PNG format', () => {
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );
    const pngButton = screen.getByRole('button', { name: 'PNG' });
    expect(pngButton).toHaveClass('bg-blue-600');
  });

  it('calls onExport with selected format when export button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'Export as PNG' }));

    expect(mockOnExport).toHaveBeenCalledWith('png');
  });

  it('changes selected format when different format button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'SVG' }));

    const svgButton = screen.getByRole('button', { name: 'SVG' });
    expect(svgButton).toHaveClass('bg-blue-600');
  });

  it('calls onExport with SVG when SVG is selected and export clicked', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'SVG' }));
    await user.click(screen.getByRole('button', { name: 'Export as SVG' }));

    expect(mockOnExport).toHaveBeenCalledWith('svg');
  });

  it('calls onExport with PDF when PDF is selected and export clicked', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'PDF' }));
    await user.click(screen.getByRole('button', { name: 'Export as PDF' }));

    expect(mockOnExport).toHaveBeenCalledWith('pdf');
  });

  it('shows success message after successful export', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'Export as PNG' }));

    expect(await screen.findByText('Successfully exported as PNG')).toBeInTheDocument();
  });

  it('shows error message after failed export', async () => {
    const user = userEvent.setup();
    mockOnExport.mockRejectedValue(new Error('Export failed'));
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'Export as PNG' }));

    expect(await screen.findByText(/Export failed/)).toBeInTheDocument();
  });

  it('calls onClose when cancel button is clicked', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(mockOnClose).toHaveBeenCalledTimes(1);
  });

  it('disables export button while exporting', async () => {
    const user = userEvent.setup();
    let resolveExport: () => void;
    mockOnExport.mockImplementation(() => new Promise<void>((resolve) => { resolveExport = resolve; }));
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'Export as PNG' }));

    expect(screen.getByRole('button', { name: 'Exporting...' })).toBeDisabled();
    resolveExport!();
  });

  it('shows output file name in info text', () => {
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );
    expect(screen.getByText(/architecture\.png/)).toBeInTheDocument();
  });

  it('updates output file name when format changes', async () => {
    const user = userEvent.setup();
    render(
      <ExportDialog show={true} projectPath="/test" onClose={mockOnClose} onExport={mockOnExport} />
    );

    await user.click(screen.getByRole('button', { name: 'SVG' }));

    expect(screen.getByText(/architecture\.svg/)).toBeInTheDocument();
  });
});
