use dioxus::prelude::*;

/// Ritorna un signal reattivo per il tema scuro, sincronizzato con localStorage
/// e con la classe `dark` sul tag <html> (letta all'avvio, scritta ad ogni cambio).
pub fn use_theme() -> Signal<bool> {
    let mut dark_mode = use_signal(|| false);
    let mut loaded = use_signal(|| false); // evita di sovrascrivere localStorage prima di averlo letto

    // Al mount: leggi la preferenza salvata
    use_effect(move || {
        spawn(async move {
            if let Ok(value) = document::eval("return localStorage.getItem('theme');").await {
                if value.as_str() == Some("dark") {
                    dark_mode.set(true);
                }
            }
            loaded.set(true);
        });
    });

    // Ad ogni cambio (dopo il caricamento iniziale): applica la classe + salva
    use_effect(move || {
        let is_dark = dark_mode();
        if !loaded() {
            return; // non scrivere finché non abbiamo letto il valore salvato
        }
        spawn(async move {
            let script = if is_dark {
                "document.documentElement.classList.add('dark'); localStorage.setItem('theme', 'dark');"
            } else {
                "document.documentElement.classList.remove('dark'); localStorage.setItem('theme', 'light');"
            };
            let _ = document::eval(script).await;
        });
    });

    dark_mode
}