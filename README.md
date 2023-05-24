launch application with:
bash make.sh

TODO: 

Design
- [ ] Add picture for random page
- [ ] Update wikitty picture
- [ ] Add favicon
- [ ] Complete information page
- [ ] Smartphone support
- [x] Add shadow to indicate which page we are on
- [x] Add message for failed loading
- [x] Fill preparation page
- [x] better navigation
- [x] better buttons
- [x] better word spacing
- [x] translate text to french
- [x] Add loading bar

Database
- [x] Add url/link to wiki page
- [ ] Filter page on length of content (not too small) and on title (not too long ?) ex: (www.wikitrouve.fr/guess/684132)

frontend bugs
- [ ] picking rating resets page scroll
- [ ] going back to wikitrouve does not reset shadow
- [x] when giving up, message should not be "trouvé en x mots"


Features
- [ ] Add team mode
- [ ] Add tutorial ?
- [ ] Add global score, median number of word to complete ?
- [ ] anglais -- equ britannique ?
- [ ] Add best words -- like previous words
- [x] Add easy mode - reveal some uncommon words ?
- [x] Add "important" word category, word that help complete the page
- [x] Add daily page !
- [x] Test daily page !
- [x] Make information page
- [x] Add https
- [x] add conjugation
- [x] add popup for confirmation of bug report and rating
- [x] remettre un mot doit surligner


Challenge mode (geoguesser like)

- [x] Time mode
- [ ] place timer on center
- [x] Mode avec beaucoup de mots déja révélés
- [x] Définir mot protégés par page
- [x] Marquer mot protégés
- [ ] améliorer mots protégés (ex: www.wikitrouve.fr/guess/3372)
- [ ] Définir points par mots, comptabilité par page (si peu de mots, pas beaucoup de points..)
- [ ] Ajouter bonus pathé
- [ ] Ajouter possibilité reveler certains mots (pas importants) ou de donner mot proche (top 10) ?

- enchainement de page défini avec choix de thème.
Si trouve pas titre, plus trouve de mot moins de perte
(croquettes. il doit rentrer à la maison si plus).
Il faut en perdre le moins, si tu pert tout remis à 0

- [ ] + choix de thème: bouffe, histoire, collège, animal ?



Bugs backend
- [x] fix CORS
- [x] Daily page should not be reloaded if already done
- [x] Daily page can be random page if random page was created before
- [x] After reloading duplicate words
- [x] Stop count after victory
- [x] 1er 2 mots
- [x] citations leave trailing commas - example: https://fr.wikipedia.org/wiki/Danielle_Darrieux
- [x] refuse words not in database
- [x] add word history without duplicate
- [x] mots en majuscules déja mis bug (add lower to comparison)
