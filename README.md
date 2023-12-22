# README

## Contact-Manager

contact-manager (cm for short), is a command line contact manager written in rust using the library [vard_parser](https://crates.io/crates/vcard_parser).
It also expose his own high lever library for other crates to use.

### Overview

**WIP !**

cm is used to manage contacts manually or automaticly and their address books. It can be integrated with softwares who need access to contacts using the public API or by the immediate mode and/or generation of index.
 
Two modes are available from the command line:\

#### Immediate mode

Return result(s) after one command, made for alias command in terminal or for integration with other software who need access to a contact manager.\

#### Interactive mode: 

Interact with the user with prompts and menu to make the use more user friendly.\

#### Features

##### Immediate Mode

- [x] create, find, modify and delete contact, filter by book
- [x] create, rename, delete address book.
- [x] search for any field by any field, filter by book
- [x] edit any field by any field, filter by book
- [ ] set priority of a field
- [ ] delete a field
- [x] generate index for other sotfware (such as an email client).

##### Public API

- [ ] create, find, modify and delete contact, filter by book
- [ ] search for any field by any field, filter by book
- [ ] edit any field by any field, filter by book

##### Interactive Mode

- [x] forgiveable search with multiple finds.
- [ ] menu for managing contacts.
- [ ] presentation of a contacts.

##### TUI

- [ ] interface for managing contacts and books (low priority).


#### Examples

interactif mode, search for the phone of someone by their approximative fullname.
```bash,ignore
cm interact find-forgiveable tel fn Partialname
```

with an alias, it could be made shorter to:
```bash,ignore
ct Partialname
```
Ajout d'un contact dans un carnet
```bash,ignore
cm immediate addto clients fn FullName
```




#### Technical details

cm is using [vcard_parse](https://crates.io/crates/vcard_parser) to make all the parsing and saving of the vcard v4 format file.
It manages the contacts in adressbooks with links to never have a contact file more than once on your storage device. So you have the main folder with all your contacts and one folder per addressbook which contains a link for every contacts in this book.

cm will not let you input invalid data (will refuse for use of immediate mode or library, but guide the user in interactive mode), because it could be used for other software which need valid data. cm needs to be made friendly to those softwares.

All the saved contacts are in vcard format, which would enable you to use this programm whih a cardav syncroniser. (see [vdirsyncer-rs](https://git.sr.ht/~whynothugo/vdirsyncer-rs) for a work on that topic in rust).


#### Performance

cm aims and is fast with immediate response from a human perspective. The first reason I began to write this software was because khard is bloated and slow.

#### Security

This programm does not use any encryption. It does consider that you use it on a environnement controlled by you and not accessible by untrusted parties.

#### Privacy

This programm does not communicate whatsoever to anyone.

#### License

This programm is GPLv3.

#### Alternatives

Only looking to crates published on crates.io

None of the alternatives have a stable version.

**mates-rs**: is dedicated to contact and their emails for integration for mutt. It does not enable to manage other fields except the tel number. It does not validate the entries.

**vcard_tui**: a tui for modifing a contact, the concept could be used for this programm.
