interface SettingsPanelProps {
  show: boolean;
  onClose: () => void;
}

export default function SettingsPanel({ show, onClose }: SettingsPanelProps) {
  if (!show) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 border border-gray-600 rounded-lg shadow-2xl w-96 max-h-[80vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold text-gray-200">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-200 text-xl leading-none"
          >
            ×
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-6">
          {/* Language Filter */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Language Filter
            </label>
            <div className="space-y-2">
              {["Rust", "Python", "JavaScript/TypeScript"].map((lang) => (
                <label key={lang} className="flex items-center gap-2 text-sm text-gray-400">
                  <input
                    type="checkbox"
                    defaultChecked
                    className="rounded border-gray-600 bg-gray-700 text-blue-500"
                  />
                  {lang}
                </label>
              ))}
            </div>
          </div>

          {/* Graph Layout */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Graph Layout Direction
            </label>
            <select className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm text-gray-200">
              <option value="LR">Left → Right</option>
              <option value="TB">Top → Bottom</option>
              <option value="RL">Right → Left</option>
              <option value="BT">Bottom → Top</option>
            </select>
          </div>

          {/* Theme */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">Theme</label>
            <select className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm text-gray-200">
              <option value="dark">Dark</option>
              <option value="light">Light (Coming Soon)</option>
            </select>
          </div>
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-gray-700 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded transition-colors"
          >
            Done
          </button>
        </div>
      </div>
    </div>
  );
}
