Rust From A Scripting Background
================================

.. note::

    60-minute "hands-on" tutorial with the goal of getting people from an
    intermediate proficiency with a scripting language to basic proficiency 
    with Rust concepts and syntax. Getting help on the interactive examples 
    will have to wait till the second hour of the meeting. 

Should you care about Rust?
---------------------------

.. note::

    5mins: Should you care about Rust?
        * Define systems programming
        * When is compiled > interpereted?
            * Improve performance of code bottlenecks
            * Targeting embedded/IoT platforms
        * When is Rust > other systems languages?
            * Safety
            * Easy concurrency
        * When is interpereted > compiled?
            * Faster development if code failure is ok
            * Care less about the platform the code runs on
        * When are other systems languages > Rust?
            * Rely on an esoteric library that you don't have time to rebuild
            * Job hunting at a company permanently invested in another language

The Rust Ecosystem
------------------

.. note::

    10mins: The Rust Ecosystem
        * Stable vs nightly
        * Cargo, crates are libraries
        * Installation options
            * System packages (usually just stable)
            * Multirust
            * rustup
            * Don't install and use play.r-l.o
        * Rustaceans
            * Code of Conduct applies; early decision to exclude people who want a
              place to troll or fight without moderation. 
            * users.r-l.o
            * IRC
                * #rust, #rust-beginners, #rust-internals on irc.mozilla.org
            * Reddit (for better or worse), reddit.com/r/rust
            * Twitter @rustlang
            * rustaceans.net is a directory of rust people
            * File issues on GitHub, ask questions on IRC or StackOverflow

Why Safety Matters
==================

.. note::

    (15mins)

Computer Anatomy
----------------

.. note::

    5mins: High-level overview of computer anatomy
        * Memory is bits. 1s and 0s. 
            * RAM is bits you can get to fast
            * disk is bits you can get to slower
        * CPUs have registers and instructions
            * Instruction set architectures are why you can't just load the same 
              Linux on your laptop and a raspberry pi
        * Systems languages compile to assembly, which is the instructions that
          your CPU executes
        * General rule of thumb: The *smaller* and *faster* a program you want to
          write, the more hands-on you have to get with what the computer is
          actually doing

Yet Another Language?!
----------------------

.. note::

    10mins: What's wrong with the current systems languages? 

    Hands-on: Maybe an interactive demo or visualization of an exploit, if we can
    find one.

        * Walk through high-level view of a use-after-free exploit
        * What's wrong is that they expect too much of programmers

How Rust Helps
==============

.. note::

    (25 mins)

    This part is basically section 4 of The Book (http://doc.rust-lang.org/stable/book/syntax-and-semantics.html) 
    but skipping as much as possible. 

Basic Syntax 
------------

.. note::

    (~5mins)

    Hands-on: Hello World in the playpen which demonstrates each concept
    correctly, then does it incorrectly. Attempt to fix the errors. The correct
    section of the script can be identical to the slide for this part. Goal is
    visual recognition of the very basics, and comfort that error messages aren't
    the end of the world.

        basic_syntax.rs
        http://rustbyexample.com/primitives/literals.html

        4.1. Variable Bindings
        4.2. Functions
        4.3. Primitive Types
        4.4. Comments
        4.32. Operators

Functions
---------

.. note::

    Needs diagram to show the boilerplate syntax of specifying types; stay out of
    type system other than that

        function_and_operator.rs

        4.15. Method Syntax
        4.24. Universal Function Call Syntax

Logic
-----

.. note::

    (~5mins)

    Hands-on: Some kind of fizz-buzz flavored thing from hackerrank with 1 good
    match statement and 1 bad one that students stare at and fix

        4.5. if
        4.6. Loops
        4.13. Match
        4.14. Patterns
        4.21. if let

Borrow Checker, your robotic mentor
-----------------------------------

.. note::

    (~10mins)

    Hands-on: Trivial playpen examples which only work after you rearrange the
    lines. Probably something from rust by example will need only slight
    modification.

        4.7. Ownership
        4.8. References and Borrowing
        4.9. Lifetimes
        4.26. `const` and `static`
        4.10. Mutability

Data structures
---------------

.. note::

    (~5mins) 

    Pick 1 or 2 and roll them in with functions and logic, to give time for a
    better hands-on example?

        4.11. Structs
        4.12. Enums
        4.16. Vectors
        4.17. Strings


What we skipped 
===============

.. note::

    (5 mins)
    * Unsafe
    * Concurrency (it's easy, though!)
    * File IO
    * Using crates
    * The entire type system: (this list is just here for reference of what the
                               book has that we're leaving out, NOT supposed to be 
                               packed into 5mins) 
        4.18. Generics
        4.19. Traits
        4.22. Trait Objects
        4.23. Closures
        4.27. Attributes
        4.28. `type` aliases
        4.29. Casting between types
        4.30. Associated Types
        4.31. Unsized Types
        4.33. Deref coercions
        4.34. Macros
        4.35. Raw Pointers

What next?
==========

.. note::

    Re-use ecosystem summary slide?
