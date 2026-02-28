const { locale } = window.__TAURI__.os;

async function i18n() {
    //const lang = await locale();
    const lang = "jp";

    console.log("Lang:");
    console.log(lang);

    if (lang.startsWith("en")) {
        return; // Text in the HTML is already in English by default
    }

    const translation = await (await fetch(`/locales/${lang}.json`)).json(); // Note: This failing for non-supported languages is fine, as it will default to en anyways
    console.log(translation);

    const mappings = [
        ["create-new-mod", "Create new mod (+)"],
        ["rom-select-button", "Load ROM"]
    ];

    for (const [id, key] of mappings) {
        console.log(id);
        document.getElementById(id).textContent = translation.index[key];
    }
}

window.addEventListener("DOMContentLoaded", () => {
    i18n();
});
