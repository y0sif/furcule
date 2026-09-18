import { ReactFlow, Background, Controls, type Node, type Edge } from "@xyflow/react";
import "@xyflow/react/dist/style.css";

// Placeholder board. Real nodes come from the schema generated out of
// `furcule schema` and are laid out with dagre in a later phase.
const nodes: Node[] = [
  { id: "f1", position: { x: 0, y: 0 }, data: { label: "Fact: the recordings are gone" } },
  {
    id: "a1",
    position: { x: 320, y: 0 },
    data: { label: "Assumption: the only way in is to open the safe" },
  },
  { id: "i1", position: { x: 160, y: 140 }, data: { label: "Inference: someone opened the safe" } },
];
const edges: Edge[] = [
  { id: "e1", source: "f1", target: "i1" },
  { id: "e2", source: "a1", target: "i1" },
];

export function App() {
  return (
    <div style={{ width: "100%", height: "100%" }}>
      <ReactFlow nodes={nodes} edges={edges} fitView>
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}
