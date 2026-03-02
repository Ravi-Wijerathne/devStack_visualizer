import { useState } from "react";

interface ExportDialogProps {
  show: boolean;
  projectPath: string;
  onClose: () => void;
  onExport: (format: string) => Promise<void>;
}

export default function ExportDialog({
  show,
  projectPath,
  onClose,
  onExport,
}: ExportDialogProps) {
  const [format, setFormat] = useState("png");
  const [exporting, setExporting] = useState(false);
  const [result, setResult] = useState<string | null>(null);

  if (!show) return null;

  const handleExport = async () => {
    setExporting(true);
    setResult(null);
    try {
      await onExport(format);
      setResult(`Successfully exported as ${format.toUpperCase()}`);
    } catch (err) {
      setResult(`Export failed: ${err}`);
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 border border-gray-600 rounded-lg shadow-2xl w-96">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold text-gray-200">Export Graph</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-200 text-xl leading-none"
          >
            ×
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Export Format
            </label>
            <div className="grid grid-cols-3 gap-2">
              {["png", "svg", "pdf"].map((fmt) => (
                <button
                  key={fmt}
                  onClick={() => setFormat(fmt)}
                  className={`px-3 py-2 rounded text-sm font-medium transition-colors ${
                    format === fmt
                      ? "bg-blue-600 text-white"
                      : "bg-gray-700 text-gray-300 hover:bg-gray-600"
                  }`}
                >
                  {fmt.toUpperCase()}
                </button>
              ))}
            </div>
          </div>

          <div className="text-xs text-gray-500">
            <p>Project: {projectPath}</p>
            <p className="mt-1">
              The graph will be exported to the project directory as{" "}
              <span className="font-mono text-gray-400">architecture.{format}</span>
            </p>
          </div>

          {result && (
            <div
              className={`text-xs p-2 rounded ${
                result.startsWith("Successfully")
                  ? "bg-green-900/30 text-green-400"
                  : "bg-red-900/30 text-red-400"
              }`}
            >
              {result}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-gray-700 flex justify-end gap-2">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-gray-200 text-sm rounded transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleExport}
            disabled={exporting}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded transition-colors"
          >
            {exporting ? "Exporting..." : `Export as ${format.toUpperCase()}`}
          </button>
        </div>
      </div>
    </div>
  );
}
