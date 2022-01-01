# QMI (The Qombo Interpreter)

---

## Overview

QMI is an interpreter for a language I have created called qombo.
It is meant to bring functionality to the language in a dynamic way

## Goals of Qombo

**Qombo**, or **qm** for short, is a language made to describe discreet logical units, or *chips* with binary inputs and outputs.
It is used to describe electronic, binary based systems reliant on basic gates like and, or, etc. with a bit more functionality.
The goal of qm is to act as a sort of replacement for the traditional, diagram based programs used to do the same things in the past. Working on single bits like this in traditional programming languages requires a bit more effort and they don't act very well as models of real, buildable circuits. Qombo is here to fix that.

## Basics of qombo

To start, create a file ending in `.qm`.

Your code starts from the `main` chip:

    main{
    
    }

The main chip will evaluate whatever is inside and output it to the console (A list of 0s and 1s)

> Functionality for parsing numbers, getting real-time user io, etc. will be added soon.

To create a new chip, simply declare it outside the main function, including inputs in parentheses `(input1 input2)` and outputs in brackets `[output1 output2]`. Arguments should be separated by spaces as shown and logic should be in curly brackets.

    chip(a b)[o]{
    
    }

Evaluation goes left to right, with gate arguments on the left separated by spaces and gates on the right

    chip(a b)[o]{
        a b &
    }

The value of gates can be passed into other gates automatically and combined as arguments themselves.

    chip(a b)[o]{
        a b & ! b &
    }

Better read as:

    chip(a b)[o]{
        ((a b &) !) (b) &
    }

The basic starting gates are as follows:
- &: and
- |: or
- ^: xor
- !: not
Some other included useful functions are:
- *: clock
- /: rising edge detector
- \: falling edge detector
To use a chip you have created in another, simply put a `.` before its name.

    main{
        0 1 .chip
    }


