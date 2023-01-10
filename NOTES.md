le crate vcard est utile pour générer un vcard valide facilement
https://github.com/magiclen/vcard
parser:
https://docs.rs/vcard_parser/0.1.0/vcard_parser/


le programmme mate.rs ressemble à ce que je veux faire, mais porte une attention à l'intégration avec mutt. Assez limité mais exploitable.
https://github.com/pimutils/mates.rs
fork avancé https://github.com/shalzz/mates.rs (crate mate-rs)
Il utilise le parser https://docs.rs/crate/vobject/latest
pas de bonne doc

vcard_tui interface pour éditer un vcard manuellement avec interface élégante.
https://github.com/kenianbei/vcard_tui
utilise vcard_parser

https://docs.rs/vcard_parser/0.1.0/vcard_parser/
semble suffisant pour parser et modifier vcard.


édition facile:

recherche facile:


étape:
choisir le contact et la valeur à modifier: cm edit-i (alias ce).
choix: ajout d'une valeur, modification d'une valeur présente.
