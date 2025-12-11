mod config;
mod public_channel;
mod quantum_channel;
mod writer;
mod reader;
mod adversary;

use config::{LUNG_MSG, ATTIVA_AVVERSARIO};
use public_channel::PublicChannel;
use quantum_channel::QuantumChannel;
use writer::Writer;
use reader::Reader;
use adversary::Adversary;

fn main() {
    // Inizializza canali e attori
    let mut canale_pubblico = PublicChannel::new();
    let mut canale_quantistico = QuantumChannel::new();
    let mut scrittore = Writer::new();
    let mut lettore = Reader::new();
    let mut avversario = Adversary::new();

    // 1) Scrittore inizializza il messaggio
    scrittore.inizializzazione();

    // 2) Trasmissione sequenziale di LUNG_MSG fotoni sul canale quantistico
    for i in 0..LUNG_MSG {
        // Scrittore spedisce il fotone i e attiva Fotone_IN
        scrittore.scrivi_su_canale_quantistico(&mut canale_quantistico, i);

        // Avversario (se attivo) legge per primo
        avversario.leggi_fotone_se_attivo(&mut canale_quantistico);

        // Lettore legge e resetta Fotone_IN
        // (nel nostro Reader, il reset avviene dentro la chiamata di lettura iterativa)
        // Per rispettare la specifica "un fotone alla volta" leggiamo esplicitamente qui.
        {
            use rand::Rng;
            use crate::config::{POL_X, POL_Z};
            let mut rng = rand::thread_rng();
            let pol_let = if rng.gen_bool(0.5) { POL_Z } else { POL_X };
            let val = canale_quantistico.lettura_fotone(pol_let);
            lettore.messaggio_quantistico_ricevuto.push((pol_let, val));
            canale_quantistico.set_fotone_out();
        }
    }

    // 3) Segnalazioni post-lettura
    if ATTIVA_AVVERSARIO {
        println!("[Avversario]: Lettura completata");
    }

    // Il lettore segnala fine lettura
    lettore.segnala_fine_lettura(&mut canale_pubblico);
    println!("[Lettore]: Lettura terminata");

    // 4) Lo scrittore pubblica le polarizzazioni utilizzate
    scrittore.pubblicazione_polarizzazione(&mut canale_pubblico);

    // 5) Il lettore attende pubblicazione e poi legge polarizzazioni,
    //    calcola esito letture e lo invia
    let polarizzazioni_scrittore = canale_pubblico.leggi_polarizzazioni();
    lettore.elabora_e_invia_esito(&mut canale_pubblico, polarizzazioni_scrittore);

    // 6) Lo scrittore attende che la sequenza sia pronta, seleziona chiave grezza
    if canale_pubblico.sequenza_polarizzazioni_pronta {
        scrittore.selezione_chiave_grezza(&mut canale_pubblico);
    }

    // 7) Il lettore invia i bit di test avversario sul canale pubblico
    lettore.invia_test_avversario(&mut canale_pubblico);

    // 8) Lo scrittore verifica il test e, se negativo, definisce la chiave finale,
    //    quindi termina il processo
    scrittore.selezione_test_e_chiave_finale(&mut canale_pubblico);

    // 9) Il lettore attende il termine del processo e conferma l'esito
    lettore.conferma_finale(&canale_pubblico);

    // 10) Stampa delle tabelle richieste
    stampa_tabelle(&scrittore, &lettore, &avversario);
}

/// Stampa tabellare: sequenza fotoni, chiavi finali, statistiche
fn stampa_tabelle(scr: &Writer, lettr: &Reader, avv: &Adversary) {
    use crate::config::ATTIVA_AVVERSARIO;

    println!("
=== Sequenza fotoni ===");
    println!("{:<6} | {:<20} | {:<20} | {:<20}", "Index", "Scrittore (pol,val)", "Avversario (pol,val)", "Lettore (pol,val)");
    println!("{}", "-".repeat(75));
    for i in 0..scr.messaggio_quantistico.len() {
        let (pol_s, val_s) = scr.messaggio_quantistico[i];
        let (pol_a, val_a) = if ATTIVA_AVVERSARIO {
            avv.avversario_messaggio_quantistico_ricevuto.get(i).cloned().unwrap_or((' ', 0))
        } else { ('-', 0) };
        let (pol_l, val_l) = lettr.messaggio_quantistico_ricevuto[i];
        println!(
            "{:<6} | {:<20} | {:<20} | {:<20}",
            i,
            format!("({}, {})", pol_s, val_s),
            if ATTIVA_AVVERSARIO { format!("({}, {})", pol_a, val_a) } else { "-".to_string() },
            format!("({}, {})", pol_l, val_l)
        );
    }

    println!("
=== Chiavi finali ===");
    println!("Scrittore: {:?}", scr.chiave_simmetrica);
    println!("Lettore  : {:?}", lettr.chiave_simmetrica);

    println!("
=== Statistiche ===");
    let tot_fotoni = scr.messaggio_quantistico.len();
    let scartati_diff_polarizzazioni = lettr.esito_letture.iter().filter(|&&b| !b).count();
    let selezionati_iniziali = lettr.esito_letture.iter().filter(|&&b| b).count();
    let test_bits = if lettr.chiave_grezza.len() > 0 { ((lettr.chiave_grezza.len() - 1) / 8) + 1 } else { 0 };
    let lung_chiave_finale = scr.chiave_simmetrica.len();

    fn perc(x: usize, tot: usize) -> f64 { if tot == 0 { 0.0 } else { (x as f64) * 100.0 / (tot as f64) } }

    println!("Fotoni totali (iniziali)              : {:>3} ({:>5.1}%)", tot_fotoni, perc(tot_fotoni, tot_fotoni));
    println!("Valori scartati per differenza pol.   : {:>3} ({:>5.1}%)", scartati_diff_polarizzazioni, perc(scartati_diff_polarizzazioni, tot_fotoni));
    println!("Valori selezionati inizialmente chiave: {:>3} ({:>5.1}%)", selezionati_iniziali, perc(selezionati_iniziali, tot_fotoni));
    println!("Valori scartati per test avversario   : {:>3} ({:>5.1}%)", test_bits, perc(test_bits, tot_fotoni));
    println!("Lunghezza chiave finale               : {:>3} ({:>5.1}%)", lung_chiave_finale, perc(lung_chiave_finale, tot_fotoni));
}
