// Configurazione globale del progetto BB84 (senza concorrenza)
// Le costanti devono essere note a scrittore, lettore e avversario.

pub const LUNG_MSG: usize = 64;         // lunghezza del messaggio (numero di fotoni) è un parametro che può essere modificato
pub const ATTIVA_AVVERSARIO: bool = false; // default: avversario disattivato; impostare a true per attivarlo

// Polarizzazioni ammesse
pub const POL_Z: char = 'Z';
pub const POL_X: char = 'X';

/// Legge `LUNG_MSG` da environment variabile `LUNG_MSG`, altrimenti ritorna il valore di default.
pub fn lung_msg() -> usize {
	std::env::var("LUNG_MSG").ok()
		.and_then(|s| s.parse::<usize>().ok())
		.unwrap_or(LUNG_MSG)
}

/// Restituisce `true` se la variabile d'ambiente `ATTIVA_AVVERSARIO` è impostata
/// su uno dei valori `1`, `true` (case-insensitive). Altrimenti ritorna il default.
pub fn attiva_avversario() -> bool {
	std::env::var("ATTIVA_AVVERSARIO").ok()
		.and_then(|s| match s.as_str() {
			"1" | "true" | "True" | "TRUE" => Some(true),
			"0" | "false" | "False" | "FALSE" => Some(false),
			_ => None,
		})
		.unwrap_or(ATTIVA_AVVERSARIO)
}
