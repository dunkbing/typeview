import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

interface UpdateSettingsEvent extends Event {
  detail: {
    fontSize: number;
    padding: number;
  };
}

window.addEventListener(
  "tauri://update-settings",
  (event: UpdateSettingsEvent) => {
    const { fontSize, padding } = event.detail;
    document.getElementById("text").style.fontSize = `${fontSize}px`;
    document.getElementById("text").style.padding = `${padding}px`;
  },
);

document.addEventListener("mousedown", (e) => {
  if (e.button === 0) {
    appWindow.startDragging();
  }
});
