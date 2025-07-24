# Documentație Proiect Trap-The-Mouse

## Contribuitori
- **Unic Contribuitor:** [COSTELINOO](https://github.com/COSTELINOO)

## Cerința problemei

**Trap-The-Mouse** este un joc implementat cu două componente principale:

1. **Server:**
   - Găzduiește jocul și permite utilizatorilor să creeze camere.
   - Jucătorii pot aștepta ca alți utilizatori să se alăture camerei.
   - Jocul începe automat când sunt conectați doi jucători.

2. **Client:**
   - O interfață grafică (GUI) care permite utilizatorilor să se conecteze la camerele de joc.
   - Oferă posibilitatea de a juca împotriva altor jucători sau împotriva calculatorului.
   - În cazul jocului cu calculatorul, serverul gestionează mișcările șoarecelui.

---

## Restricții și limitări

Aplicațiile dezvoltate trebuie să respecte următoarele cerințe:

- Să utilizeze ediția 2021 a limbajului Rust pe un compilator stabil.
- Să compileze fără erori sau avertismente (`cargo check`).
- Să compileze fără avertismente generate de Clippy (`cargo clippy`).
- Avertismentele trebuie rezolvate corect, nu doar ascunse (de exemplu, să nu se folosească `_` pentru a ascunde variabile neutilizate).
- Erorile trebuie propagate corect până la funcția `main`, acolo unde este cazul.
- Să nu utilizeze cod `unsafe`, decât dacă este specificat altfel.
- Să treacă testele efectuate cu Miri (`cargo miri run`) dacă proiectul folosește `unsafe`.
- Să răspundă corect la argumentele liniei de comandă, incluzând o comandă de `help` care afișează comenzile disponibile.

---

## Build and Run
Proiectul se poate compila și rula prin executarea scriptului `run.bat` (destinat pentru Windows).

## Videoclip de prezentare
[Link către videoclipul de prezentare a aplicației](https://drive.google.com/file/d/1NNHzOCsK_4RKgm1YyTm_BBqVB3vNrSy8/view?usp=drive_link)

## Capturi de ecran
<img width="1917" height="1017" alt="Screenshot 2025-07-24 143151" src="https://github.com/user-attachments/assets/835c5e25-d6ba-4303-b767-fdfe95d68343" />

<img width="1907" height="1010" alt="Screenshot 2025-07-24 143217" src="https://github.com/user-attachments/assets/870517ec-9aad-4e81-b1fa-a7d6a8308798" />
<img width="1918" height="1012" alt="Screenshot 2025-07-24 143231" src="https://github.com/user-attachments/assets/eb52269e-2208-4f3b-bafa-135b9ad7ea39" />
<img width="1918" height="1010" alt="Screenshot 2025-07-24 143255" src="https://github.com/user-attachments/assets/7c78f65e-802e-41fa-bc03-7b2c3cb62eba" />
<img width="1917" height="1012" alt="Screenshot 2025-07-24 143310" src="https://github.com/user-attachments/assets/4b01d896-d3b5-4542-a577-955bb8baef1a" />
<img width="1915" height="1010" alt="Screenshot 2025-07-24 143323" src="https://github.com/user-attachments/assets/a1376669-c14e-42d5-b9de-61f534641a78" />
<img width="1918" height="997" alt="Screenshot 2025-07-24 143338" src="https://github.com/user-attachments/assets/e08ef6fd-ea32-4a4e-b064-f693531497e7" />
<img width="1917" height="1002" alt="Screenshot 2025-07-24 143352" src="https://github.com/user-attachments/assets/53e92cea-b23a-46d7-8619-942579022dbe" />
<img width="1918" height="1016" alt="Screenshot 2025-07-24 143411" src="https://github.com/user-attachments/assets/3e9c9002-bcb0-4995-890b-b6bbe3295d03" />
<img width="1918" height="1007" alt="Screenshot 2025-07-24 143437" src="https://github.com/user-attachments/assets/d12b33dc-3242-4290-bef1-e03697f6d654" />
<img width="1918" height="1010" alt="Screenshot 2025-07-24 143505" src="https://github.com/user-attachments/assets/ebe95fa2-31b6-4106-a411-765f6436e374" />
<img width="1915" height="1002" alt="Screenshot 2025-07-24 143641" src="https://github.com/user-attachments/assets/090437d0-c2be-437f-920f-1904c1016527" />

## Documentație Proiect Trap-The-Mouse

### Structura aplicației

Proiectul este format din două module principale:
1. **Server (`Trap_the_mouse_server`)**
2. **Client (`Trap_the_mouse_client`)**

---

### 1. Server

#### Funcționalități principale

1. Găzduirea camerelor de joc:
   - Crearea camerelor în care jucătorii se pot alătura.
   - Managementul a până la 2 jucători per cameră.
   - Atribuirea rolurilor (`Hunter` și `Mouse`) jucătorilor.

2. Gestionarea jocului:
   - Mișcările jucătorilor sunt validate și sincronizate între clienți.
   - În cazul unui joc împotriva calculatorului, serverul controlează mișcările șoarecelui.

3. Joc multiplayer:
   - Server-ul gestionează comunicarea dintre jucători folosind protocoale TCP.

4. Joc împotriva calculatorului:
   - Server-ul generează obstacole și determină mișcările șoarecelui utilizând algoritmi.

#### Structura codului

- **`main.rs`**: Punctul de intrare al serverului. Gestionează conexiunile TCP și comenzile primite de la clienți.
- **`drum.rs`**: Conține funcții pentru generarea obstacolelor și determinarea drumului optim pentru șoarece folosind BFS.

#### Dependențe

- **Tokio**: Gestionarea conexiunilor asincrone.
- **Rand**: Generarea numerelor aleatorii pentru plasarea obstacolelor.
- **Rand-ChaCha**: Generator de numere pseudo-aleatorii pentru mișcările șoarecelui.

---

### 2. Client

#### Funcționalități principale

1. Interfață grafică:
   - Utilizatorii pot naviga între diverse pagini (e.g., `Menu`, `SinglePlayer`, `TwoPlayers`).
   - Jocul afișează tabla de joc și permite interacțiunea utilizatorilor.

2. Conectare la server:
   - Clientul poate crea sau se poate alătura unei camere de joc.
   - Poate iniția un joc împotriva calculatorului.

3. Managementul stării jocului:
   - Clientul primește și procesează mesajele de la server pentru a actualiza starea jocului.

#### Structura codului

- **`main.rs`**: Punctul de intrare al aplicației client. Inițializează interfața grafică și conexiunea la server.
- **`model.rs`**: Definirea modelelor utilizate în aplicație (e.g., paginile GUI, rolurile jucătorilor).
- **`view.rs`**: Afișarea interfeței grafice folosind biblioteca `iced`.
- **`update.rs`**: Gestionarea evenimentelor și actualizarea stării aplicației.
- **`messages.rs`**: Definirea mesajelor utilizate pentru navigare și interacțiunea cu serverul.
- **`parser.rs`**: Procesarea mesajelor primite de la server.

#### Dependențe

- **Iced**: Bibliotecă pentru dezvoltarea interfețelor grafice.
- **Tokio**: Gestionarea conexiunilor asincrone.

---

### 3. Flux de date

1. **Crearea unei camere:**
   - Clientul trimite comanda `create` către server.
   - Serverul creează camera și răspunde cu mesajul `READY`.

2. **Alăturarea la o cameră:**
   - Clientul trimite comanda `join` cu un PIN specific.
   - Serverul sincronizează starea jocului și atribuie rolurile jucătorilor.

3. **Joc împotriva calculatorului:**
   - Clientul trimite comanda `computer`.
   - Serverul generează mișcările șoarecelui și răspunde cu starea actualizată a jocului.

---

### 4. Tehnologii utilizate

- **Rust (ediția 2021):** Limbajul principal de programare.
- **Cargo:** Sistem de build și gestionare a dependențelor.
- **Tokio:** Gestionarea asincronă a conexiunilor.
- **Iced:** Bibliotecă pentru GUI.
- **Rand și Rand-ChaCha:** Generarea numerelor pseudo-aleatorii.

---

### 5. Configurații

#### Server

- **Adresa:** `127.0.0.1`
- **Port:** `9090`

#### Client

- **Dimensiuni inițiale fereastră:** `800x670`
- **Culori tematice:** Definite în fișierul `forme.rs`.

---

### 6. Considerații finale

Proiectul **Trap-The-Mouse** respectă cerințele privind compilarea fără avertismente și utilizarea unui model robust de gestionare a conexiunilor și a interfeței grafice. Este o aplicație scalabilă, modulară și ușor de extins.
