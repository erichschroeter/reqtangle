# req check
The `check` command validates the requirements to the source code.
This requires tracing information to exist before execution, which can be done via the `gather` command.

## Implementation Notes
Some specific scenarios and how I think the program should react.

### Detecting when requirement is invalidated due to code change
When a source file is modified the command needs to compare all code for the requirements it knows about (via the JSON database) within that file.
Since the SHA256 is stored in the JSON database, `reqtangle` can recompute the hash when it finds the respective code in the file.

#### Boundary case: What happens when there's duplicate code for a requirement
In this scenario the code may have been moved from the position stored in the JSON database.
If `reqtangle` just searches for the text in the file when there is duplicate code elsewhere, unrelated to the code associated with the requirement, then `reqtangle` cannot assume the requirement is still met.

### Tracking requirement when file is modified but associated code is not
When a source file is modified, it potentially invalidates metadata for all requirements within that file.
For example, the line number for specific code is tracked inside the JSON database.
If code is tracked at being at line 100 in _src/main.c_, and then we add or remove lines somewhere above that line number then the tracked code is no longer at line 100.
In cases like this, where the code mapped to the requirement does not change, the program should not consider the requirement changed at all.
However, the JSON database needs to be updated to reflect the new line number where that code now resides.