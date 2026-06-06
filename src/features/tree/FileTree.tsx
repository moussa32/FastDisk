import { useMemo, useState } from "react";
import { EmptyState, LoadingState, PercentageBar, SizeBadge } from "../../shared/components";
import type { FileEntryDto } from "../../shared/api/types";
import { formatDate, formatPercentage } from "../../shared/utils/format";
import { loadChildren } from "../../shared/api/tree";
import { setSelectedFolder } from "./treeState";

type TreeNode = FileEntryDto & {
  children?: FileEntryDto[];
};

type FileTreeProps = {
  scanSessionId: number | null;
  root?: FileEntryDto;
  loadChildrenForNode?: (parentId: number | null) => Promise<FileEntryDto[]>;
};

export function FileTree({ scanSessionId, root, loadChildrenForNode }: FileTreeProps) {
  const [expandedIds, setExpandedIds] = useState<Set<number>>(new Set());
  const [childrenByParent, setChildrenByParent] = useState<Record<number, FileEntryDto[]>>({});
  const [loadingId, setLoadingId] = useState<number | null>(null);
  const [selectedId, setSelectedId] = useState<number | null>(root?.id ?? null);

  const nodes = useMemo(() => (root ? [root] : []), [root]);

  async function toggleNode(node: FileEntryDto) {
    if (!node.isDirectory) {
      return;
    }

    const nextExpanded = new Set(expandedIds);
    if (nextExpanded.has(node.id)) {
      nextExpanded.delete(node.id);
      setExpandedIds(nextExpanded);
      return;
    }

    nextExpanded.add(node.id);
    setExpandedIds(nextExpanded);
    if (!childrenByParent[node.id]) {
      setLoadingId(node.id);
      const loader = loadChildrenForNode ?? defaultLoadChildren(scanSessionId);
      const children = await loader(getLoadParentId(node));
      setChildrenByParent((current) => ({ ...current, [node.id]: children }));
      setLoadingId(null);
    }
  }

  function selectNode(node: FileEntryDto) {
    setSelectedId(node.id);
    if (node.isDirectory) {
      setSelectedFolder(node.id, node.path);
    }
  }

  if (!root) {
    return (
      <EmptyState
        title="No tree data yet"
        description="Complete a scan before browsing folder children."
      />
    );
  }

  return (
    <div className="file-tree" role="tree" aria-label="File tree">
      {nodes.map((node) => (
        <TreeRow
          childrenByParent={childrenByParent}
          expandedIds={expandedIds}
          key={node.id}
          loadingId={loadingId}
          node={node}
          onSelect={selectNode}
          onToggle={toggleNode}
          selectedId={selectedId}
        />
      ))}
    </div>
  );
}

function defaultLoadChildren(scanSessionId: number | null) {
  return (parentId: number | null) => {
    if (scanSessionId === null) {
      return Promise.resolve([]);
    }
    return loadChildren({
      scanSessionId,
      parentId,
      sortBy: "size",
      sortDirection: "desc",
      limit: 500,
      offset: 0,
    });
  };
}

function getLoadParentId(node: FileEntryDto) {
  return node.id === 0 && node.parentId === null ? null : node.id;
}

type TreeRowProps = {
  node: TreeNode;
  expandedIds: Set<number>;
  childrenByParent: Record<number, FileEntryDto[]>;
  loadingId: number | null;
  selectedId: number | null;
  onSelect: (node: FileEntryDto) => void;
  onToggle: (node: FileEntryDto) => void;
};

function TreeRow({
  node,
  expandedIds,
  childrenByParent,
  loadingId,
  selectedId,
  onSelect,
  onToggle,
}: TreeRowProps) {
  const isExpanded = expandedIds.has(node.id);
  const children = childrenByParent[node.id] ?? [];
  const percentage = node.parentId === null ? 100 : 0;

  return (
    <div>
      <div
        aria-selected={selectedId === node.id}
        className="tree-row"
        role="treeitem"
      >
        <button
          aria-label={isExpanded ? `Collapse ${node.name}` : `Expand ${node.name}`}
          disabled={!node.isDirectory}
          onClick={() => onToggle(node)}
          type="button"
        >
          {node.isDirectory ? (isExpanded ? "-" : "+") : ""}
        </button>
        <button className="tree-name" onClick={() => onSelect(node)} type="button">
          {node.name}
        </button>
        <span>{node.isDirectory ? "Folder" : "File"}</span>
        <SizeBadge bytes={node.size} />
        <span>{formatPercentage(percentage)}</span>
        <PercentageBar value={percentage} />
        <span>{node.childCount} children</span>
        <span>{formatDate(node.modifiedAt)}</span>
      </div>
      {loadingId === node.id ? <LoadingState label="Loading children" /> : null}
      {isExpanded && children.length === 0 && loadingId !== node.id ? (
        <EmptyState title="Empty folder" description="No direct children were returned." />
      ) : null}
      {isExpanded && children.length > 0 ? (
        <div className="tree-children" role="group">
          {children.map((child) => (
            <TreeRow
              childrenByParent={childrenByParent}
              expandedIds={expandedIds}
              key={child.id}
              loadingId={loadingId}
              node={child}
              onSelect={onSelect}
              onToggle={onToggle}
              selectedId={selectedId}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
}
