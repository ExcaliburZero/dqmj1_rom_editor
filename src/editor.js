const { invoke } = window.__TAURI__.core;

const tempDirectory = "tmp";

let encounters = null;

async function show_encounters() {
    console.log("Showing encounters");
    if (encounters === null) {
        const options = { tempDirectory: tempDirectory };
        console.log(`Getting encounters: ${JSON.stringify(options)}`);
        encounters = await invoke("get_btl_enmy_prm", options);
    }

    console.log(`Encounters: ${JSON.stringify(encounters.entries[0])}`);
    console.log(`Encounters: ${JSON.stringify(encounters.entries[1])}`);

    const table = document.getElementById("encounters-table");
    table.innerHTML = "";

    table.innerHTML = "<tr><th>Index</th><th>Species id</th></tr>"

    //table.append
    let i = 0;
    for (const encounter of encounters.entries) {
        const row = document.createElement("tr");
        table.appendChild(row);

        row.innerHTML = `<tr><td>${i}</td><td>${encounter.species_id}</td></tr>`;

        i++;
    }
}

show_encounters()