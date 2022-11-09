# Étapes:

1/ créer un contact  (new, del)

/ suprimer un contact

2/ éditer une fiche de contact

3/ afficher la liste des contact

4/ trouver les contact possédant une valeur de champ défini.
cm find FN (of) TEL XXXXXXXXX
cherche le TEL dans tous les contacts, affiche FN des contacts trouvé.
implémentation:
utilisation de la librairie vcard_parser: OK


4.1/ faire appel à certains champs -X



5/ modifier un champ par commande

6/ afficher les informations d'un contact élégamment.

# Implémentations

- utilisation de la norme [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
XDG_DATA_HOME/contact-manager/contacts/name_of_contact.vcf

- tous les contacts sont dans un seul dossier. Le carnets d'adresse possède un lien symbolique vers le contact.
