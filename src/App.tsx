import { useState, useCallback } from "react";
import { useTauriCommands } from "./hooks/useTauriCommands";
import Toolbar from "./components/Toolbar";
import ProjectPicker from "./components/ProjectPicker";
import GraphView from "./components/GraphView";
import Sidebar from "./components/Sidebar";
import SettingsPanel from "./components/SettingsPanel";
import ExportDialog from "./components/ExportDialog";
import type { AnalysisResult, FileAnalysis } from "./types";

export default function App() {
  const [projectPath, setProjectPath] = useState<string | null>(null);
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [selectedFile, setSelectedFile] = useState<FileAnalysis | null>(null);
  const [showSidebar, setShowSidebar] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showExport, setShowExport] = useState(false);

  const { loading, error, setError, analyzeProject, getFileDetails, exportGraph } =
    useTauriCommands();

  const handleProjectSelected = useCallback(
    async (path: string) => {
      setProjectPath(path);
      setSelectedFile(null);
      setShowSidebar(false);
      setError(null);

      const result = await analyzeProject(path);
      if (result) {
        setAnalysisResult(result);
        setShowSidebar(true);
      }
    },
    [analyzeProject, setError]
  );

  const handleReanalyze = useCallback(async () => {
    if (!projectPath) return;
    setSelectedFile(null);

    const result = await analyzeProject(projectPath);
    if (result) {
      setAnalysisResult(result);
    }
  }, [projectPath, analyzeProject]);

  const handleNodeClick = useCallback(
    async (nodeId: string) => {
      if (!analysisResult) return;

      // Find the file analysis from the result
      const fileAnalysis = analysisResult.file_analyses.find((f) => {
        const path = f.file.replace(/\\/g, "/");
        return path.endsWith(nodeId);
      });

      if (fileAnalysis) {
        setSelectedFile(fileAnalysis);
        setShowSidebar(true);
      } else {
        // Try to get details from backend
        if (projectPath) {
          const details = await getFileDetails(nodeId);
          if (details) {
            setSelectedFile(details);
            setShowSidebar(true);
          }
        }
      }
    },
    [analysisResult, projectPath, getFileDetails]
  );

  const handleExport = useCallback(
    async (format: string) => {
      if (!projectPath) return;
      await exportGraph(projectPath, format);
    },
    [projectPath, exportGraph]
  );

  const handleCloseSidebar = useCallback(() => {
    setSelectedFile(null);
    // Keep sidebar open with overview if we have results  
    if (!analysisResult) {
      setShowSidebar(false);
    }
  }, [analysisResult]);

  return (
    <div className="flex flex-col h-screen bg-gray-900">
      <Toolbar
        projectPath={projectPath}
        onProjectSelected={handleProjectSelected}
        onReanalyze={handleReanalyze}
        onExport={() => setShowExport(true)}
        onToggleSettings={() => setShowSettings(!showSettings)}
        loading={loading}
        filesParsed={analysisResult?.files_parsed ?? 0}
        totalNodes={analysisResult?.total_nodes ?? 0}
        totalEdges={analysisResult?.total_edges ?? 0}
      />

      {error && (
        <div className="bg-red-900/30 border-b border-red-800 px-4 py-2 text-sm text-red-400 flex items-center justify-between">
          <span>{error}</span>
          <button onClick={() => setError(null)} className="text-red-400 hover:text-red-200">
            ×
          </button>
        </div>
      )}

      <main className="flex flex-1 overflow-hidden w-full">
        {!projectPath ? (
          <ProjectPicker onProjectSelected={handleProjectSelected} disabled={loading} />
        ) : (
          <div className="flex flex-1 overflow-hidden w-full">
            <div className="flex-1">
              <GraphView
                graphData={analysisResult?.graph_data ?? null}
                onNodeClick={handleNodeClick}
              />
            </div>

            {showSidebar && (
              <Sidebar
                analysisResult={analysisResult}
                selectedFile={selectedFile}
                onClose={handleCloseSidebar}
              />
            )}
          </div>
        )}
      </main>

      <SettingsPanel show={showSettings} onClose={() => setShowSettings(false)} />

      <ExportDialog
        show={showExport}
        projectPath={projectPath ?? ""}
        onClose={() => setShowExport(false)}
        onExport={handleExport}
      />
    </div>
  );
}
