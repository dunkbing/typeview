import { invoke } from "@tauri-apps/api/tauri";

document.getElementById("applyButton")!.addEventListener("click", () => {
  const fontSize = (document.getElementById("fontSize") as HTMLInputElement).value;
  const padding = (document.getElementById("padding") as HTMLInputElement).value;
  console.log(fontSize, padding);
  invoke("update_settings", {
    fontSize: parseInt(fontSize),
    padding: parseInt(padding),
  });
});

const soundSelect = document.getElementById("sound-select") as HTMLSelectElement;

soundSelect.addEventListener("change", () => {
  const selectedSound = soundSelect.value;
  invoke("set_selected_sound", { sound: selectedSound });
});

invoke("get_selected_sound").then((selectedSound) => {
  soundSelect.value = selectedSound as string;
});
