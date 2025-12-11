use rand::Rng;
use crate::config::{POL_X, POL_Z};

/// Canale Quantistico
/// Mantiene una tupla (polarizzazione, valore) e un flag Fotone_IN.
#[derive(Debug, Clone)]
pub struct QuantumChannel {
    pub canale_quantistico: (char, u8), // (polarizzazione, valore)
    pub fotone_in: bool,                // true se è presente un fotone nel canale
}

impl QuantumChannel {
    /// Inizializzazione: polarizzazione vuota e valore 0, fotone_in = false
    pub fn new() -> Self {
        Self {
            canale_quantistico: (' ', 0),
            fotone_in: false,
        }
    }

    /// Spedizione del fotone (scrittore)
    pub fn spedizione_fotone(&mut self, polarizzazione: char, valore: u8) {
        assert!(polarizzazione == POL_Z || polarizzazione == POL_X);
        assert!(valore == 0 || valore == 1);
        self.canale_quantistico = (polarizzazione, valore);
    }

    /// Lettura del fotone (lettore/avversario)
    /// Se la polarizzazione di misura coincide, restituisce il valore.
    /// Se differisce, randomizza il valore (0/1), lo scrive e lo restituisce.
    pub fn lettura_fotone(&mut self, polarizzazione_misura: char) -> u8 {
        assert!(polarizzazione_misura == POL_Z || polarizzazione_misura == POL_X);
        let (pol_tx, val) = self.canale_quantistico;
        if pol_tx == polarizzazione_misura {
            // Polarizzazione allineata, valore invariato
            val
        } else {
            // Polarizzazione diversa: il valore collassa casualmente e sostituisce il precedente
            let mut rng = rand::thread_rng();
            let nuovo = rng.gen_range(0..=1);
            self.canale_quantistico = (pol_tx, nuovo);
            nuovo
        }
    }

    /// Settaggio Fotone_IN: true quando un nuovo fotone è nel canale
    pub fn set_fotone_in(&mut self) { self.fotone_in = true; }
    /// Settaggio Fotone_OUT: false quando la lettura termina
    pub fn set_fotone_out(&mut self) { self.fotone_in = false; }
}
