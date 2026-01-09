'use client';

import { useCallback, useMemo } from 'react';
import ReactFlow, {
  Node,
  Edge,
  Background,
  Controls,
  MiniMap,
  Handle,
  Position,
  NodeProps,
} from 'reactflow';
import 'reactflow/dist/style.css';
import type { DecisionTree } from '@/lib/types';

interface DecisionTreeViewProps {
  data: DecisionTree;
}

function DecisionNodeComponent({ data }: NodeProps) {
  const bgColor = data.outcome
    ? 'bg-green-500/20 border-green-500'
    : 'bg-red-500/20 border-red-500';

  const iconColor = data.outcome ? 'text-green-400' : 'text-red-400';

  const icon = {
    identity: (
      <svg className={`w-5 h-5 ${iconColor}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
      </svg>
    ),
    policy: (
      <svg className={`w-5 h-5 ${iconColor}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
      </svg>
    ),
    action: (
      <svg className={`w-5 h-5 ${iconColor}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
      </svg>
    ),
  }[data.nodeType] || null;

  return (
    <div className={`px-4 py-3 rounded-lg border-2 ${bgColor} min-w-[180px]`}>
      <Handle type="target" position={Position.Top} className="!bg-slate-500" />
      <div className="flex items-center gap-2">
        {icon}
        <div>
          <div className="text-xs text-slate-400 uppercase">{data.nodeType}</div>
          <div className="text-sm text-white font-medium">{data.label}</div>
        </div>
      </div>
      <Handle type="source" position={Position.Bottom} className="!bg-slate-500" />
    </div>
  );
}

const nodeTypes = {
  decision: DecisionNodeComponent,
};

export function DecisionTreeView({ data }: DecisionTreeViewProps) {
  const nodes: Node[] = useMemo(() => {
    // Calculate positions for nodes
    const nodeGroups: Record<number, typeof data.nodes> = {};
    data.nodes.forEach((node, index) => {
      const groupIndex = Math.floor(index / 3);
      if (!nodeGroups[groupIndex]) nodeGroups[groupIndex] = [];
      nodeGroups[groupIndex].push(node);
    });

    return data.nodes.map((node, index) => {
      const groupIndex = Math.floor(index / 3);
      const positionInGroup = index % 3;

      return {
        id: node.id,
        type: 'decision',
        position: {
          x: 100 + positionInGroup * 220,
          y: 50 + groupIndex * 120,
        },
        data: {
          label: node.label,
          nodeType: node.node_type,
          outcome: node.outcome,
          details: node.details,
        },
      };
    });
  }, [data.nodes]);

  const edges: Edge[] = useMemo(
    () =>
      data.edges.map((edge, index) => ({
        id: `edge-${index}`,
        source: edge.from,
        target: edge.to,
        label: edge.label || undefined,
        animated: true,
        style: {
          stroke: edge.label === 'valid' || edge.label === 'allowed' ? '#22c55e' : '#ef4444',
        },
        labelStyle: {
          fill: '#94a3b8',
          fontSize: 12,
        },
        labelBgStyle: {
          fill: '#1e293b',
        },
      })),
    [data.edges]
  );

  if (data.nodes.length === 0) {
    return (
      <div className="text-center text-slate-400 py-8">
        No decision tree data available
      </div>
    );
  }

  return (
    <div className="h-[500px] w-full bg-slate-900 rounded-lg border border-slate-700">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        fitView
        attributionPosition="bottom-right"
      >
        <Background color="#334155" gap={16} />
        <Controls className="!bg-slate-800 !border-slate-600" />
        <MiniMap
          nodeColor={node => (node.data?.outcome ? '#22c55e' : '#ef4444')}
          maskColor="rgba(15, 23, 42, 0.8)"
          className="!bg-slate-800 !border-slate-600"
        />
      </ReactFlow>
    </div>
  );
}
