# reqgather

The reqgather utility searches files for requirements.

## How will reqgather find requirements within a repo
I'm not sure if reqtangle should specify how requirements are represented within a project.
My current thought is for reqtangle to provide specifications/guidelines that can be specified on the commandline.
For example, let's say a project creates their own set of guidelines that were agreed upon through an RFC process named RFC1234.
That project can formalize those guidelines in a manor that reqgather understands.
Then when reqgather is executed, the project would specify `--spec RFC1234` which would ultimately resolve down to a config file that reqgather understands.

## Requirement management in open source
At the time of starting this project I am unaware of any open source projects officially managing requirements.
The closest thing I have observed are practices like Python's PEP and other RFC-related practices.
Those are great, but I don't think changes within those documents are traced throughout the actual source code.

I think there's a lot of potentially valid ways to represent requirements within a source directory.
There may be existing implementations already, such as existing standards or company/project guidelines.
I'm currently only aware of database oriented requirement managers (e.g. ClearCase).
I have no delusion that requirement management is a complicated job, but I feel like there needs to be a solution for open source projects.
If open source software is to be used at scale in the future there must be a way to manage requirements since there can potentially be hundreds or even thousands of contributors to a project that may affect millions or even billions of people.
When contributors make changes, they need confidence that their contributions won't break expected behavior.
Currently, best software practices utilize tools such as regression tests and continuous integration (CI) services, such as [Travis CI][0].
Regression tests are typically implemented via unit tests in many projects, however unit tests are limited in what they can test.
Generally unit tests validate logic of functions at their lowest level.
At the opposite end, there are system tests that validate logic of the entire system.
In between the system tests and unit tests are what are called functional tests, which validate interaction between subsets of parts of the system.

TDD has been a great practice for creating regression tests as software is being written.
BDD is similar, however it tackles the problem from a user's perspective.
Typcially, BDD is not a practice followed at the level of unit tests since users tend to use the system as a whole.
BDD works great for communicating human intent through scenarios that can be broken up into individual steps which are then converted to code that executes the actual software of which the BDD scenario was intended to communicate.
As you may be able to see, BDD scenarios are a great way to formalize requirements agreed upon among humans.
Those formalized requirements can then be broken down to the actual code that implements them.

## How to trace requirements down to the source code
How do projects know that a feature its users expect to exit actually exists?
In fact how does the project actually know that that feature is expected?
This information that should exist within a project's documentation, which can include the RFC's that spawned the features themselves.
There needs to be a way to identify specific requirements.
For example, requirement identifiers can exist within comments like `// [REQ1234]`.
But, just by having a tag within a source file doesn't do much since all it really does is say, "Hey, there's a requirement being implemented within this file".
We want to be able to say, "There's a requirement being satisfied in this file via these specific lines of code".
So, perhaps a better example could be `// [REQ1234:+6]`.
This example could indicate, "REQ1234 is satisfied by the following 6 lines of code".

[0]: https://travis-ci.org