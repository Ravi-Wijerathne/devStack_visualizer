import type { FileAnalysis, AnalysisResult } from "../types";

interface SidebarProps {
  analysisResult: AnalysisResult | null;
  selectedFile: FileAnalysis | null;
  onClose: () => void;
}

export default function Sidebar({ analysisResult, selectedFile, onClose }: SidebarProps) {
  if (!selectedFile && !analysisResult) {
    return null;
  }

  return (
    <aside className="w-80 bg-gray-800 border-l border-gray-700 flex flex-col shrink-0 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-gray-700">
        <h3 className="text-sm font-semibold text-gray-200 truncate">
          {selectedFile
            ? selectedFile.file.split(/[\\/]/).pop() || "File Details"
            : "Project Overview"}
        </h3>
        <button
          onClick={onClose}
          className="text-gray-400 hover:text-gray-200 text-lg leading-none"
        >
          ×
        </button>
      </div>

      <div className="overflow-y-auto flex-1 p-3 space-y-4">
        {selectedFile ? (
          <FileDetails file={selectedFile} />
        ) : analysisResult ? (
          <ProjectOverview result={analysisResult} />
        ) : null}
      </div>
    </aside>
  );
}

function FileDetails({ file }: { file: FileAnalysis }) {
  return (
    <>
      {/* File path */}
      <Section title="Path">
        <p className="text-xs text-gray-400 break-all font-mono">{file.file}</p>
      </Section>

      {/* Imports */}
      <Section title={`Imports (${file.imports.length})`}>
        {file.imports.length === 0 ? (
          <p className="text-xs text-gray-500 italic">None</p>
        ) : (
          <ul className="space-y-1">
            {file.imports.map((imp, i) => (
              <li key={i} className="text-xs text-gray-300 font-mono bg-gray-900 px-2 py-1 rounded">
                {imp}
              </li>
            ))}
          </ul>
        )}
      </Section>

      {/* Functions */}
      <Section title={`Functions (${file.functions.length})`}>
        {file.functions.length === 0 ? (
          <p className="text-xs text-gray-500 italic">None</p>
        ) : (
          <ul className="space-y-1">
            {file.functions.map((fn_, i) => (
              <li key={i} className="text-xs text-green-300 font-mono bg-gray-900 px-2 py-1 rounded">
                fn {fn_}()
              </li>
            ))}
          </ul>
        )}
      </Section>

      {/* Structs */}
      <Section title={`Structs / Types (${file.structs.length})`}>
        {file.structs.length === 0 ? (
          <p className="text-xs text-gray-500 italic">None</p>
        ) : (
          <ul className="space-y-1">
            {file.structs.map((s, i) => (
              <li key={i} className="text-xs text-yellow-300 font-mono bg-gray-900 px-2 py-1 rounded">
                {s}
              </li>
            ))}
          </ul>
        )}
      </Section>
    </>
  );
}

function ProjectOverview({ result }: { result: AnalysisResult }) {
  return (
    <>
      <Section title="Stack">
        <div className="space-y-1 text-xs text-gray-300">
          {result.stack.backend && (
            <p>
              <span className="text-gray-500">Backend:</span> {result.stack.backend}
            </p>
          )}
          {result.stack.frontend && (
            <p>
              <span className="text-gray-500">Frontend:</span> {result.stack.frontend}
            </p>
          )}
          {result.stack.database && (
            <p>
              <span className="text-gray-500">Database:</span> {result.stack.database}
            </p>
          )}
          <p>
            <span className="text-gray-500">Containerized:</span>{" "}
            {result.stack.containerized ? "Yes" : "No"}
          </p>
        </div>
      </Section>

      <Section title="Summary">
        <div className="grid grid-cols-2 gap-2 text-xs">
          <Stat label="Files Parsed" value={result.files_parsed} />
          <Stat label="Graph Nodes" value={result.total_nodes} />
          <Stat label="Dependencies" value={result.total_edges} />
          <Stat
            label="Circular Deps"
            value={result.circular_dependencies.length}
            warn={result.circular_dependencies.length > 0}
          />
        </div>
      </Section>

      {result.circular_dependencies.length > 0 && (
        <Section title="Circular Dependencies">
          <ul className="space-y-1">
            {result.circular_dependencies.map(([a, b], i) => (
              <li key={i} className="text-xs text-red-400 font-mono bg-gray-900 px-2 py-1 rounded">
                {a} ↔ {b}
              </li>
            ))}
          </ul>
        </Section>
      )}
    </>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h4 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">
        {title}
      </h4>
      {children}
    </div>
  );
}

function Stat({
  label,
  value,
  warn = false,
}: {
  label: string;
  value: number;
  warn?: boolean;
}) {
  return (
    <div className="bg-gray-900 rounded p-2 text-center">
      <div className={`text-lg font-bold ${warn ? "text-red-400" : "text-blue-400"}`}>
        {value}
      </div>
      <div className="text-[10px] text-gray-500">{label}</div>
    </div>
  );
}
