// Configurazione globale del progetto BB84 (senza concorrenza)
// Le costanti devono essere note a scrittore, lettore e avversario.
// LUNG_MSG è uguale a 8 come da specifica.

pub const LUNG_MSG: usize = 64;         // lunghezza del messaggio (numero di fotoni)
pub const ATTIVA_AVVERSARIO: bool = true; // default: avversario disattivato; impostare a true per attivarlo

// Polarizzazioni ammesse
pub const POL_Z: char = 'Z';
pub const POL_X: char = 'X';
