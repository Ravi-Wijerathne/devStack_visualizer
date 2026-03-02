import { invoke } from "@tauri-apps/api/core";
import { useState, useCallback } from "react";
import type {
  AnalysisResult,
  FileAnalysis,
  ProjectStack,
  ComplexityReport,
  LayerInfo,
} from "../types";

export function useTauriCommands() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const analyzeProject = useCallback(async (path: string): Promise<AnalysisResult | null> => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<AnalysisResult>("analyze_project", { path });
      return result;
    } catch (e) {
      const msg = typeof e === "string" ? e : (e as Error).message;
      setError(msg);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const getFileDetails = useCallback(async (path: string): Promise<FileAnalysis | null> => {
    try {
      return await invoke<FileAnalysis>("get_file_details", { path });
    } catch (e) {
      const msg = typeof e === "string" ? e : (e as Error).message;
      setError(msg);
      return null;
    }
  }, []);

  const exportGraph = useCallback(
    async (path: string, format: string, outputPath: string = ""): Promise<string | null> => {
      try {
        return await invoke<string>("export_graph", {
          path,
          format,
          outputPath,
        });
      } catch (e) {
        const msg = typeof e === "string" ? e : (e as Error).message;
        setError(msg);
        return null;
      }
    },
    []
  );

  const detectStack = useCallback(async (path: string): Promise<ProjectStack | null> => {
    try {
      return await invoke<ProjectStack>("detect_stack", { path });
    } catch (e) {
      const msg = typeof e === "string" ? e : (e as Error).message;
      setError(msg);
      return null;
    }
  }, []);

  const getComplexity = useCallback(async (path: string): Promise<ComplexityReport[] | null> => {
    try {
      return await invoke<ComplexityReport[]>("get_complexity", { path });
    } catch (e) {
      const msg = typeof e === "string" ? e : (e as Error).message;
      setError(msg);
      return null;
    }
  }, []);

  const detectLayers = useCallback(async (path: string): Promise<LayerInfo | null> => {
    try {
      return await invoke<LayerInfo>("detect_layers", { path });
    } catch (e) {
      const msg = typeof e === "string" ? e : (e as Error).message;
      setError(msg);
      return null;
    }
  }, []);

  return {
    loading,
    error,
    setError,
    analyzeProject,
    getFileDetails,
    exportGraph,
    detectStack,
    getComplexity,
    detectLayers,
  };
}
