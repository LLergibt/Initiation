export interface FileItem {
  id: string;
  kind: string;
  tags: Array<String>;
  created_at: Date;
  path: string;
  title: string;
  [key: string]: unknown;
}

interface TreeNode {
  name: string;
  isDir: boolean;
  children: Map<string, TreeNode>;
  file?: FileItem;
}

export interface RenderItem {
  id: string;
  name: string;
  depth: number;
  isDir: boolean;
  isOpen: boolean;
  file?: FileItem;
}

function buildTree(files: FileItem[]): TreeNode {
  const root: TreeNode = { name: "", isDir: true, children: new Map() };

  for (const file of files) {
    const parts = file.path.split("/");
    let node = root;

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const isLast = i === parts.length - 1;

      if (!node.children.has(part)) {
        node.children.set(part, {
          name: part,
          isDir: !isLast,
          children: new Map(),
          file: isLast ? file : undefined,
        });
      }
      node = node.children.get(part)!;
    }
  }

  return root;
}

//render list
function flattenTree(
  node: TreeNode,
  openDirs: Set<string>,
  depth = -1,
  parentPath = "",
): RenderItem[] {
  const result: RenderItem[] = [];

  for (const [, child] of node.children) {
    const id = parentPath ? `${parentPath}/${child.name}` : child.name;
    const isOpen = openDirs.has(id);

    if (depth >= 0) {
      // skip root
      result.push({
        id,
        name: child.name,
        depth,
        isDir: child.isDir,
        isOpen,
        file: child.file,
      });
    }

    // recursion for opened directories
    if (child.isDir && isOpen) {
      result.push(...flattenTree(child, openDirs, depth + 1, id));
    }
  }

  console.log(result);
  return result;
}

// rerender when updating
export function buildRenderList(
  files: FileItem[],
  openDirs: Set<string>,
): RenderItem[] {
  const tree = buildTree(files);
  return flattenTree(tree, openDirs, 0);
}
