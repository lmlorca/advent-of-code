const input = await Deno.readTextFile("input");
const chars = input.split("");
const signals = chars
  .map((_, idx) => {
    const set = [
      // Part 1
      //   ...new Set([chars[idx], chars[idx + 1], chars[idx + 2], chars[idx + 3]]),
      // Part 2
      ...new Set([
        chars[idx],
        chars[idx + 1],
        chars[idx + 2],
        chars[idx + 3],
        chars[idx + 4],
        chars[idx + 5],
        chars[idx + 6],
        chars[idx + 7],
        chars[idx + 8],
        chars[idx + 9],
        chars[idx + 10],
        chars[idx + 11],
        chars[idx + 12],
        chars[idx + 13],
      ]),
    ];

    if (set.length === 14) {
      return idx + 14;
    }
  })
  .filter((s) => s);

console.log(`Found ${signals.length} signals: ${JSON.stringify(signals)}`);
console.log(`First signal: ${signals[0]}`);
