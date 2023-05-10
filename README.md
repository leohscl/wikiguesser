launch application with:
bash make.sh

TODO: 


Design
- [ ] Add message for failed loading
- [ ] Add shadow to indicate which page we are on
- [ ] Add picture for random page
- [ ] Update wikitty picture
- [ ] Add favicon
- [ ] Complete information page
- [ ] Smartphone support
- [x] Fill preparation page
- [x] better navigation
- [x] better buttons
- [x] better word spacing
- [x] translate text to french
- [x] Add loading bar


frontend bugs
- [ ] picking rating resets page scroll


Features
- [ ] Add team mode
- [ ] Add tutorial ?
- [ ] Add global score, median number of word to complete ?
- [ ] Add easy mode - reveal some uncommon words ?
- [ ] anglais -- equ britannique ?
- [ ] Add "important" word category, word that help complete the page
- [ ] add url/link to wiki page
- [x] Add daily page !
- [x] Test daily page !
- [x] Make information page
- [x] Add https
- [x] add conjugation
- [x] add popup for confirmation of bug report and rating
- [x] remettre un mot doit surligner


Challenge mode (geoguesser like)

- [ ] Time mode
- [ ] si trouve pas titre, plus trouve de mot moins de perte (croquettes. il doit rentrer à la maison si plus). Il faut en perdre le moins, si tu pert tout remis à 0
- [ ] enchainement de page défini avec choix de thème
- [ ] mode avec beaucoup de mots déja définis
- [ ] + choix de thème: bouffe, histoire collège, animal ?
- [ ] bonus: reveler 1 mot ?



Bugs backend
- [ ] fix CORS
- [x] Daily page should not be reloaded if already done
- [x] Daily page can be random page if random page was created before
- [x] After reloading duplicate words
- [x] Stop count after victory
- [x] 1er 2 mots
- [x] citations leave trailing commas - example: https://fr.wikipedia.org/wiki/Danielle_Darrieux
- [x] refuse words not in database
- [x] add word history without duplicate
- [x] mots en majuscules déja mis bug (add lower to comparison)
