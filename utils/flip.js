// Flips the keycode combinations from "X => Y,"" to "Y => X,"
const args = Deno.args;
const keys = args[0];

const result = keys
  .split('\n')
  .map((line) => {
    const [key, value] = line.trim().split('=>');

    return `${value.replaceAll(',', '')}=>${key},`;
  })
  .join('\n');

console.log(result);
