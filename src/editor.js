const { invoke } = window.__TAURI__.core;

const tempDirectory = "tmp";

let encounters = null;
let stringTables = null;

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

    const defaultEncounterId = 80; // early Slime encounter
    select.value = defaultEncounterId;
    showEncounter(defaultEncounterId);
}

async function showEncounter(encounterId) {
    console.log(`Showing encounter: ${encounterId}`);

    const encounter = encounters.entries[encounterId];
    console.log(encounter);

    document.getElementById("encounters-encounter-id").innerHTML = padToDigits(encounterId, 3);

    document.getElementById("encounters-species").innerHTML = stringTables.species_names[encounter.species_id];
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