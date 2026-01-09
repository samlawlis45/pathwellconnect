'use client';

import { useMemo } from 'react';
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
import { Icon } from '@iconify/react';
import type { DecisionTree } from '@/lib/types';

interface DecisionTreeViewProps {
  data: DecisionTree;
}

function DecisionNodeComponent({ data }: NodeProps) {
  const bgColor = data.outcome
    ? 'bg-emerald-50 border-emerald-400'
    : 'bg-red-50 border-red-400';

  const iconName = {
    identity: 'solar:user-circle-outline',
    policy: 'solar:shield-check-outline',
    action: 'solar:bolt-outline',
  }[data.nodeType as string] || 'solar:widget-outline';

  const iconColor = data.outcome ? 'text-emerald-500' : 'text-red-500';

  return (
    <div className={`px-4 py-3 rounded-lg border-2 ${bgColor} min-w-[180px] bg-white`}>
      <Handle type="target" position={Position.Top} className="!bg-slate-400" />
      <div className="flex items-center gap-2">
        <Icon icon={iconName} className={`w-5 h-5 ${iconColor}`} />
        <div>
          <div className="text-xs text-slate-500 uppercase tracking-wide">{data.nodeType}</div>
          <div className="text-sm text-slate-900 font-medium">{data.label}</div>
        </div>
      </div>
      <Handle type="source" position={Position.Bottom} className="!bg-slate-400" />
    </div>
  );
}

const nodeTypes = {
  decision: DecisionNodeComponent,
};

export function DecisionTreeView({ data }: DecisionTreeViewProps) {
  const nodes: Node[] = useMemo(() => {
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
          stroke: edge.label === 'valid' || edge.label === 'allowed' ? '#10b981' : '#ef4444',
          strokeWidth: 2,
        },
        labelStyle: {
          fill: '#64748b',
          fontSize: 11,
          fontWeight: 500,
        },
        labelBgStyle: {
          fill: '#ffffff',
          fillOpacity: 0.9,
        },
      })),
    [data.edges]
  );

  if (data.nodes.length === 0) {
    return (
      <div className="text-center text-slate-400 py-8 flex flex-col items-center">
        <Icon icon="solar:share-outline" className="w-8 h-8 mb-2" />
        No decision tree data available
      </div>
    );
  }

  return (
    <div className="h-[500px] w-full bg-slate-50 rounded-lg border border-slate-200">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        fitView
        attributionPosition="bottom-right"
      >
        <Background color="#cbd5e1" gap={16} />
        <Controls className="!bg-white !border-slate-200 !shadow-sm [&>button]:!bg-white [&>button]:!border-slate-200 [&>button]:!text-slate-600 [&>button:hover]:!bg-slate-50" />
        <MiniMap
          nodeColor={node => (node.data?.outcome ? '#10b981' : '#ef4444')}
          maskColor="rgba(248, 250, 252, 0.8)"
          className="!bg-white !border-slate-200"
        />
      </ReactFlow>
    </div>
  );
}
