const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

const tempDirectory = "tmp";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function unpack_rom() {
  console.log("Prompting user to choose rom file");
  const romFilepath = await open({ multiple: false, directory: false, filters: [{ name: "Nintendo DS ROM", extensions: ["nds"] }] });

  const options = { romFilepath: romFilepath, tempDirectory: tempDirectory };
  console.log(`Unpacking rom: ${JSON.stringify(options)}`);
  await invoke("unpack_rom", options);
  console.log("Finished unpacking rom");
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#rom-select-button").addEventListener("click", (e) => {
    e.preventDefault();
    unpack_rom();
  });
});
