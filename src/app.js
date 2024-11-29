try {
  const user = await custom_runtime.fetch(
    "https://jsonplaceholder.typicode.com/users/1"
  );
  Deno.core.print(user);
} catch (error) {
  Deno.core.print(error);
}
