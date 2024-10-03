import test from 'ava';
import { rollWalkerTable } from '../index.js';

test('sum from native', (t) => {
  const items = [2,4,6];
  const result = (rollWalkerTable(3333, new Float32Array(items)));
  t.is(result.length,items.length);
  const sum = result.reduce((acc, item) => acc + item, 0);
  t.is(sum, 3333);
})

test('test 2', (t) => {
  const items = [2,4,6];
  const result = (rollWalkerTable(1_000_000, new Float32Array(items)));
  t.is(result.length,items.length);
  const sum = result.reduce((acc, item) => acc + item, 0);
  t.is(sum, 1_000_000);
})
