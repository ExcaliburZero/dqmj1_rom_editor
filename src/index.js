const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

const tempDirectory = "tmp";

let selectedModLi = null;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function unpackRom() {
  console.log("Prompting user to choose rom file");
  const romFilepath = await open({ multiple: false, directory: false, filters: [{ name: "Nintendo DS ROM", extensions: ["nds"] }] });

  const options = { romFilepath: romFilepath, tempDirectory: tempDirectory };
  console.log(`Unpacking rom: ${JSON.stringify(options)}`);
  await invoke("unpack_rom", options);
  console.log("Finished unpacking rom");
}

function selectMod(modName, tag) {
  tag.classList = "selected";

  if (selectedModLi !== null) {
    selectedModLi.classList = ""
  }

  selectedModLi = tag;
}

async function updateModList() {
  const modsUl = document.getElementById("mod-list");

  modsUl.innerHTML = "";

  const mods = await invoke("get_mods", {});

  let firstModLi = null;
  for (const mod of mods) {
    const modLi = document.createElement("li");
    modLi.innerText = mod;

    modsUl.appendChild(modLi);

    modLi.addEventListener("click", () => {
      selectMod(modLi.innerHTML, modLi)
    });

    if (firstModLi === null) {
      firstModLi = modLi;
    }
  }

  selectMod(firstModLi.innerHTML, firstModLi);

  console.log(mods)
}

async function loadMod(modName) {
  const options = { modName: modName };
  console.log(`Loading mod: ${JSON.stringify(options)}`);
  await invoke("load_mod", options);
  console.log("Finished loading mod");
}

async function openEditor() {
  const modName = selectedModLi.innerHTML;

  await unpackRom();
  await loadMod(modName);

  window.location.href = `editor.html?modName=${encodeURIComponent(modName)}`;
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#rom-select-button").addEventListener("click", (e) => {
    e.preventDefault();
    openEditor();
  });

  updateModList();
});
