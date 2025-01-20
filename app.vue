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
        src="https://static.penumbracosmica.earth/strapi-uploads/assets/blog_atance_datos_agricultura_00_c1492da2dd.png "
      />
      <Converter />
    </div>
  </main>
</template>
