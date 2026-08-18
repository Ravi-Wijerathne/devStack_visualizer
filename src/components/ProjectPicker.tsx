import { open } from "@tauri-apps/plugin-dialog";

interface ProjectPickerProps {
  onProjectSelected: (path: string) => void;
  disabled?: boolean;
}

export default function ProjectPicker({ onProjectSelected, disabled }: ProjectPickerProps) {
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

  return (
    <div className="flex flex-1 w-full h-full items-center justify-center p-6">
      <div className="flex flex-col items-center text-center space-y-6 max-w-md mx-auto">
        <div className="text-6xl select-none">📁</div>
        <div className="space-y-2">
          <h2 className="text-3xl font-bold text-gray-100 tracking-tight">DevStack Visualizer</h2>
          <p className="text-gray-400 text-sm leading-relaxed">
            Analyze your project's architecture and visualize dependencies as an interactive graph.
          </p>
        </div>
        <button
          onClick={handleOpenFolder}
          disabled={disabled}
          className="px-6 py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 disabled:bg-gray-700 disabled:text-gray-500 disabled:cursor-not-allowed text-white rounded-lg font-medium transition-all shadow-lg hover:shadow-blue-500/20 active:scale-95"
        >
          Open Project Folder
        </button>
      </div>
    </div>
  );
}
