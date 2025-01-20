export default defineEventHandler(async (event) => {
  const query = getQuery(event);
  const url = query.url as string | undefined;

  if (!url) return createError({ message: "URL not valid" });

  const blobImage = await $fetch<Blob>(url, { responseType: "blob" });

  return blobImage;
});
