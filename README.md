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

# What Problem Does Reqtangle Try To Solve
There are millions of lines of code that exist in millions of different open source projects.
Not every line of code needs to be traced to a requirement, and trying to do so would be way too much work.
Reqtangle simply tries to provide a means to make tracing requirements down to the actual code that implements a requirement possible.

Let's not fool ourselves, managing requirements is not something people who want to write code want to do.
But for some projects requirements _should_ be something that is documented and managed to some level.
On a project with millions of lines of code, when a new contributor wants to help by fixing bugs or adding new features, how can they have confidence that their changes are not going to break something?
For the most part, this is resolved by regression tests.
And that is probably fine for a majority of open source projects.
But what happens when a project is so big or has been around for a long time, there may be several design changes and code moved around where a single person, and even a small group, can keep up with so many contributions.
One solution is to provide a means for individuals to know whether or not they may have made changes that break agreed upon requirements among the community.
When this occurs it can spawn a discussion within the community that can result in officially changing one or more requirements.

But we need to be careful not to turn an open source software project into an open source requirement management project.
This is where rules/guidelines need to be decided upon for individual projects.
For example, most projects probably won't want to invasively track most requirements, especially if specific code is updated frequently.

One particular scenario I envision Reqtangle being used in large open source projects is for tracking deprecated code.
This is one scenario where invasive requirements may be of greater use, because maintainers can use reqtangle to track when to remove code that is no longer supported.
This makes it trivial to find the code that should no longer exist in the code base and simplify the code for maintainers in the future.
