# reqtangle
A utility for managing requirements in open source projects.

The simple overview of this project is to allow requirements to be spread throughout a project and somehow track whether those requirements are satisfied.
Requirement management is hard. Period.
One of the main reasons they are hard is because they are the boundary between humans and programs.
Programs are typically pretty good at doing what they're told.
Humans _can_ be pretty good at doing what they're told, but there are noticeable differences.
One of the most difficult things to do when gathering requirements is actually getting humans to communicate their ideas and expectations between one another.
Not to get too philisophical, but wars have been started of this type of miscommunication.

One of the main goals of this project is to simplify requirement management down and to make it something a community can do together in a decentralized manor.
Given the current state of software development in an open source economy, requirements are not an "official" part of the source code.
They should be.
Most open source projects develop organically, where a particular feature exists due to a single person taking their free time to write code and submit it to be merged into the project.
Many times features are discussed for days, weeks, or months before it comes to anything tangible in the project.
These discussions can happen in several mediums, but the end result are actually requirements.
However these requirements are typically not "officialized" by getting included with the source code repository.
They should be.
However, simply including requirements inside the repo doen't really do anything useful for a community driven project if those requirements aren't checked against changes developers submit.
That's where tooling and process come to the aid.
Most community driven projects nowadays utilize some form of Continuous Integration/Development (CI/CD) which are now possible thanks to many open-source friendly companies.
Projects that implement CI/CD practices that automatically run regression tests or build and release pipelines should be able to use those same practices to enforce requirements.

Typically requirements are stored in large databases.
These databases are controlled

The intent of this project is not to constrain how a project is managed, but it does need to know how to do its job.
If requirements aren't stored in the repo itself, it will need a way to retrieve said requirements and to determine if said requirements are satisfied or not.
For simplicity's sake an example will be provided to show how Reqtangle can be used.

Not all requirements can be automatically validated, but some can.
Reqtangle shall support specifying a requirement that can be automatically validated and the means by which to perform that validation.
For example, if a requirement can be validated by running a specific regression test, or set of regression tests, when that regression test passes in the CI process the requirement can be marked as valid.
Requirements that cannot be validated automatically shall be marked such that it can be manually validated.

- reqgather - Parses source files for requirements.
- reqcheck  - Checks the tracing between requirements and any potential errors. This program could analyze any suspect requirement breakage in terms of green-yellow-orange-red for likelhood of actual breakage.
