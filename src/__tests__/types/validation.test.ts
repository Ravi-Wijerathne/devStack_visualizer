import { describe, it, expect } from 'vitest';
import type {
  ProjectStack,
  FileAnalysis,
  GraphNode,
  GraphEdge,
  GraphData,
  AnalysisResult,
  ComplexityReport,
  LayerInfo,
} from '../../types';

describe('Type Validation', () => {
  describe('ProjectStack', () => {
    it('accepts valid ProjectStack with all fields', () => {
      const stack: ProjectStack = {
        backend: 'rust',
        frontend: 'react',
        database: 'postgresql',
        containerized: true,
        secondary_languages: [],
      };
      expect(stack.backend).toBe('rust');
      expect(stack.frontend).toBe('react');
      expect(stack.database).toBe('postgresql');
      expect(stack.containerized).toBe(true);
    });

    it('accepts ProjectStack with null fields', () => {
      const stack: ProjectStack = {
        backend: null,
        frontend: null,
        database: null,
        containerized: false,
        secondary_languages: [],
      };
      expect(stack.backend).toBeNull();
      expect(stack.frontend).toBeNull();
      expect(stack.database).toBeNull();
      expect(stack.containerized).toBe(false);
    });

    it('accepts partial ProjectStack', () => {
      const stack: ProjectStack = {
        backend: 'python',
        frontend: null,
        database: 'sqlite',
        containerized: false,
        secondary_languages: [],
      };
      expect(stack.backend).toBe('python');
    });
  });

  describe('FileAnalysis', () => {
    it('accepts valid FileAnalysis', () => {
      const file: FileAnalysis = {
        file: '/src/main.rs',
        imports: ['std::fmt', 'std::io'],
        functions: ['main', 'helper'],
        structs: ['MyStruct', 'AnotherStruct'],
      };
      expect(file.file).toBe('/src/main.rs');
      expect(file.imports).toHaveLength(2);
      expect(file.functions).toHaveLength(2);
      expect(file.structs).toHaveLength(2);
    });

    it('accepts FileAnalysis with empty arrays', () => {
      const file: FileAnalysis = {
        file: '/src/empty.rs',
        imports: [],
        functions: [],
        structs: [],
      };
      expect(file.imports).toHaveLength(0);
      expect(file.functions).toHaveLength(0);
      expect(file.structs).toHaveLength(0);
    });
  });

  describe('GraphNode', () => {
    it('accepts valid GraphNode with rust type', () => {
      const node: GraphNode = {
        id: 'main.rs',
        label: 'main',
        file_path: '/src/main.rs',
        node_type: 'rust',
        complexity: 'Medium',
        functions_count: 5,
        structs_count: 2,
      };
      expect(node.node_type).toBe('rust');
      expect(node.complexity).toBe('Medium');
    });

    it('accepts GraphNode with all node types', () => {
      const types: GraphNode['node_type'][] = ['rust', 'python', 'js', 'other'];
      types.forEach((type) => {
        const node: GraphNode = {
          id: `file.${type === 'js' ? 'ts' : type === 'rust' ? 'rs' : type === 'python' ? 'py' : 'txt'}`,
          label: 'test',
          file_path: '/test',
          node_type: type,
          complexity: 'Unknown',
          functions_count: 0,
          structs_count: 0,
        };
        expect(node.node_type).toBe(type);
      });
    });

    it('accepts GraphNode with different complexity levels', () => {
      const complexities = ['Low', 'Medium', 'High', 'Unknown'];
      complexities.forEach((complexity) => {
        const node: GraphNode = {
          id: 'test',
          label: 'test',
          file_path: '/test',
          node_type: 'rust',
          complexity,
          functions_count: 0,
          structs_count: 0,
        };
        expect(node.complexity).toBe(complexity);
      });
    });
  });

  describe('GraphEdge', () => {
    it('accepts valid GraphEdge', () => {
      const edge: GraphEdge = {
        source: 'a.rs',
        target: 'b.rs',
        is_circular: false,
      };
      expect(edge.source).toBe('a.rs');
      expect(edge.target).toBe('b.rs');
      expect(edge.is_circular).toBe(false);
    });

    it('accepts circular GraphEdge', () => {
      const edge: GraphEdge = {
        source: 'a.rs',
        target: 'b.rs',
        is_circular: true,
      };
      expect(edge.is_circular).toBe(true);
    });
  });

  describe('GraphData', () => {
    it('accepts valid GraphData with nodes and edges', () => {
      const graphData: GraphData = {
        nodes: [
          {
            id: 'main.rs',
            label: 'main',
            file_path: '/src/main.rs',
            node_type: 'rust',
            complexity: 'Low',
            functions_count: 1,
            structs_count: 0,
          },
        ],
        edges: [
          {
            source: 'main.rs',
            target: 'lib.rs',
            is_circular: false,
          },
        ],
      };
      expect(graphData.nodes).toHaveLength(1);
      expect(graphData.edges).toHaveLength(1);
    });

    it('accepts empty GraphData', () => {
      const graphData: GraphData = {
        nodes: [],
        edges: [],
      };
      expect(graphData.nodes).toHaveLength(0);
      expect(graphData.edges).toHaveLength(0);
    });

    it('accepts GraphData with circular dependencies', () => {
      const graphData: GraphData = {
        nodes: [
          { id: 'a.rs', label: 'a', file_path: '/a.rs', node_type: 'rust', complexity: 'Low', functions_count: 1, structs_count: 0 },
          { id: 'b.rs', label: 'b', file_path: '/b.rs', node_type: 'rust', complexity: 'Low', functions_count: 1, structs_count: 0 },
        ],
        edges: [
          { source: 'a.rs', target: 'b.rs', is_circular: true },
          { source: 'b.rs', target: 'a.rs', is_circular: true },
        ],
      };
      expect(graphData.edges.filter((e) => e.is_circular)).toHaveLength(2);
    });
  });

  describe('AnalysisResult', () => {
    it('accepts valid AnalysisResult', () => {
      const result: AnalysisResult = {
        stack: { backend: 'rust', frontend: null, database: null, containerized: false, secondary_languages: [] },
        files_parsed: 10,
        total_nodes: 5,
        total_edges: 8,
        circular_dependencies: [],
        graph_data: { nodes: [], edges: [] },
        file_analyses: [],
      };
      expect(result.files_parsed).toBe(10);
      expect(result.total_nodes).toBe(5);
      expect(result.total_edges).toBe(8);
    });

    it('accepts AnalysisResult with circular dependencies', () => {
      const result: AnalysisResult = {
        stack: { backend: 'rust', frontend: null, database: null, containerized: false, secondary_languages: [] },
        files_parsed: 5,
        total_nodes: 3,
        total_edges: 4,
        circular_dependencies: [['a.rs', 'b.rs'], ['c.rs', 'd.rs']],
        graph_data: { nodes: [], edges: [] },
        file_analyses: [],
      };
      expect(result.circular_dependencies).toHaveLength(2);
    });
  });

  describe('ComplexityReport', () => {
    it('accepts valid ComplexityReport', () => {
      const report: ComplexityReport = {
        file: '/src/main.rs',
        complexity: 'Medium',
        functions_count: 10,
        structs_count: 3,
        score: 75,
      };
      expect(report.score).toBe(75);
    });

    it('accepts ComplexityReport with various scores', () => {
      const scores = [0, 50, 100, 999];
      scores.forEach((score) => {
        const report: ComplexityReport = {
          file: '/test.rs',
          complexity: 'Unknown',
          functions_count: 0,
          structs_count: 0,
          score,
        };
        expect(report.score).toBe(score);
      });
    });
  });

  describe('LayerInfo', () => {
    it('accepts valid LayerInfo', () => {
      const layers: LayerInfo = {
        controllers: ['user_controller.rs', 'product_controller.rs'],
        services: ['user_service.rs'],
        models: ['user.rs', 'product.rs', 'order.rs'],
        others: ['main.rs', 'lib.rs'],
      };
      expect(layers.controllers).toHaveLength(2);
      expect(layers.services).toHaveLength(1);
      expect(layers.models).toHaveLength(3);
      expect(layers.others).toHaveLength(2);
    });

    it('accepts empty LayerInfo', () => {
      const layers: LayerInfo = {
        controllers: [],
        services: [],
        models: [],
        others: [],
      };
      expect(layers.controllers).toHaveLength(0);
    });

    it('accepts LayerInfo with partial layers', () => {
      const layers: LayerInfo = {
        controllers: ['ctrl.rs'],
        services: [],
        models: [],
        others: ['main.rs'],
      };
      expect(layers.controllers).toHaveLength(1);
      expect(layers.services).toHaveLength(0);
    });
  });
});
