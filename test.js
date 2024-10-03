const test = require('./index.js');

const items = [2,4,6];
const weights = [2,4,6];

const start = performance.now();
const result = (test.rollWalkerTable(120_000_000,new Float32Array(weights)));
const end = performance.now();
console.log(`Execution time: ${end - start}ms`);
const sum = result.reduce((acc, item) => acc + item, 0);
items.map((item, index) => {
  console.log(`Item ${item} got ${result[index].toLocaleString()} ${( result[index] / sum)*100 }%`);
});

console.log(sum);