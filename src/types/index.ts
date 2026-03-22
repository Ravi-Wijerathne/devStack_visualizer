// TypeScript types matching the Rust backend structs

export interface SecondaryLanguage {
  name: string;
  description: string;
}

export interface ProjectStack {
  backend: string | null;
  frontend: string | null;
  database: string | null;
  containerized: boolean;
  secondary_languages: SecondaryLanguage[];
}

export interface FileAnalysis {
  file: string;
  imports: string[];
  functions: string[];
  structs: string[];
}

export interface GraphNode {
  id: string;
  label: string;
  file_path: string;
  node_type: "rust" | "python" | "js" | "other";
  complexity: string;
  functions_count: number;
  structs_count: number;
}

export interface GraphEdge {
  source: string;
  target: string;
  is_circular: boolean;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export interface AnalysisResult {
  stack: ProjectStack;
  files_parsed: number;
  total_nodes: number;
  total_edges: number;
  circular_dependencies: [string, string][];
  graph_data: GraphData;
  file_analyses: FileAnalysis[];
}

export interface ComplexityReport {
  file: string;
  complexity: string;
  functions_count: number;
  structs_count: number;
  score: number;
}

export interface LayerInfo {
  controllers: string[];
  services: string[];
  models: string[];
  others: string[];
}
