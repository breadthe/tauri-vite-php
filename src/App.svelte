<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let textInput: HTMLInputElement;
  let rustResponse: string = "";

  function sayHiToRust() {
    const message = textInput.value.trim();
    if (message) {
      invoke("say_hi", { message }).then(
        (response) => (rustResponse = response)
      );
    }
  }
</script>

<main>
  <input type="text" bind:this={textInput} />
  <button on:click={sayHiToRust}>Say hi to Rust</button>

  {#if rustResponse}
    <div>Rust response:</div>
    <div>{rustResponse}</div>
  {/if}
</main>

<style>
</style>
