# Étapes:

1/ créer un contact  (new, del)

/ suprimer un contact

2/ éditer une fiche de contact OK
éditer de manière plus simple sans avoir à préciser le type de field dans la valeur

3/ afficher la liste des contact

4/ trouver les contact possédant une valeur de champ défini. OK


4.1/ faire appel à certains champs -X OK



5/ ajouter/modifier un champ par commande

6/ afficher les informations d'un contact élégamment.

7/ gérer plusieurs carnets d'adresse

# Implémentations normes

- utilisation de la norme [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html) OK

- tous les contacts sont dans un seul dossier. Le carnets d'adresse possède un lien symbolique vers le contact.

# Intégration:

- client mail neomutt: 
trouver adresse mail par nom de contact ou adresse mail à compléter 
ajout contact par adresse mail

- client sip
trouver numéro par nom de contact à compléter.
