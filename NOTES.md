# NOTES


How to make the user able with immediate mode to find with specific parameter.
He needs to put thoses parameters in plain text and the binary will convert them to be able to use the api.

## Immediate Mode

Instead of separating the values, parameters and Property name, we can use the syntax of vcard and inserting directly in the command line parser.

example:

cm find tel fn "The Full Name"

cm immediate-mode find -s 'TEl;' -f 'FN:The Full Name'
render the first property of the first vcard found.

UID:,TEL:0900999

multi property show:
cm immediate-mode -s 'TEL;' -s 'EMAIL;' -f 'FN: The Full Name'
UID:,TEl:90000,EMAIL:lm@baermail

empty parameters will match every parameters.

-f for filter.

### LOGIC OPERATOR:

It cloud be very powerfull if we could precise multiple Property to filter and applying some comparator like AND/OR for multiple groups.
However, it would bring a lot of boilerplate and in practive not very usefull.

A single LOGIC OPERATOR for all the group could be enough to brig some freedom and not be too cumbersome.
OR/AND/NOT/XOR

### Pretty and Forgiveable

--pretty
put the result in a user friendly way, with focus of the result and full name if multiple results exists.

--forgiveable
the filters will not be exact, the value will be compared with contains().


the binary could bring some shortcuts, or it could be bash alias.
for example:
$ ct Paul
Paul Lebfèbre
mobile: if multiple and intereting (fix, mobile... not pid).
93493094093
fixe:
O3940394093

Pauline Sate // if multiples results, seperate, max 5 results.
93940349304

It will make a search for only FN value, return the results with a limit and present only intresting results depending on the number of same property.


