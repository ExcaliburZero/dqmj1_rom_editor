const { invoke } = window.__TAURI__.core;

const tempDirectory = "tmp";

let encounters = null;
let stringTables = null;

async function show_encounters() {
    console.log("Showing encounters");
    if (encounters === null) {
        const options = { tempDirectory: tempDirectory };
        console.log(`Getting encounters: ${JSON.stringify(options)}`);
        encounters = await invoke("get_btl_enmy_prm", options);
    }

    console.log(`Encounters: ${JSON.stringify(encounters.entries[0])}`);
    console.log(`Encounters: ${JSON.stringify(encounters.entries[1])}`);

    await get_string_tables();

    const table = document.getElementById("encounters-table");
    table.innerHTML = "";

    table.innerHTML = "<tr><th>Index</th><th>Species id</th></tr>"

    //table.append
    let i = 0;
    for (const encounter of encounters.entries) {
        const row = document.createElement("tr");
        table.appendChild(row);

        row.innerHTML = `<tr><td>${i}</td><td>${stringTables.species_names[encounter.species_id]} (${encounter.species_id})</td></tr>`;

        i++;
    }
}

async function get_string_tables() {
    if (stringTables !== null) {
        return;
    }

    const options = { tempDirectory: tempDirectory };
    console.log(`Getting string tables: ${JSON.stringify(options)}`);
    stringTables = await invoke("get_string_tables", options);
    console.log(stringTables);
}

show_encounters()