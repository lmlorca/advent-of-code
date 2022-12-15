const input = await Deno.readTextFile("input").then((input) =>
  input.split("\n")
);

type Tree = {
  id: string;
  seen: boolean;
  height: number;
};

const checkTrees = (matrix: Tree[][]) =>
  matrix.forEach((treeList) => {
    let bigst = treeList[0];
    treeList.forEach((tree, idx) => {
      const prev = treeList[idx - 1];
      if (!prev) {
        tree.seen = true;
      } else {
        if (tree.height > bigst.height) {
          tree.seen = true;
          bigst = tree;
        }
      }
    });
  });

const matrix: Tree[][] = input
  .map((line) => line.split(""))
  .map((n) => n.map(Number))
  .map((line, idx1) =>
    line.map((x, idx2) => ({ id: `${idx1}${idx2}`, seen: false, height: x }))
  );

const vertMatrix = matrix.map((x) => x.slice());

const horizMatrix = vertMatrix[0].map((_, i) => vertMatrix.map((x) => x[i]));

checkTrees(vertMatrix);
checkTrees(horizMatrix);
checkTrees(horizMatrix.map((x) => x.reverse()));
checkTrees(vertMatrix.map((x) => x.reverse()));

const part1 = matrix
  .map((treeList) => treeList.filter((x) => x.seen).length)
  .reduce((sum, acc) => sum + acc, 0);

console.log("part1", part1);

// Part 2
let part2 = 0;

matrix.forEach((treeList) => treeList.forEach((tree) => (tree.seen = false)));

for (let matrixIdx = 1; matrixIdx < matrix.length - 1; matrixIdx++) {
  const treeList = matrix[matrixIdx];

  for (let lineIdx = 1; lineIdx < treeList.length - 1; lineIdx++) {
    const tree = treeList[lineIdx];

    let leftCount = 0;
    let rightCount = 0;
    let upCount = 0;
    let downCount = 0;

    // check left
    for (let x = lineIdx - 1; x >= 0; x--) {
      const left = treeList[x];
      leftCount++;

      if (left.height >= tree.height) break;
    }

    // check right
    for (let x = lineIdx + 1; x < treeList.length; x++) {
      const right = treeList[x];
      rightCount++;
      if (right.height >= tree.height) break;
    }

    // check upwards
    for (let y = matrixIdx - 1; y >= 0; y--) {
      const up = matrix[y][lineIdx];
      upCount++;
      if (up.height >= tree.height) break;
    }

    // check downwards
    for (let y = matrixIdx + 1; y < matrix.length; y++) {
      const down = matrix[y][lineIdx];
      downCount++;
      if (down.height >= tree.height) break;
    }

    const scenicScore = upCount * leftCount * downCount * rightCount;

    if (scenicScore > part2) {
      part2 = scenicScore;
    }
  }
}

console.log("part2", part2);
