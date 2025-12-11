use crate::config::LUNG_MSG;
use crate::public_channel::PublicChannel;

/// Lettore
/// Legge i fotoni, confronta le polarizzazioni pubblicate e invia esito/chiavi.
pub struct Reader {
    pub messaggio_quantistico_ricevuto: Vec<(char, u8)>,
    pub esito_letture: Vec<bool>,
    pub chiave_grezza: Vec<u8>,
    pub chiave_simmetrica: Vec<u8>,
    pub test_avversario: Vec<u8>,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            messaggio_quantistico_ricevuto: Vec::with_capacity(LUNG_MSG),
            esito_letture: vec![false; LUNG_MSG],
            chiave_grezza: Vec::new(),
            chiave_simmetrica: Vec::new(),
            test_avversario: Vec::new(),
        }
    }

    // Nota: la funzione `leggi_tutti_i_fotoni` è stata rimossa perché non utilizzata
    // nel flusso principale; la lettura viene gestita esplicitamente in `main.rs`.

    /// Fine lettura sul canale pubblico
    pub fn segnala_fine_lettura(&self, p: &mut PublicChannel) {
        p.fine_lettura();
    }

    /// Confronta polarizzazioni pubblicate dallo scrittore e invia esito letture
    pub fn elabora_e_invia_esito(&mut self, p: &mut PublicChannel, polarizzazioni_scrittore: Vec<char>) {
        // Itera insieme alle polarizzazioni pubblicate evitando l'uso di `0..LUNG_MSG`.
        for (i, &pol_pub) in polarizzazioni_scrittore.iter().enumerate() {
            let (pol_let, _) = self.messaggio_quantistico_ricevuto[i];
            self.esito_letture[i] = pol_let == pol_pub;
        }
        p.invia_sequenza_ricezione(self.esito_letture.clone());
        println!("[Lettore]: Invio esito confronto polarizzazioni completato");

        // Costruisce chiave grezza (valori con polarizzazione allineata)
        self.chiave_grezza.clear();
        for i in 0..LUNG_MSG {
            if self.esito_letture[i] {
                let (_, val) = self.messaggio_quantistico_ricevuto[i];
                self.chiave_grezza.push(val);
            }
        }

        // Prepara test avversario (un bit ogni 8, partendo dal primo)
        self.test_avversario.clear();
        let mut idx = 0usize;
        while idx < self.chiave_grezza.len() {
            self.test_avversario.push(self.chiave_grezza[idx]);
            idx += 8;
        }

        // Chiave simmetrica locale (rimuove i bit usati per test)
        self.chiave_simmetrica.clear();
        for (i, b) in self.chiave_grezza.iter().enumerate() {
            if i % 8 != 0 { self.chiave_simmetrica.push(*b); }
        }
    }

    /// Scrive sul canale pubblico il test avversario
    pub fn invia_test_avversario(&self, p: &mut PublicChannel) {
        p.scrivi_test_avversario(self.test_avversario.clone());
    }

    /// Attende processo terminato e stampa conferma finale
    pub fn conferma_finale(&mut self, p: &PublicChannel) {
        if p.chiave_simmetrica_ok {
            println!("[Lettore]: Confermo definizione chiave simmetrica {:?}", self.chiave_simmetrica);
        } else {
            println!("[Lettore]: Confermo presenza avversari, chiave simmetrica cancellata");
            // Cancella il valore della chiave simmetrica
            self.chiave_simmetrica.clear();
        }
    }
}
