# BB84 – Simulazione senza concorrenza

Questa applicazione simula lo scambio di chiavi simmetriche con il protocollo **BB84** rispettando le specifiche fornite (nessuna concorrenza; l'avversario, se attivato, legge **prima** del lettore). È scritta in **Rust**, con un file per ogni oggetto e un `main`. Commenti estesi e codice verboso.

## Requisiti
- Rust (toolchain consigliata: stable)
- Cargo
- Visual Studio Code (opzionale)

## Esecuzione
```bash
cd bb84_simulazione
cargo run
```

## Struttura
- `src/config.rs` – costanti globali (es. `LUNG_MSG=8`, `ATTIVA_AVVERSARIO=false`).
- `src/public_channel.rs` – canale pubblico: vettori e flag condivisi e metodi di pubblicazione/lettura.
- `src/quantum_channel.rs` – canale quantistico: tupla (polarizzazione, valore) persistente e flag `Fotone_IN`.
- `src/writer.rs` – scrittore: inizializza messaggio, spedisce fotoni, pubblica polarizzazioni, costruisce chiave grezza e finale, test avversario.
- `src/reader.rs` – lettore: misura fotoni, invia esito letture, costruisce chiavi e invia test, conferma finale.
- `src/adversary.rs` – avversario (opzionale): legge i fotoni prima del lettore.
- `src/main.rs` – orchestrazione sequenziale e stampa delle tabelle richieste.
