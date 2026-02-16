import { invoke } from "@tauri-apps/api/core";

console.log("Bubble loaded.");

document.getElementById("bubble").addEventListener("click", async () => {
  console.log("Bubble clicked → opening main widget…");

  await invoke("expand_from_bubble");
  window.close(); // hide bubble
});
