<div align="center">
    <h1>MyGCPay CLI</h1>
    <img src="logo.png" width="230">
    <br/>

[Voir la version anglaise](./README.md)

</div>


## Description

MyGCPay CLI est un client de ligne de commande Rust NON OFFICIEL pour travailler avec les donnees du portail MaPayeGC du gouvernement du Canada.

Il est concu pour offrir un acces local et scriptable a :

- votre cookie de session MaPayeGC
- les listes de cheques de paye et les details des cheques de paye
- le cache local de l'application et son repertoire de donnees
- un calendrier integre des dates de paye et des autres dates annotees

L'outil produit du JSON pour les commandes axees sur les donnees, ce qui le rend pratique autant pour une utilisation directe au terminal que pour l'automatisation dans PowerShell ou d'autres scripts shell.

## Fonctionnalites

- enregistrer et valider le cookie de session MaPayeGC utilise pour les requetes authentifiees
- lister les cheques de paye provenant de MaPayeGC
- recuperer les details d'un cheque de paye precis
- recuperer au besoin les details complets de tous les cheques de paye en une seule commande
- afficher la forme des reponses des commandes de cheque de paye sans effectuer de requete
- afficher le repertoire d'accueil de l'application et le repertoire de cache
- effacer les donnees mises en cache
- afficher un calendrier local des dates de paye, des jours feries et d'autres annotations
- produire des journaux NDJSON structures au besoin

## Compilation et installation

### Prerequis

- Rust 1.94 ou une version plus recente
- un acces a MaPayeGC dans un navigateur web

### Compiler localement

```powershell
cargo build --release
```

L'executable compile se trouvera a l'emplacement suivant :

```text
target/release/mgcp.exe
```

### Executer a partir du code source

```powershell
cargo run -- --help
```

### Installer dans le repertoire binaire de Cargo

```powershell
cargo install --path .
```

Apres l'installation, vous pouvez utiliser :

```powershell
mgcp --help
```

## Apercu des commandes

```text
mgcp cookie ...
mgcp paycheque ...
mgcp cache ...
mgcp home ...
mgcp calendar ...
```

Options globales communes :

- `--debug` active la journalisation de debogage et les traces detaillees lors des paniques
- `--log-filter <DIRECTIVE>` definit le filtre de journalisation
- `--log-file <FILE|DIR>` ecrit des journaux NDJSON structures

## Authentification

MyGCPay CLI utilise votre cookie de session MaPayeGC pour effectuer des requetes authentifiees.

Pour capturer et enregistrer le cookie :

1. Visitez <https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/>.
2. Ouvrez les outils de developpement du navigateur et allez a l'onglet Reseau.
3. Actualisez la page.
4. Reperez une requete vers `paycheque-data/`.
5. Cliquez avec le bouton droit sur la requete et choisissez `Copy > Copy request headers`.
6. Acheminez les en-tetes copies vers l'outil en ligne de commande.

Exemple PowerShell :

```powershell
Get-Clipboard | mgcp cookie set
```

Vous pouvez aussi fournir directement la valeur du cookie :

```powershell
mgcp cookie set "Cookie: <votre-valeur-de-cookie>"
```

Verifier si un cookie est actuellement enregistre :

```powershell
mgcp cookie check
```

Effacer le cookie enregistre :

```powershell
mgcp cookie clear
```

## Exemples d'utilisation

### Lister les cheques de paye

Lister les cheques de paye actuellement visibles :

```powershell
mgcp paycheque list
```

Inclure les cheques de paye archives et recuperer les details complets de chacun :

```powershell
mgcp paycheque list --all
```

Ajuster le delai entre les requetes detaillees lors de l'utilisation de `--all` :

```powershell
mgcp paycheque list --all --sleep 1s
```

Examiner le schema de sortie sans effectuer de requete :

```powershell
mgcp paycheque list --shape
mgcp paycheque list --all --shape
```

### Afficher un cheque de paye

Recuperer un cheque de paye par identifiant :

```powershell
mgcp paycheque show <ID>
```

Examiner le schema de la reponse detaillee :

```powershell
mgcp paycheque show <ID> --shape
```

### Afficher les donnees du calendrier

Afficher les dates annotees pour l'annee en cours :

```powershell
mgcp calendar show
```

Afficher une annee precise :

```powershell
mgcp calendar show 2026
```

La sortie du calendrier comprend des attributs comme `PayDate`, `Holiday`, `CivicHoliday` et `NationalQuebecHoliday`.

### Examiner les repertoires locaux

Afficher le repertoire d'accueil de l'application :

```powershell
mgcp home path show
```

Afficher le repertoire de cache :

```powershell
mgcp cache path show
```

Effacer le repertoire de cache :

```powershell
mgcp cache clean
```

## Sortie et journalisation

Les commandes de donnees affichent du JSON formate sur la sortie standard. Cela rend l'outil facile a combiner avec d'autres commandes.

Exemple :

```powershell
mgcp paycheque list | Out-File paycheques.json
```

Pour ecrire des journaux structures :

```powershell
mgcp --log-file .\logs\ paycheque list
```

Pour activer la journalisation de debogage :

```powershell
mgcp --debug paycheque list
```

## Notes

- `mgcp paycheque list --all` effectue une requete par detail de cheque de paye apres avoir recupere la liste.
- `mgcp cookie set` accepte soit une valeur brute de cookie, soit des en-tetes de requete copies via l'entree standard.
- Si le cookie enregistre expire, repetez le processus de capture du cookie.

## Developpement

Commandes utiles pendant le developpement du projet :

```powershell
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## Droits d’auteur

Droits d’auteur appartiennent à © Sa Majesté le Roi du chef du Canada, qui est représenté par le ministre de l’Agriculture et de l’Agroalimentaire, 2025.