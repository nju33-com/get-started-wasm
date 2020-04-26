// eslint-disable-next-line @typescript-eslint/no-floating-promises
;(async () => {
  const module = await import('../pkg')
  module.hello_wasm()
})()

export {}
