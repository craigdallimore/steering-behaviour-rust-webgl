import init, { setup, tick } from "./pkg/steering.js";

init().then(() => {
  const context = setup();
  console.log({ context });

  function frame(prevtime) {
    return function (time) {
      const t = (time - prevtime) / 1000;
      tick(context, t);
      window.requestAnimationFrame(frame(time));
    };
  }

  window.requestAnimationFrame(frame(0));
});
