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

        i++;
    }
}

async function showEncounter(encounterId) {
    console.log(`Showing encounter: ${encounterId}`);

    const encounter = encounters.entries[encounterId];
    console.log(encounter);

    const encounterIdTd = document.getElementById("encounters-encounter-id");
    encounterIdTd.innerHTML = padToDigits(encounterId, 3);

    const speciesTd = document.getElementById("encounters-species");
    speciesTd.innerHTML = stringTables.species_names[encounter.species_id];
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