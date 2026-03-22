import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useTauriCommands } from '../../hooks/useTauriCommands';
import { invoke } from '@tauri-apps/api/core';
import type { AnalysisResult, FileAnalysis } from '../../types';

vi.mock('@tauri-apps/api/core');

const mockInvoke = vi.mocked(invoke);

describe('useTauriCommands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('analyzeProject', () => {
    it('returns AnalysisResult on success', async () => {
      const mockResult: AnalysisResult = {
        stack: { backend: 'rust', frontend: null, database: null, containerized: false, secondary_languages: [] },
        files_parsed: 10,
        total_nodes: 5,
        total_edges: 8,
        circular_dependencies: [],
        graph_data: { nodes: [], edges: [] },
        file_analyses: [],
      };
      mockInvoke.mockResolvedValue(mockResult);

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.analyzeProject('/test/path');
        expect(response).toEqual(mockResult);
      });

      expect(mockInvoke).toHaveBeenCalledWith('analyze_project', { path: '/test/path' });
    });

    it('sets loading state during analysis', async () => {
      mockInvoke.mockImplementation(() => new Promise((resolve) => setTimeout(() => resolve({}), 100)));

      const { result } = renderHook(() => useTauriCommands());

      expect(result.current.loading).toBe(false);

      act(() => {
        result.current.analyzeProject('/test/path');
      });

      await waitFor(() => {
        expect(result.current.loading).toBe(true);
      });

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
      });
    });

    it('returns null and sets error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Analysis failed'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.analyzeProject('/test/path');
        expect(response).toBeNull();
      });

      expect(result.current.error).toBe('Analysis failed');
    });

    it('clears previous error before new analysis', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('Previous error'));
      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        await result.current.analyzeProject('/test/path');
      });
      expect(result.current.error).toBe('Previous error');

      mockInvoke.mockResolvedValue({
        stack: { backend: null, frontend: null, database: null, containerized: false, secondary_languages: [] },
        files_parsed: 0,
        total_nodes: 0,
        total_edges: 0,
        circular_dependencies: [],
        graph_data: { nodes: [], edges: [] },
        file_analyses: [],
      });

      await act(async () => {
        await result.current.analyzeProject('/test/path');
      });

      expect(result.current.error).toBeNull();
    });
  });

  describe('getFileDetails', () => {
    it('returns FileAnalysis on success', async () => {
      const mockFileDetails: FileAnalysis = {
        file: '/test/main.rs',
        imports: ['std::fmt'],
        functions: ['main'],
        structs: ['MyStruct'],
      };
      mockInvoke.mockResolvedValue(mockFileDetails);

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.getFileDetails('/test/main.rs');
        expect(response).toEqual(mockFileDetails);
      });

      expect(mockInvoke).toHaveBeenCalledWith('get_file_details', { path: '/test/main.rs' });
    });

    it('returns null and sets error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('File not found'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.getFileDetails('/test/main.rs');
        expect(response).toBeNull();
      });

      expect(result.current.error).toBe('File not found');
    });

    it('does not set loading state', async () => {
      mockInvoke.mockResolvedValue({
        file: '/test.rs',
        imports: [],
        functions: [],
        structs: [],
      });

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        await result.current.getFileDetails('/test.rs');
      });

      expect(result.current.loading).toBe(false);
    });
  });

  describe('exportGraph', () => {
    it('calls export_graph command', async () => {
      mockInvoke.mockResolvedValue('/output/graph.png');

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.exportGraph('/test', 'png');
        expect(response).toBe('/output/graph.png');
      });

      expect(mockInvoke).toHaveBeenCalledWith('export_graph', {
        path: '/test',
        format: 'png',
        outputPath: '',
      });
    });

    it('returns null and sets error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Export failed'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.exportGraph('/test', 'svg');
        expect(response).toBeNull();
      });

      expect(result.current.error).toBe('Export failed');
    });

    it('passes custom output path', async () => {
      mockInvoke.mockResolvedValue('/custom/path.png');

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        await result.current.exportGraph('/test', 'png', '/custom/path.png');
      });

      expect(mockInvoke).toHaveBeenCalledWith('export_graph', {
        path: '/test',
        format: 'png',
        outputPath: '/custom/path.png',
      });
    });
  });

  describe('detectStack', () => {
    it('returns ProjectStack on success', async () => {
      const mockStack = {
        backend: 'rust',
        frontend: 'react',
        database: 'postgresql',
        containerized: true,
      };
      mockInvoke.mockResolvedValue(mockStack);

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.detectStack('/test');
        expect(response).toEqual(mockStack);
      });

      expect(mockInvoke).toHaveBeenCalledWith('detect_stack', { path: '/test' });
    });

    it('returns null on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Detection failed'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.detectStack('/test');
        expect(response).toBeNull();
      });

      expect(result.current.error).toBe('Detection failed');
    });
  });

  describe('getComplexity', () => {
    it('returns ComplexityReport array on success', async () => {
      const mockReports = [
        { file: '/test/a.rs', complexity: 'Medium', functions_count: 5, structs_count: 2, score: 50 },
        { file: '/test/b.rs', complexity: 'Low', functions_count: 2, structs_count: 1, score: 20 },
      ];
      mockInvoke.mockResolvedValue(mockReports);

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.getComplexity('/test');
        expect(response).toEqual(mockReports);
      });

      expect(mockInvoke).toHaveBeenCalledWith('get_complexity', { path: '/test' });
    });

    it('returns null on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Complexity analysis failed'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.getComplexity('/test');
        expect(response).toBeNull();
      });
    });
  });

  describe('detectLayers', () => {
    it('returns LayerInfo on success', async () => {
      const mockLayers = {
        controllers: ['/src/controllers/user.rs'],
        services: ['/src/services/user_service.rs'],
        models: ['/src/models/user.rs'],
        others: ['/src/main.rs'],
      };
      mockInvoke.mockResolvedValue(mockLayers);

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.detectLayers('/test');
        expect(response).toEqual(mockLayers);
      });

      expect(mockInvoke).toHaveBeenCalledWith('detect_layers', { path: '/test' });
    });

    it('returns null on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Layer detection failed'));

      const { result } = renderHook(() => useTauriCommands());

      await act(async () => {
        const response = await result.current.detectLayers('/test');
        expect(response).toBeNull();
      });
    });
  });

  describe('setError', () => {
    it('allows setting error manually', () => {
      const { result } = renderHook(() => useTauriCommands());

      act(() => {
        result.current.setError('Manual error');
      });

      expect(result.current.error).toBe('Manual error');
    });

    it('allows clearing error', () => {
      const { result } = renderHook(() => useTauriCommands());

      act(() => {
        result.current.setError('Some error');
      });
      expect(result.current.error).toBe('Some error');

      act(() => {
        result.current.setError(null);
      });
      expect(result.current.error).toBeNull();
    });
  });
});
