# CollaBoard
### Backend
- Axum HTTP Server
- Axum WS Server
- Diesel ORM & Query Builder
- Lettre Email Client

### Frontend
- Yew Framework
- WASM Bindgen to bind [Excalidraw](https://docs.excalidraw.com/)'s JS API to Rust functions

## Demo

https://github.com/vukasinb7/CollaBoard/assets/51921035/dc414dd7-98a6-4a12-8809-2d0bbb393b42


## Opis problema
Potrebno je implementirati aplikaciju koja omogućava kolaborativno pisanje/crtanje po tabli. Korisnicima je potrebno omogućiti lak pristup tabli za crtanje sa mnoštvom alata i mogućnosti, kao i lako i jednostavno deljenje i čuvanje njihovih radova. Projekat ima za cilj razvijanje visoko performantne kolaborativne table koja omogućava interakciju više korisnika na tabli u realnom vremenu. Backend sistema će biti implementiran koristeći programski jezik Rust kako bi se osigurala low latency osobina koja je kljucna za ovakav tip aplikacije, uz dodatno korišćenje  WebAssembly tehnologije, Yew framework. Za vizualizaciju tabele koristiće se  WASM-Bindgen kao veza sa nekom od popularnih JS biblioteka za Canvas (konkretna JS biblioteka će biti odabrana tokom implementacije). Više korisnika će se realizovati pomoću WS servera koji radi na principu room sistema i on će voditi računa o korisnicima unutar sobe.

## Funkcionalni zahtevi
### 1. **Registracija korisnika**
   Korisnik može da se registruje na sistem unosom korisničkog imena, imejl adrese i lozinke. Neregistrovani korisnik može da kreira novu tablu, bez mogućnosti deljenja sadržaja sa drugim korisnicima, ali uz mogućnost izvoza sadržaja.

### 2. **Prijavljivanje korisnika**
   Korisnik može da se prijavi na sistem unosom kredencijala. Nakon uspešnog prijavljivanja korisniku je na raspolaganju početna stranica sa tablama u kojima je on autor, kolaborator ili gledalac. Takođe, korisnik ima opciju za kreiranje nove table.

### 3. **Kreiranje table**  
   Korisnik može da kreira sadržaj zadavanjem imena table nakon čega mu se dodaje opcija da dodaje korisnike i dodeljuje prava pristupa kreiranjem jednokratnog invitacionog linka.

### 4. **Alati**
   Korisniku je u uglu ekrana dostupan set alata i podešavanja. Neki od alata i podešavanja su: olovka, geometrijski oblici, linija, gumica, undo/redo operacija, podešavanje debljine i boje unosa.

### 5. **Izvoz table**
   Korisniku je potrebno omogućiti izvoz table u jednom ili više najčešće korišćenih formata (JPG, PDF, PNG).

### 6. **Brisanje table**
   Autor ima mogućnost da obriše kompletnu tablu. Nakon toga nijedan kolaborator/gledalac neće imati pristup toj tabli.    

### 7. **Deljenje table**
   Nakon kreiranja, ili bilo kada u toku crtanja/pisanja autor može da pozove druge korisnike da pristupe tabli putem linka (i/ili koda). Postoje tri nivoa pristupa sadržaju:
   1. Nivo - Gledalac - korisniku je omogućeno da gleda i izvozi sadržaj, ali ne i da ga menja.
   2. Nivo - Kolaborator - korisniku je omogućeno da gleda, menja i izvozi sadržaj.
   3. Nivo - Autor - korisniku je omogućeno da gleda,menja i izvozi sadržaj kao i da upravlja pravima pristupa ostalih korisnika. Takođe, korisnik može da obriše celu tablu.
      
