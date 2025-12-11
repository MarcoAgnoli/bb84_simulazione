use rand::Rng;
use crate::config::{ATTIVA_AVVERSARIO, LUNG_MSG, POL_X, POL_Z};
use crate::quantum_channel::QuantumChannel;

/// Avversario (opzionale): legge i fotoni sempre prima del lettore se attivato.
pub struct Adversary {
    pub avversario_messaggio_quantistico_ricevuto: Vec<(char, u8)>,
}

impl Adversary {
    pub fn new() -> Self {
        Self { avversario_messaggio_quantistico_ricevuto: Vec::with_capacity(LUNG_MSG) }
    }

    /// Lettura di un fotone (se ATTIVA_AVVERSARIO=true). Deve avvenire prima del lettore.
    pub fn leggi_fotone_se_attivo(&mut self, q: &mut QuantumChannel) {
        if ATTIVA_AVVERSARIO {
            let mut rng = rand::thread_rng();
            let pol_let = if rng.gen_bool(0.5) { POL_Z } else { POL_X };
            let valore = q.lettura_fotone(pol_let);
            self.avversario_messaggio_quantistico_ricevuto.push((pol_let, valore));
            // NOTA: l'avversario **non** resetta Fotone_IN; il reset è responsabilità del lettore
        }
    }
}
