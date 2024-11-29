const { core } = Deno;
const { ops } = core;

globalThis.custom_runtime = {
  fetch: (url) => {
    return ops.op_fetch(url);
  },
};
