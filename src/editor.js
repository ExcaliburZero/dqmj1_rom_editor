const { invoke } = window.__TAURI__.core;
const { save } = window.__TAURI__.dialog;

const tempDirectory = "tmp";

let encounters = null;
let stringTables = null;

let currentEncounterId = null;

async function showEncounters() {
    console.log("Showing encounters");
    if (encounters === null) {
        const options = { tempDirectory: tempDirectory };
        console.log(`Getting encounters: ${JSON.stringify(options)}`);
        encounters = await invoke("get_btl_enmy_prm", options);
    }

    console.log(`Encounters: ${JSON.stringify(encounters.entries[0])}`);
    console.log(`Encounters: ${JSON.stringify(encounters.entries[1])}`);

    await getStringTables();

    const select = document.getElementById("encounters-select");
    select.innerHTML = "";

    let i = 0;
    for (const encounter of encounters.entries) {
        const option = document.createElement("option");
        select.appendChild(option);

        option.text = `${padToDigits(i, 3)} ${stringTables.species_names[encounter.species_id]}`
        option.value = i;

        i++;
    }

    setupEncounterSpecies();
    setupItemDrop(1);
    setupItemDrop(2);

    const defaultEncounterId = 48; // starter Dracky
    select.value = defaultEncounterId;
    showEncounter(defaultEncounterId);
}

async function showEncounter(encounterId) {
    console.log(`Showing encounter: ${encounterId}`);

    currentEncounterId = encounterId;

    const encounter = encounters.entries[encounterId];
    console.log(encounter);

    document.getElementById("encounters-encounter-id").innerHTML = padToDigits(encounterId, 3);
    document.getElementById("encounters-species").value = encounter.species_id;

    setupInput("encounters-level", encounter.level, (tag) => { encounters.entries[currentEncounterId].level = parseInt(tag.value) });
    setupInput("encounters-max-hp", encounter.max_hp, (tag) => { encounters.entries[currentEncounterId].max_hp = parseInt(tag.value) });
    setupInput("encounters-max-mp", encounter.max_mp, (tag) => { encounters.entries[currentEncounterId].max_mp = parseInt(tag.value) });
    setupInput("encounters-attack", encounter.attack, (tag) => { encounters.entries[currentEncounterId].attack = parseInt(tag.value) });
    setupInput("encounters-defense", encounter.defense, (tag) => { encounters.entries[currentEncounterId].defense = parseInt(tag.value) });
    setupInput("encounters-wisdom", encounter.wisdom, (tag) => { encounters.entries[currentEncounterId].wisdom = parseInt(tag.value) });
    setupInput("encounters-agility", encounter.agility, (tag) => { encounters.entries[currentEncounterId].agility = parseInt(tag.value) });
    setupInput("encounters-scout-chance", encounter.scout_chance, (tag) => { encounters.entries[currentEncounterId].scout_chance = parseInt(tag.value) });
    setupInput("encounters-exp", encounter.exp, (tag) => { encounters.entries[currentEncounterId].exp = parseInt(tag.value) });
    setupInput("encounters-gold", encounter.gold, (tag) => { encounters.entries[currentEncounterId].gold = parseInt(tag.value) });

    populateItemDrop(encounter, 1);
    populateItemDrop(encounter, 2);

    populateSkill(encounter, 1);
    populateSkill(encounter, 2);
    populateSkill(encounter, 3);
    populateSkill(encounter, 4);
    populateSkill(encounter, 5);
    populateSkill(encounter, 6);

    populateSkillSet(encounter, 1);
    populateSkillSet(encounter, 2);
    populateSkillSet(encounter, 3);
}

function setupInput(id, value, setter) {
    const tag = document.getElementById(id);
    tag.value = value;

    tag.addEventListener("change", () => setter(tag))
}

function setupEncounterSpecies() {
    const speciesSelect = document.getElementById("encounters-species");

    let numSpecies = stringTables.species_names.length;
    for (let i = 0; i < numSpecies; i++) {
        const option = document.createElement("option");
        option.value = i;
        option.innerHTML = `${stringTables.species_names[i]} (${i})`;

        speciesSelect.appendChild(option);
    }

    speciesSelect.addEventListener("change", () => {
        encounters.entries[currentEncounterId].species_id = parseInt(speciesSelect.value);
    });
}

function setupItemDrop(i) {
    const itemDropItem = document.getElementById("encounters-item-drop-" + i + "-item");

    let numItems = stringTables.item_names.length;
    for (let i = 0; i < numItems; i++) {
        const option = document.createElement("option");
        option.value = i;
        option.innerHTML = `${stringTables.item_names[i]} (${i})`;

        itemDropItem.appendChild(option);
    }

    itemDropItem.addEventListener("change", () => {
        encounters.entries[currentEncounterId].item_drops[i - 1].item_id = parseInt(itemDropItem.value);
    });
}

function populateItemDrop(encounter, i) {
    const itemDropItem = document.getElementById("encounters-item-drop-" + i + "-item");

    const itemDrop = encounter.item_drops[i - 1];

    itemDropItem.value = itemDrop.item_id;

    setupInput("encounters-item-drop-" + i + "-chance", itemDrop.chance_denominator_2_power, (tag) => { itemDrop.chance_denominator_2_power = parseInt(tag.value) });
}

function populateSkill(encounter, i) {
    const skillTd = document.getElementById("encounters-skill-" + i);

    const skill = encounter.skills[i - 1];

    skillTd.innerHTML = `${stringTables.skill_names[skill.skill_id]} (${skill.skill_id})`;
}

function populateSkillSet(encounter, i) {
    const skillSetTd = document.getElementById("encounters-skill-set-" + i);

    console.log(encounter);

    const skillSetId = encounter.skill_set_ids[i - 1];

    skillSetTd.innerHTML = `${stringTables.skill_set_names[skillSetId]} (${skillSetId})`;
}

async function getStringTables() {
    if (stringTables !== null) {
        return;
    }

    const options = { tempDirectory: tempDirectory };
    console.log(`Getting string tables: ${JSON.stringify(options)}`);
    stringTables = await invoke("get_string_tables", options);
    console.log(stringTables);
}

function padToDigits(number, numDigits) {
    let string = number.toString();
    while (string.length < numDigits) {
        string = "0" + string;
    }

    return string;
}

async function savePatchedRom() {
    console.log(encounters);

    // TODO: could do concurrently with user using the save dialog
    await invoke("set_btl_enmy_prm", { tempDirectory: tempDirectory, btlEnmyPrm: encounters });

    console.log("Prompting user to choose patched rom file save location");
    const romFilepath = await save({ multiple: false, directory: false, filters: [{ name: "Nintendo DS ROM", extensions: ["nds"] }] });

    const options = { romFilepath: romFilepath, tempDirectory: tempDirectory };
    console.log(`Packing rom: ${JSON.stringify(options)}`);
    await invoke("pack_rom", options);
    console.log("Finished packing rom");
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#encounters-select").addEventListener("change", (e) => {
        e.preventDefault();

        const select = document.getElementById("encounters-select");
        const value = select.value;

        console.log(value)

        const encounterId = parseInt(value.substring(0, 3));
        showEncounter(encounterId);
    });

    document.querySelector("#save-patched-rom").addEventListener("click", (e) => {
        e.preventDefault();

        savePatchedRom();
    });
});

showEncounters()