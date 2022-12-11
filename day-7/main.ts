const input = await Deno.readTextFile("input").then((input) =>
  input.split("\n")
);

type Node = {
  type: "dir" | "file";
  name: string;
  size?: number;
  parent: Node | null;
  children: Node[];
};

class Context {
  tree: Node | null = null;
  nRef: Node | null = null;

  cd(ctx: string) {
    if (!this.tree) {
      const node: Node = {
        type: "dir",
        name: ctx,
        parent: null,
        children: [],
      };
      this.nRef = node;
      this.tree = node;
      return;
    }

    const node = this.nRef?.children.find((child) => child.name === ctx);

    if (node) {
      this.nRef = node;
    }
  }

  appendDir(dir: string) {
    if (this.nRef) {
      const node: Node = {
        type: "dir",
        name: dir,
        parent: this.nRef,
        children: [],
      };

      this.nRef.children.push(node);
    }
  }

  appendFile(file: string, size: number) {
    if (this.nRef) {
      const node: Node = {
        type: "file",
        name: file,
        size,
        parent: this.nRef,
        children: [],
      };

      this.nRef.children.push(node);
    }
  }

  cdUp() {
    if (this.nRef) {
      this.nRef = this.nRef.parent;
    }
  }

  log() {
    console.log(
      JSON.stringify(
        this.tree,
        (key, value) => (key === "parent" ? null : value),
        2
      )
    );
  }
}

const ctx = new Context();

input.forEach((line, idx) => {
  const prev = input[idx - 1];

  if (line.startsWith("$ ls")) {
    ctx.cd(prev.split(" ")[2]);
  }
  if (line.startsWith("dir")) {
    ctx.appendDir(line.split(" ")[1]);
  }

  if (Number(line[0])) {
    const [size, file] = line.split(" ");

    ctx.appendFile(file, Number(size));
  }

  if (line.includes("$ cd ..")) {
    ctx.cdUp();
  }
});

const nodeSizes: number[] = [];

const allSizes: number[] = [];

const sumFiles = (node: Node | null): Node | null => {
  if (!node) return node;
  if (node.type === "dir") {
    const size = node.children.reduce((acc, curr) => {
      if (curr.type === "file") {
        return acc + (curr.size || 0);
      }
      const size = sumFiles(curr)?.size;

      return acc + (size || 0);
    }, 0);

    node.size = size;
    allSizes.push(size);
    if (size <= 100000) nodeSizes.push(size);
  }

  return node;
};

const tree = sumFiles(ctx.tree);

const part1 = nodeSizes.reduce((acc, curr) => acc + curr, 0);

console.log("part1:", part1);

// Part 2
const FREE = 70000000 - (tree?.size || 0);
const UPDATE = 30000000;
const DELETE = UPDATE - FREE;

const part2 = allSizes
  .map((size) => [size, size - DELETE])
  .filter(([_, diff]) => diff >= 0)
  .sort(([_, diffA], [__, diffB]) => diffA - diffB)[0][0];

console.log("part2:", part2);
