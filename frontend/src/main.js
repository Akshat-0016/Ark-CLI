import { invoke } from "@tauri-apps/api/core";

console.log("Ark Widget Loaded.");

const sendBtn  = document.getElementById("send");
const promptEl = document.getElementById("prompt");
const outputEl = document.getElementById("output");
const spinner  = document.getElementById("loading-spinner");

async function askArk() {
  const text = promptEl.value.trim();
  if (!text) return;

  // UI: user feedback
  outputEl.textContent = "";
  spinner.style.display = "inline-block";

  sendBtn.disabled = true;
  promptEl.disabled = true;

  try {
    // Ask backend
    const res = await invoke("ask", { prompt: text });
    outputEl.textContent = res;
  } catch (err) {
    outputEl.textContent = `Error: ${err}`;
  }

  // Reset UI
  spinner.style.display = "none";
  sendBtn.disabled = false;
  promptEl.disabled = false;
}

sendBtn.addEventListener("click", askArk);

// Press Enter to send
promptEl.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    askArk();
  }
});
1
