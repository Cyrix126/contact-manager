# README


 **WIP !**

## Contact-Manager

contact-manager (cm for short), is a [fast](../benchs), minimalist, powerfull and correct command line interface contact manager written in rust.

It also expose his own high-level library for other crates to use.

### Overview

cm is used to manage contacts manually or automaticly with address books. It can be integrated with softwares who need access to contacts, using the library or with the immediate mode binary.

cm is still in devellopement, but is actually useable. You can check the [ROADMAP](ROADMAP.md) to see the direction I intend to take for this sotfware and the [TODO](TODO.md) for more shortly features.
 
### Features

- [x] fast, power and correct contact-manager

An inconveniance could be that if you try to use this programm with already existing vcards contact not following the spec, the programm could crash.

If that's the case, the programm will tell you exacltly which vcard is wrong and why.

Use the **import** function to be sure everything is valid.

#### Immediate Mode

- [x] prevent bad input from user.
- [x] optional pretty output
- [x] no input after execution, so very easy to alias or integrate to scripts.
- [x] use a default book in case no books are specified.
- [x] args to use easly the public functions of the library.
- [ ] shell autocompletion generation

#### Public API

- [x] import from file/directory
- [x] export to file
- [x] create/delete contact
- [x] create/delete/rename address book.
- [x] create/search/delete any property to vcard with any property with any logical operator
- [x] generate index for other sotfware (such as an email client).
- [x] filter by book
- [x] forgiveable search
- [x] contacts in books as links to save space and trouble.

#### Interactive Mode

- [x] menu for managing contacts and books.
- [x] presentation of a contacts.

#### TUI

- [ ] interface for managing contacts and books (low priority).

### Usage


You can use this software in multiple ways:

- Alias to make quick commands.
- Interactive interface, with menus.
- Immediate Mode, to get all the powerfull options for one time use (if often used, consider making an alias making your life easier).
- Integrate in sotfware using the included library.
- Integrate in scripts calling the Immediate Mode binary directly.
- Generate an index for uses in other sotwares. 


```cm --help``` 

To get all available commands for immediate and interactive mode.


#### Integration for script

If using the binary in a script, do not use --pretty, as it can have unstable ouput depending on the number of results.\
Instead, assume the first line for find-value result is always the uid, followed by the full string of a property. If multiples contacts have been found to have properties matched, a empty line seperate thoses.

### Technical details

cm is using [vcard_parse](https://crates.io/crates/vcard_parser) to make all the parsing and saving of the vcard v4 format file.
It manages the contacts in adressbooks with links to never have a contact file more than once on your storage device. So you have the main folder with all your contacts and one folder per addressbook which contains a link for every contacts in this book.

cm will not let you input invalid data (will refuse for use of immediate mode or library, but guide the user in interactive mode).

All the saved contacts are in vcard format, which would enable you to use this programm whih a cardav syncroniser. (see [vdirsyncer-rs](https://git.sr.ht/~whynothugo/vdirsyncer-rs) for a work on that topic in rust).



### RFC

[RFC6350 vCard Format Specification](https://datatracker.ietf.org/doc/html/rfc6350) thanks for [vcard_parser](https://github.com/kenianbei/vcard_parser).

[XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/) Thanks to [xdg](https://docs.rs/xdg/latest/xdg/)

### Performance

cm aims and is fast with immediate response from a human perspective. The first reason I began to write this software was because khard was bloated, slow and buggy.

### Security

This programm does not use any encryption. It does consider that you use it on a environnement controlled by you and not accessible by untrusted parties.

### Privacy

This programm does not communicate whatsoever to anyone.

### License

This programm is GPLv3.

### Alternatives

Only looking to crates published on crates.io

None of the alternatives have a stable version.

**mates-rs**: is dedicated to contact and their emails for integration for mutt. It does not enable to manage other fields except the tel number. It does not validate the entries.

**vcard_tui**: a tui for modifing a contact, the concept could be used for this programm.
