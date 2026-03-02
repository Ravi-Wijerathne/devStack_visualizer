import { open } from "@tauri-apps/plugin-dialog";

interface ToolbarProps {
  projectPath: string | null;
  onProjectSelected: (path: string) => void;
  onReanalyze: () => void;
  onExport: () => void;
  onToggleSettings: () => void;
  loading: boolean;
  filesParsed: number;
  totalNodes: number;
  totalEdges: number;
}

export default function Toolbar({
  projectPath,
  onProjectSelected,
  onReanalyze,
  onExport,
  onToggleSettings,
  loading,
  filesParsed,
  totalNodes,
  totalEdges,
}: ToolbarProps) {
  const handleOpenFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Project Directory",
      });
      if (selected && typeof selected === "string") {
        onProjectSelected(selected);
      }
    } catch (err) {
      console.error("Failed to open folder dialog:", err);
    }
  };

  const projectName = projectPath
    ? projectPath.split(/[\\/]/).filter(Boolean).pop() || projectPath
    : null;

  return (
    <header className="bg-gray-800 border-b border-gray-700 px-4 py-2 flex items-center justify-between shrink-0">
      <div className="flex items-center gap-3">
        <span className="text-lg font-bold text-blue-400">⚡ DevStack</span>

        {projectName && (
          <span className="text-sm text-gray-400 border-l border-gray-600 pl-3">
            {projectName}
          </span>
        )}

        {projectPath && !loading && (
          <div className="flex items-center gap-2 text-xs text-gray-500 border-l border-gray-600 pl-3">
            <span>{filesParsed} files</span>
            <span>·</span>
            <span>{totalNodes} nodes</span>
            <span>·</span>
            <span>{totalEdges} edges</span>
          </div>
        )}

        {loading && (
          <span className="text-xs text-yellow-400 animate-pulse">Analyzing...</span>
        )}
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={handleOpenFolder}
          disabled={loading}
          className="px-3 py-1.5 text-sm bg-gray-700 hover:bg-gray-600 disabled:opacity-50 text-gray-200 rounded transition-colors"
          title="Open Project"
        >
          📂 Open
        </button>

        {projectPath && (
          <>
            <button
              onClick={onReanalyze}
              disabled={loading}
              className="px-3 py-1.5 text-sm bg-gray-700 hover:bg-gray-600 disabled:opacity-50 text-gray-200 rounded transition-colors"
              title="Re-analyze"
            >
              🔄 Refresh
            </button>

            <button
              onClick={onExport}
              disabled={loading}
              className="px-3 py-1.5 text-sm bg-gray-700 hover:bg-gray-600 disabled:opacity-50 text-gray-200 rounded transition-colors"
              title="Export Graph"
            >
              📤 Export
            </button>
          </>
        )}

        <button
          onClick={onToggleSettings}
          className="px-3 py-1.5 text-sm bg-gray-700 hover:bg-gray-600 text-gray-200 rounded transition-colors"
          title="Settings"
        >
          ⚙️
        </button>
      </div>
    </header>
  );
}
