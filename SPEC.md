# SPEC of Contact-Manager CLI

The cli part of the program is a tool to manage books and contacts that is based on interaction with the end user by prompts with menu.

The software will load the actual representation of books and contacts in memory.
The user can read the state of his books through this representation.

## This spec is Work In Progress

Not everything functionnality is present, and the spec could change.

## Modification of actual data.

## Appendice:

data: data stored outside the runtime of the program.
user: End User of the program.
dev user: Dev User using this library


## User interaction

The data if any is viewable by the user.
The user can change the content of the presented data without modiying the underlining data.
The user can apply his modifications immedialty or change others things and apply after.
The applied changed must triggers a set of reversible processes.
The user can pick which modifications to discard or apply.
The user can discard or apply all modifications in one confirmation.
The user must be made aware of the presence of unapplied modifications.
The differences of the changes must appear before confirmation of application of modifications.
The user can save the differences without applying them to quit the software and later rework.
If the user discard modifications, the representation of data must be up to date.
When the user apply the changes, 
If the confirmation is done, the user must have a way to undo easly his new recent modification.
The user should have a way to review past modifications of underlying data.
The user should have a way to restore completly/partially old underlying data.
A read-only mode must be made availaible.
A write-only mode must be made availaible.


## Dev interaction


