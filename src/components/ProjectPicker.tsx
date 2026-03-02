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
    <div className="flex items-center justify-center h-full">
      <div className="text-center space-y-6">
        <div className="text-6xl">📁</div>
        <h2 className="text-2xl font-bold text-gray-200">DevStack Visualizer</h2>
        <p className="text-gray-400 max-w-md">
          Analyze your project's architecture and visualize dependencies as an interactive graph.
        </p>
        <button
          onClick={handleOpenFolder}
          disabled={disabled}
          className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg font-medium transition-colors shadow-lg"
        >
          Open Project Folder
        </button>
      </div>
    </div>
  );
}
