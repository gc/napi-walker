import test from 'ava';
import { rollWalkerTable } from '../index.js';

function run(quantity, items) {
  const obj = rollWalkerTable(quantity, new Float32Array(items));
  return JSON.parse(obj);
}

test('sum from native', (t) => {
  const items = [2,4,6];
  const result = run(3333, items);
  const sum = Object.values(result).reduce((acc, item) => acc + item, 0);
  t.is(sum, 3333);
})

test('test 2', (t) => {
  const items = [2,4,6];
  const result = run(1_000_000, (items));
  t.is(Object.keys(result).length,items.length);
  const sum = Object.values(result).reduce((acc, item) => acc + item, 0);
  t.is(sum, 1_000_000);
})

test('test 3', (t) => {
  const items = [1,1,1,1,51,5252,5,33,7,2];
  const result = run(5, (items));
  const sum = Object.values(result).reduce((acc, item) => acc + item, 0);
  t.is(sum, 5);
})

test('test 4', (t) => {
  const items = [1,1,1,1,51,5252,5,33,7,2];
  const result = run(1, (items));
  const sum = Object.values(result).reduce((acc, item) => acc + item, 0);
  t.is(sum, 1);
})
