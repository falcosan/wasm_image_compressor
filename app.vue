<template>
  <Html lang="en" />
  <Title>Wasm Image Compressor</Title>
  <Head>
    <Link rel="icon" href="/favicon.ico" />
  </Head>
  <main
    class="flex flex-col min-h-dvh items-center justify-center bg-slate-800"
  >
    <div class="w-screen max-w-2xl p-5">
      <Image
        class="w-full mb-10 rounded"
        src="https://psymedia.co.za/wp-content/uploads/ozora-from-the-hill-1200x900.jpg"
      />
      <Converter />
    </div>
  </main>
</template>
