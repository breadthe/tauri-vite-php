<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  interface PhpResponse {
    error: string;
    message: string;
  }

  let textInput: HTMLInputElement;
  let rustResponse: PhpResponse;

  function sayHiToRust() {
    const message = textInput.value.trim();
    if (message) {
      invoke("say_hi", { message }).then(
        (response) => (rustResponse = JSON.parse(response))
      );
    }
  }
</script>

<main>
  <input type="text" bind:this={textInput} />
  <button on:click={sayHiToRust}>Say hi to Rust</button>

  {#if rustResponse}
    <div>Rust response:</div>
    <div>{JSON.stringify(rustResponse)}</div>
        <div><strong>Status:</strong> {rustResponse.error ? "error" : "OK"}</div>
        <div><strong>Message:</strong> {rustResponse.message}</div>
  {/if}
</main>

<style>
</style>
