# rust_20250131

```
cargo run -- -n Jim open
cargo run -- -n Oscar join <ticket>

```
## Hva er iroh?
I følge iroh er det et peer-to-peer rammeverk som "bare virker". Iroh gjør det enkelt for konsumenter å opprette direkte
tilkobling til hverandre ( peer to peer ). Nøyaktig hvordan dette fungerer kan dere lese mer om på https://www.iroh.computer/docs/overview.
Kort oppsummert bruker man en `NodeId` ( 32-byte ed25519 public key) til å koble til en annen node. Under panseret har de en rekke funksjoner og
fallbacks som gjør at "det bare virker."

I iroh kan man kode egne protokoller for det man ønsker å gjøre. De har et sett med "standard" protokoller man kan bruke. `iroh-blobs` for filoverføring, `iroh-gossip` for 
sending av meldinger og `iroh-docs` for realtime multiwriting ( google docs feks.)

I dag skal vi bygge en chat basert på `iroh-gossip`


# Oppgaver
1. cli_parser.rs
3. broadcast_about_me.rs
4. listen_and_send_message.rs
5. subscribe_loop.rs
