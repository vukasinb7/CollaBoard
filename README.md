# CollaBoard

## Opis problema
Potrebno je implementirati aplikaciju omogućuje višekorisničko pisanje/crtanje po tabli. Korisnicima je potrebno omogućiti lak pristup tabli za crtanje sa mnoštvom alata i mogućnosti, kao i lako i jednostavno deljenje i čuvanje njihovih radova. Projekat ima za cilj razvijanje visoko performantne kolaborativne table koja omogućava interakciju više korisnika na tabli u realnom vremenu. Backend sistema će biti implementiran koristeći programski jezik Rust kako bi se osigurala low latency osobina koja je kljucna za ovakav tip aplikacije, uz dodatno korišćenje WebAssembly tehnologije za optimizaciju korisničkog interfejsa.

## Funkcionalni zahtevi
### 1. **Registracija korisnika**
   Korisnik može da se registruje na sistem unosom korisničkog imena, imejl adrese i lozinke. Neregistrovani korisnik može da kreira novu tablu, bez mogućnosti deljenja sadržaja sa drugim korisnicima, ali uz mogućnost izvoza sadržaja.

### 2. **Prijavljivanje korisnika**
   Korisnik može da se prijavi na sistem unosom kredencijala. Nakon uspešnog prijavljivanja korisniku je na raspolaganju početna stranica sa tablama u kojima je on autor, kolaborator ili gledalac. Takođe, korisnik ima opciju za kreiranje nove table.

### 3. **Kreiranje table**  
   Korisnik može da kreira sadržaj zadavanjem imena table nakon čega mu se dodaje opcija da dodaje korisnike i dodeljuje prava pristupa kreiranjem jednokratnog invitacionog linka.

### 4. **Alati**
   Korisniku je u uglu ekrana dostupan set alata i podešavanja. Neki od alata i podešavanja su: olovka, tekst, geometrijski oblici, linija, gumica, undo/redo operacija, podešavanje debljine i boje unosa.

### 5. **Izvoz table**
   Korisniku je potrebno omogućiti izvoz table u jednom ili više najčešćih formata (JPG,PDF,PNG).

### 6. **Brisanje table**
   Autor ima mogućnost da obriše kompletanu tablu. Nakon toga nijedan kolaborator/gledalac neće imati pristup toj tabli.    

### 7. **Deljenje table**
   Nakon kreiranja, ili bilo kada u toku crtanja/pisanja vlasnik može da pozove druge korisnike da pristupe tabli putem linka (i/ili koda). Postoje tri nivoa pristupa sadržaju:
   1. Nivo - Gledalac - korisniku je omogućeno da gleda i izvozi sadržaj, ali ne i da ga menja.
   2. Nivo - Kolaborator - korisniku je omogućeno da gleda, menja i izvozi sadržaj.
   3. Nivo - Autor - korisniku je omogućeno da gleda,menja i izvozi sadržaj kao i da upravlja pravima pristupa ostalih korisnika. Takođe, korisnik može da obriše celu datoteku.
      
