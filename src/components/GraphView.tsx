import { useCallback, useEffect, useMemo } from "react";
import {
  ReactFlow,
  Controls,
  Background,
  MiniMap,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
  BackgroundVariant,
  type NodeMouseHandler,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

import type { GraphData } from "../types";

interface GraphViewProps {
  graphData: GraphData | null;
  onNodeClick: (nodeId: string) => void;
}

const NODE_COLORS: Record<string, string> = {
  rust: "#dea584",
  python: "#3776ab",
  js: "#f7df1e",
  other: "#9ca3af",
};

const COMPLEXITY_BORDER: Record<string, string> = {
  Low: "#4ade80",
  Medium: "#fbbf24",
  High: "#ef4444",
  Unknown: "#6b7280",
};

export default function GraphView({ graphData, onNodeClick }: GraphViewProps) {
  const { initialNodes, initialEdges } = useMemo(() => {
    if (!graphData || graphData.nodes.length === 0) {
      return { initialNodes: [], initialEdges: [] };
    }

    // Simple layout: arrange nodes in a grid
    const cols = Math.ceil(Math.sqrt(graphData.nodes.length));

    const nodes: Node[] = graphData.nodes.map((n, i) => ({
      id: n.id,
      position: {
        x: (i % cols) * 250 + 50,
        y: Math.floor(i / cols) * 120 + 50,
      },
      data: {
        label: (
          <div className="text-left">
            <div className="font-semibold text-xs">{n.label}</div>
            <div className="text-[10px] text-gray-400 mt-0.5">
              {n.functions_count}fn · {n.structs_count}st · {n.complexity}
            </div>
          </div>
        ),
      },
      style: {
        backgroundColor: "#1e293b",
        color: "#e2e8f0",
        border: `2px solid ${COMPLEXITY_BORDER[n.complexity] || "#6b7280"}`,
        borderLeft: `4px solid ${NODE_COLORS[n.node_type] || "#9ca3af"}`,
        borderRadius: "8px",
        padding: "8px 12px",
        fontSize: "12px",
        minWidth: "140px",
      },
    }));

    const edges: Edge[] = graphData.edges.map((e, i) => ({
      id: `e-${i}`,
      source: e.source,
      target: e.target,
      animated: e.is_circular,
      style: {
        stroke: e.is_circular ? "#ef4444" : "#4a90d9",
        strokeWidth: e.is_circular ? 2 : 1,
      },
      label: e.is_circular ? "circular" : undefined,
      labelStyle: e.is_circular
        ? { fill: "#ef4444", fontSize: 10 }
        : undefined,
    }));

    return { initialNodes: nodes, initialEdges: edges };
  }, [graphData]);

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Sync state when graphData changes (useNodesState/useEdgesState only use initial value once)
  useEffect(() => {
    setNodes(initialNodes);
  }, [initialNodes, setNodes]);

  useEffect(() => {
    setEdges(initialEdges);
  }, [initialEdges, setEdges]);

  const handleNodeClick: NodeMouseHandler = useCallback(
    (_, node) => {
      onNodeClick(node.id);
    },
    [onNodeClick]
  );

  if (!graphData || graphData.nodes.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-500">
        <p>No graph data. Open and analyze a project to see the dependency graph.</p>
      </div>
    );
  }

  return (
    <div className="h-full w-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={handleNodeClick}
        fitView
        fitViewOptions={{ padding: 0.2 }}
        minZoom={0.1}
        maxZoom={4}
      >
        <Controls
          className="!bg-gray-800 !border-gray-600 !shadow-lg"
          showInteractive={false}
        />
        <MiniMap
          className="!bg-gray-800 !border-gray-600"
          nodeColor="#3b82f6"
          maskColor="rgba(0,0,0,0.5)"
        />
        <Background variant={BackgroundVariant.Dots} gap={16} size={1} color="#334155" />
      </ReactFlow>
    </div>
  );
}
