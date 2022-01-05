import init, { greet } from "./pkg/steering.js";
init().then(() => {
  greet("WebAssembly");
});
