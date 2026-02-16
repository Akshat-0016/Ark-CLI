import { invoke } from "@tauri-apps/api/core";

console.log("MAIN.JS LOADED");  // DEBUG

const promptInput = document.getElementById("prompt");
const sendBtn = document.getElementById("send");
const responseBox = document.getElementById("response");

async function sendPrompt() {
  const prompt = promptInput.value.trim();
  console.log("SEND TRIGGERED, prompt =", prompt);  // DEBUG
  if (!prompt) return;

  try {
    const reply = await invoke("ask", { prompt });
    console.log("BACKEND REPLY:", reply);  // DEBUG
    responseBox.textContent = reply;
  } catch (err) {
    console.error("INVOKE ERROR:", err);
    responseBox.textContent = "Backend Error: " + err;
  }
}

sendBtn.addEventListener("click", sendPrompt);

promptInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") sendPrompt();
});
