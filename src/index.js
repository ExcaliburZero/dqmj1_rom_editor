const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

const tempDirectory = "tmp";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function unpackRom() {
  console.log("Prompting user to choose rom file");
  const romFilepath = await open({ multiple: false, directory: false, filters: [{ name: "Nintendo DS ROM", extensions: ["nds"] }] });

  const options = { romFilepath: romFilepath, tempDirectory: tempDirectory };
  console.log(`Unpacking rom: ${JSON.stringify(options)}`);
  await invoke("unpack_rom", options);
  console.log("Finished unpacking rom");

  window.location.href = "editor.html";
}

async function updateModList() {
  const modsUl = document.getElementById("mod-list");

  modsUl.innerHTML = "";

  const mods = await invoke("get_mods", {});
  for (const mod of mods) {
    const modLi = document.createElement("li");
    modLi.innerText = mod;

    modsUl.appendChild(modLi);
  }

  console.log(mods)
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#rom-select-button").addEventListener("click", (e) => {
    e.preventDefault();
    unpackRom();
  });

  updateModList();
});
