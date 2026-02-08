const { invoke } = window.__TAURI__.core;

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

    const defaultEncounterId = 80; // early Slime encounter
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
    document.getElementById("encounters-level").innerHTML = encounter.level;
    document.getElementById("encounters-max-hp").innerHTML = encounter.max_hp;
    document.getElementById("encounters-max-mp").innerHTML = encounter.max_mp;
    document.getElementById("encounters-attack").innerHTML = encounter.attack;
    document.getElementById("encounters-defense").innerHTML = encounter.defense;
    document.getElementById("encounters-wisdom").innerHTML = encounter.wisdom;
    document.getElementById("encounters-agility").innerHTML = encounter.agility;
    document.getElementById("encounters-scout-chance").innerHTML = encounter.scout_chance;
    document.getElementById("encounters-exp").innerHTML = encounter.exp;
    document.getElementById("encounters-gold").innerHTML = encounter.gold;

    populateItemDrop(encounter, 1);
    populateItemDrop(encounter, 2);

    populateSkill(encounter, 1);
    populateSkill(encounter, 2);
    populateSkill(encounter, 3);
    populateSkill(encounter, 4);
    populateSkill(encounter, 5);
    populateSkill(encounter, 6);
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
        encounters.entries[currentEncounterId].species_id = speciesSelect.value;
    });
}

function populateItemDrop(encounter, i) {
    const itemDropItem = document.getElementById("encounters-item-drop-" + i + "-item");
    const itemDropChance = document.getElementById("encounters-item-drop-" + i + "-chance");

    const itemDrop = encounter.item_drops[i - 1];

    itemDropItem.innerHTML = `${stringTables.item_names[itemDrop.item_id]} (${itemDrop.item_id})`;
    itemDropChance.innerHTML = itemDrop.chance_denominator_2_power;
}

function populateSkill(encounter, i) {
    const skillTd = document.getElementById("encounters-skill-" + i);

    const skill = encounter.skills[i - 1];

    skillTd.innerHTML = `${stringTables.skill_names[skill.skill_id]} (${skill.skill_id})`;
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

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#encounters-select").addEventListener("click", (e) => {
        e.preventDefault();

        const select = document.getElementById("encounters-select");
        const value = select.value;

        console.log(value)

        const encounterId = parseInt(value.substring(0, 3));
        showEncounter(encounterId);
    });
});

showEncounters()