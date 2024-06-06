import { invoke } from "@tauri-apps/api/tauri";

document.getElementById("applyButton")!.addEventListener("click", () => {
  const fontSize = document.getElementById("fontSize")!.value;
  const padding = document.getElementById("padding")!.value;
  invoke("update_settings", {
    fontSize: parseInt(fontSize),
    padding: parseInt(padding),
  });
});
