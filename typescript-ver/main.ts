const input: string = '1 2 3 4 5 2';
const array: Array<number> = input.split(' ').map((i) => Number(i));

let slow: number = 0;
let fast: number = 0;
let cycle = false;
while (fast < array.length) {
  slow = array[slow];
  fast = array[array[fast]];
  if (slow === fast) {
    cycle = true;
    break;
  }
}
console.log(cycle ? 'Has cycle' : 'No cycle');
