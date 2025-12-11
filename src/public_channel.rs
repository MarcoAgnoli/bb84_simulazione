use crate::config::lung_msg;

/// Canale Pubblico
/// Gestisce vettori condivisi e variabili booleane come da specifica.
#[derive(Debug, Clone)]
pub struct PublicChannel {
    // Vettore di polarizzazioni pubblicato dallo scrittore (Z/X) di lunghezza LUNG_MSG
    pub canale_pubblico: Vec<char>,
    // Vettore di esito letture (true se polarizzazione lettore == polarizzazione scrittore)
    pub sequenza_ricezione: Vec<bool>,
    // Valori di test avversario inviati dal lettore e letti dallo scrittore
    pub test_avversario: Vec<u8>,

    // Flag di stato
    pub pubblicazione_pronta: bool,
    pub fine_lettura: bool,
    pub sequenza_polarizzazioni_pronta: bool,
    pub test_avversario_pronto: bool,
    pub chiave_simmetrica_ok: bool,
    pub processo_terminato: bool,
}

impl PublicChannel {
    /// Inizializzazione del canale pubblico
    pub fn new() -> Self {
        Self {
            canale_pubblico: vec![' '; lung_msg()],
            sequenza_ricezione: vec![false; lung_msg()],
            test_avversario: Vec::new(),
            pubblicazione_pronta: false,
            fine_lettura: false,
            sequenza_polarizzazioni_pronta: false,
            test_avversario_pronto: false,
            chiave_simmetrica_ok: false,
            processo_terminato: false,
        }
    }

    /// Pubblicazione polarizzazione dei fotoni trasmessi (scrittore -> canale)
    pub fn pubblica_polarizzazioni(&mut self, polarizzazioni: Vec<char>) {
        assert_eq!(polarizzazioni.len(), lung_msg());
        self.canale_pubblico = polarizzazioni;
        self.pubblicazione_pronta = true;
    }

    /// Lettura polarizzazione dei fotoni trasmessi (lettore)
    pub fn leggi_polarizzazioni(&mut self) -> Vec<char> {
        self.pubblicazione_pronta = false;
        self.canale_pubblico.clone()
    }

    /// Fine lettura (lettore -> scrittore)
    pub fn fine_lettura(&mut self) {
        self.fine_lettura = true;
    }

    /// Spedizione sequenza ricezione fotoni (lettore -> scrittore)
    pub fn invia_sequenza_ricezione(&mut self, esito: Vec<bool>) {
        assert_eq!(esito.len(), lung_msg());
        self.sequenza_ricezione = esito;
        self.sequenza_polarizzazioni_pronta = true;
    }

    /// Lettura sequenza ricezione fotoni (scrittore)
    pub fn leggi_sequenza_ricezione(&mut self) -> Vec<bool> {
        self.sequenza_polarizzazioni_pronta = false;
        self.sequenza_ricezione.clone()
    }

    /// Scrittura test avversario (lettore)
    pub fn scrivi_test_avversario(&mut self, test: Vec<u8>) {
        self.test_avversario = test;
        self.test_avversario_pronto = true;
    }

    /// Lettura test avversario (scrittore)
    pub fn leggi_test_avversario(&mut self) -> Vec<u8> {
        self.test_avversario_pronto = false;
        self.test_avversario.clone()
    }

    /// Segnala che la chiave simmetrica è stata definita correttamente (scrittore)
    pub fn chiave_simmetrica_ok(&mut self) {
        self.chiave_simmetrica_ok = true;
    }

    /// Segnala che il processo è terminato (scrittore)
    pub fn processo_terminato(&mut self) {
        self.processo_terminato = true;
    }
}
