const { invoke } = window.__TAURI__.core;
const { save } = window.__TAURI__.dialog;

const tempDirectory = "tmp";

const url = new URL(window.location.toLocaleString());
const modName = url.searchParams.get("modName");

let encounters = null;
let skillSets = null;
let stringTables = null;

let currentEncounterId = null;
let currentSkillSetId = null;

async function getEncounters() {
    if (encounters === null) {
        const options = {};
        console.log(`Getting encounters: ${JSON.stringify(options)}`);
        encounters = await invoke("get_btl_enmy_prm", options);
    }
}

async function showEncounters() {
    console.log("Showing encounters");

    document.getElementById("encounters-page").style["display"] = "block";

    await getEncounters();
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
    setupSkillForEncounter(1);
    setupSkillForEncounter(2);
    setupSkillForEncounter(3);
    setupSkillForEncounter(4);
    setupSkillForEncounter(5);
    setupSkillForEncounter(6);
    setupSkillSet(1);
    setupSkillSet(2);
    setupSkillSet(3);

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

function setupSkillForEncounter(i) {
    const skill = document.getElementById("encounters-skill-" + i);

    let numSkills = stringTables.skill_names.length;
    for (let i = 0; i < numSkills; i++) {
        const option = document.createElement("option");
        option.value = i;
        option.innerHTML = `${stringTables.skill_names[i]} (${i})`;

        skill.appendChild(option);
    }

    skill.addEventListener("change", () => {
        encounters.entries[currentEncounterId].skills[i - 1].skill_id = parseInt(skill.value);
    });
}

function setupSkillSet(i) {
    const skillSet = document.getElementById("encounters-skill-set-" + i);

    let numSkills = stringTables.skill_set_names.length;
    for (let i = 0; i < numSkills; i++) {
        const option = document.createElement("option");
        option.value = i;
        option.innerHTML = `${stringTables.skill_set_names[i]} (${i})`;

        skillSet.appendChild(option);
    }

    skillSet.addEventListener("change", () => {
        encounters.entries[currentEncounterId].skill_set_ids[i - 1] = parseInt(skillSet.value);
    });
}

function populateItemDrop(encounter, i) {
    const itemDropItem = document.getElementById("encounters-item-drop-" + i + "-item");

    const itemDrop = encounter.item_drops[i - 1];

    itemDropItem.value = itemDrop.item_id;

    setupInput("encounters-item-drop-" + i + "-chance", itemDrop.chance_denominator_2_power, (tag) => { itemDrop.chance_denominator_2_power = parseInt(tag.value) });
}

function populateSkill(encounter, i) {
    const skillInput = document.getElementById("encounters-skill-" + i);

    const skill = encounter.skills[i - 1];

    skillInput.value = skill.skill_id;
}

function populateSkillSet(encounter, i) {
    const skillSetTd = document.getElementById("encounters-skill-set-" + i);

    const skillSetId = encounter.skill_set_ids[i - 1];

    skillSetTd.value = skillSetId;
}

async function showSkillSets() {
    console.log("Showing skill sets");

    document.getElementById("skill-sets-page").style["display"] = "block";

    await getSkillSets();
    await getStringTables();

    const select = document.getElementById("skill-sets-select");
    select.innerHTML = "";

    let i = 0;
    for (const _skillSet of skillSets.entries) {
        const option = document.createElement("option");
        select.appendChild(option);

        option.text = `${padToDigits(i, 3)} ${stringTables.skill_set_names[i]}`
        option.value = i;

        i++;
    }

    setupSpecies(1);
    setupSpecies(2);
    setupSpecies(3);
    setupSpecies(4);
    setupSpecies(5);
    setupSpecies(6);

    for (let i = 1; i <= 10; i++) {
        for (let j = 1; j <= 4; j++) {
            setupSkillForSkillSet(i, j);
            setupTraitForSkillSet(i, j);
        }
    }

    console.log(skillSets);
    showSkillSet(0);
}

function showSkillSet(skillSetId) {
    const skillSet = skillSets.entries[skillSetId];
    console.log(skillSet);

    currentSkillSetId = skillSetId;

    document.getElementById("skill-sets-skill-set-id").innerHTML = skillSetId;

    setupInput("skill-sets-can-upgrade", skillSet.can_upgrade, (tag) => { skillSets.entries[currentSkillSetId].can_upgrade = parseInt(tag.value) });
    setupInput("skill-sets-category", skillSet.category, (tag) => { skillSets.entries[currentSkillSetId].category = parseInt(tag.value) });
    setupInput("skill-sets-max-skill-points", skillSet.max_skill_points, (tag) => { skillSets.entries[currentSkillSetId].max_skill_points = parseInt(tag.value) });

    populateSpecies(skillSet, 1);
    populateSpecies(skillSet, 2);
    populateSpecies(skillSet, 3);
    populateSpecies(skillSet, 4);
    populateSpecies(skillSet, 5);
    populateSpecies(skillSet, 6);

    for (let i = 1; i <= 10; i++) {
        for (let j = 1; j <= 4; j++) {
            populateSkillForSkillSet(i, j);
            populateTraitForSkillSet(i, j);
        }
    }
}

function populateSpecies(skillSet, i) {
    const speciesSelect = document.getElementById("skill-sets-species-" + i);

    const skillSetId = skillSet.species_ids[i - 1];

    speciesSelect.value = skillSetId;
}

function setupSpecies(i) {
    const species = document.getElementById("skill-sets-species-" + i);

    let numSpecies = stringTables.species_names.length;
    for (let i = 0; i < numSpecies; i++) {
        const option = document.createElement("option");
        option.value = i;
        option.innerHTML = `${stringTables.species_names[i]} (${i})`;

        species.appendChild(option);
    }

    species.addEventListener("change", () => {
        skillSets.entries[currentSkillSetId].species_ids[i - 1] = parseInt(species.value);
    });
}

function setupSkillForSkillSet(i, j) {
    const skill = document.getElementById("skill-sets-skill-" + i + "-" + j);

    let numSkills = stringTables.skill_names.length;
    for (let i = 0; i < numSkills; i++) {
        const option = document.createElement("option");
        option.value = i;

        let label = `${stringTables.skill_names[i]} (${i})`;
        if (i === 0) {
            label = "";
        }

        option.innerHTML = label;

        skill.appendChild(option);
    }

    skill.addEventListener("change", () => {
        skillSets.entries[currentSkillSetId].skills[i - 1].skill_ids[j - 1] = parseInt(skill.value);
    });
}

function populateSkillForSkillSet(i, j) {
    const skill = document.getElementById("skill-sets-skill-" + i + "-" + j);

    skill.value = skillSets.entries[currentSkillSetId].skills[i - 1].skill_ids[j - 1];
}

function setupTraitForSkillSet(i, j) {
    const trait = document.getElementById("skill-sets-trait-" + i + "-" + j);

    let numTraits = stringTables.trait_names.length;
    for (let i = 0; i < numTraits; i++) {
        const option = document.createElement("option");
        option.value = i;

        let label = `${stringTables.trait_names[i]} (${i})`;
        if (i === 0) {
            label = "";
        }

        option.innerHTML = label;

        trait.appendChild(option);
    }

    trait.addEventListener("change", () => {
        skillSets.entries[currentSkillSetId].traits[i - 1].trait_ids[j - 1] = parseInt(trait.value);
    });
}

function populateTraitForSkillSet(i, j) {
    const trait = document.getElementById("skill-sets-trait-" + i + "-" + j);

    trait.value = skillSets.entries[currentSkillSetId].traits[i - 1].trait_ids[j - 1];
}

async function getSkillSets() {
    if (skillSets === null) {
        const options = {};
        console.log(`Getting skill sets: ${JSON.stringify(options)}`);
        skillSets = await invoke("get_skill_tbl", options);
        console.log(skillSets);
    }
}

async function getStringTables() {
    if (stringTables !== null) {
        return;
    }

    const options = {};
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

async function syncFiles() {
    await invoke("set_btl_enmy_prm", { btlEnmyPrm: encounters });
}

async function savePatchedRom() {
    console.log(encounters);

    // TODO: could do concurrently with user using the save dialog
    await syncFiles();

    console.log("Prompting user to choose patched rom file save location");
    const romFilepath = await save({ multiple: false, directory: false, filters: [{ name: "Nintendo DS ROM", extensions: ["nds"] }] });

    const options = { romFilepath: romFilepath };
    console.log(`Packing rom: ${JSON.stringify(options)}`);
    await invoke("pack_rom", options);
    console.log("Finished packing rom");
}

async function saveMod() {
    await syncFiles();

    const options = { modName: modName };
    console.log(`Saving mod: ${JSON.stringify(options)}`);
    await invoke("save_mod", options);
    console.log("Finished saving mod");
}


window.addEventListener("DOMContentLoaded", () => {
    //showEncounters();
    showSkillSets();

    document.querySelector("#encounters-select").addEventListener("change", (e) => {
        e.preventDefault();

        const select = document.getElementById("encounters-select");
        const value = select.value;

        console.log(value)

        const encounterId = parseInt(value.substring(0, 3));
        showEncounter(encounterId);
    });

    document.querySelector("#skill-sets-select").addEventListener("change", (e) => {
        e.preventDefault();

        const select = document.getElementById("skill-sets-select");
        const value = select.value;

        console.log(value)

        const id = parseInt(value.substring(0, 3));
        showSkillSet(id);
    });

    document.querySelector("#save-mod").addEventListener("click", (e) => {
        e.preventDefault();

        saveMod();
    });

    document.querySelector("#save-patched-rom").addEventListener("click", (e) => {
        e.preventDefault();

        savePatchedRom();
    });

    document.addEventListener("keydown", async (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === "s") {
            // Save mod - Ctrl+s (or Cmd+s on Mac)
            e.preventDefault();
            saveMod();
        } else if ((e.ctrlKey || e.metaKey) && e.key === "e") {
            // Export patched ROM - Ctrl+e (or Cmd+e on Mac)
            e.preventDefault();
            savePatchedRom();
        }
    });
});
