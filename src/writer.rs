use rand::Rng;
use crate::config::{lung_msg, POL_X, POL_Z};
use crate::public_channel::PublicChannel;
use crate::quantum_channel::QuantumChannel;

/// Scrittore
/// Genera un messaggio quantistico e gestisce la pubblicazione e la selezione delle chiavi.
pub struct Writer {
    pub messaggio_quantistico: Vec<(char, u8)>, // vettore di (polarizzazione, valore)
    pub chiave_grezza: Vec<u8>,
    pub chiave_simmetrica: Vec<u8>,
    pub test_avversario: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            messaggio_quantistico: Vec::with_capacity(lung_msg()),
            chiave_grezza: Vec::new(),
            chiave_simmetrica: Vec::new(),
            test_avversario: Vec::new(),
        }
    }

    /// Inizializzazione: genera LUNG_MSG fotoni con polarizzazione (Z/X) e valore (0/1) casuali
    pub fn inizializzazione(&mut self) {
        println!("[Scrittore]: Inizializzazione e scelta messaggio quantistico");
        let mut rng = rand::thread_rng();
        for _ in 0..lung_msg() {
            let pol = if rng.gen_bool(0.5) { POL_Z } else { POL_X };
            let val = rng.gen_range(0..=1);
            self.messaggio_quantistico.push((pol, val));
        }
    }

    /// Scrive sul canale quantistico un fotone alla volta e attiva Fotone_IN
    pub fn scrivi_su_canale_quantistico(&self, q: &mut QuantumChannel, indice: usize) {
        let (pol, val) = self.messaggio_quantistico[indice];
        q.spedizione_fotone(pol, val);
        q.set_fotone_in();
    }

    /// Pubblica la sequenza delle polarizzazioni utilizzate sul canale pubblico
    pub fn pubblicazione_polarizzazione(&self, p: &mut PublicChannel) {
        let polarizzazioni: Vec<char> = self
            .messaggio_quantistico
            .iter()
            .map(|(pol, _)| *pol)
            .collect();
        p.pubblica_polarizzazioni(polarizzazioni.clone());
    }

    /// Selezione chiave grezza a partire dall'esito letture
    pub fn selezione_chiave_grezza(&mut self, p: &mut PublicChannel) {
        let esito = p.leggi_sequenza_ricezione();
        self.chiave_grezza.clear();
        for (i, ok) in esito.iter().enumerate() {
            if *ok {
                let (_, val) = self.messaggio_quantistico[i];
                self.chiave_grezza.push(val);
            }
        }
    }

    /// Selezione bit di test, verifica presenza avversario e definizione chiave finale
    pub fn selezione_test_e_chiave_finale(&mut self, p: &mut PublicChannel) {
        // Prende un bit ogni 8 dalla chiave grezza, partendo dal primo
        self.test_avversario.clear();
        let mut idx = 0usize;
        while idx < self.chiave_grezza.len() {
            self.test_avversario.push(self.chiave_grezza[idx]);
            idx += 8;
        }

        // Attende che il lettore abbia scritto il suo test e lo legge
        // (nella nostra orchestrazione, il lettore scrive prima di arrivare qui)
        if p.test_avversario_pronto {
            let test_lettore = p.leggi_test_avversario();
            // Confronto
            let mut errore = false;
            for (a, b) in self.test_avversario.iter().zip(test_lettore.iter()) {
                if a != b { errore = true; break; }
            }
            if errore || self.test_avversario.len() != test_lettore.len() {
                println!("[Scrittore]: Test presenza avversario positivo. Chiave scartata");
                // Scrive sul terminale quanti bit sono stati confrontati e quanti sono risultati errati
                let mut errori_count = 0;
                for (a, b) in self.test_avversario.iter().zip(test_lettore.iter()) {
                    if a != b { errori_count += 1; }
                }
                if self.test_avversario.len() != test_lettore.len() {
                    errori_count += (self.test_avversario.len() as isize - test_lettore.len() as isize).unsigned_abs();
                }
                println!("[Scrittore]: Test confrontato su {} bit, con {} errori", self.test_avversario.len(), errori_count);    
                // Processo terminato senza chiave valida
                p.processo_terminato();
            } else {
                println!("[Scrittore]: Test presenza avversario negativo");
                // Costruisce la chiave simmetrica rimuovendo i bit usati per il test (uno ogni 8)
                self.chiave_simmetrica.clear();
                for (i, b) in self.chiave_grezza.iter().enumerate() {
                    if i % 8 != 0 { // i=0,8,16... sono usati per il test
                        self.chiave_simmetrica.push(*b);
                    }
                }
                println!("[Scrittore]: Chiave simmetrica definita: {:?}", self.chiave_simmetrica);
                p.chiave_simmetrica_ok();
                p.processo_terminato();
            }
        } else {
            // In casi limite, se non pronto, consideriamo test non disponibile
            println!("[Scrittore]: Test avversario non pronto: processo terminato senza chiave");
            p.processo_terminato();
        }
    }
}
