<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let rustResponse: string = "";

  function sayHiToRust() {
    invoke("say_hi", { name: "Rust" }).then(
      (response) => (rustResponse = response)
    );
  }
</script>

<main>
  <button on:click={sayHiToRust}>Say hi to Rust</button>

  {#if rustResponse}
    <div>Rust response:</div>
    <div>{rustResponse}</div>
  {/if}
</main>

<style>
</style>
