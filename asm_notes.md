# ARM Assembly Notes

## Style Guide

* Capialization doesn't matter for asm instrucions (it does for labels!), so instructions are given here using lowercase.
* The configurable parts of an instruction are given in angle brakcets (`< >`), and then described as necessary below the instruction outline.
  You should substitute those parts with appropriate selections in your own code, and not write the angle brackets.
* Any other punctuation such as curly braces (`{ }`), square brackets (`[ ]`), commas (`,`), and hash signs (`#`) are all required parts of the syntax.
  Remember that rust uses curly braces for inline asm substitutions.
  For the instrucitons that use curly braces you need to either type the curly brace twice when you want a brace that's not part of a substitution,
  or you need to use the `raw` option on the inline assembly block (but that prevents *all* substitutions for that inline asm block).

## Instruction Timing

When timing information is given for an instruction, the timing are listed in terms of `S`, `N`, and `I`.

* `S` is a sequential access.
  This is when a memory unit needs to access the word/halfword/byte directly after the previous one.
  A sequential access is *sometimes* faster than a non-sequential access.
* `N` is a non-sequential access.
  This is when a memory unit is asked to access any address other than the one exactly following the previous one.
* `I` is an internal cycle.
  This is when the CPU is doing a longer computation that doesn't involve any memory access during a CPU cycle.

The actual time taken for Sequantial and Non-seuquential depends on the memory involved.
Some memory operates slower than the CPU itself, and so it can take multiple CPU cycles to complete the access (load or store).

Internal cycles don't interact with memory, and so an internal cycle is always 1 CPU cycle.
During internal cycles, the CPU's pre-fetch system can sometimes begin accessing the next instruction from ROM,
which can reduce the time required for the following instruction.
The pre-fetch system only works in limited sitautions, so I wouldn't rely on it.

The minimum time taken by any instruction will always be 1S,
because while each instruction is performed the next instruction also needs to be fetched from memory.

The GBA has a 3 stage instruction pipeline: read/decode/perform.
Taking a branch, or modifying the `pc` in any other way, has to flush the pipeline and start it again at the new address.
This means that it will generally cost at least 1N+2S.

## A32

### Conditional Operations

### Status Flags

### Bit Logic

### Add/Subtract

### Multiply

### Program Status

### Branching

### Single Memory Transfer

### Block Memory Transfer

## T32

### Conditional Operations

### Status Flags

### Bit Logic

### Add/Subtract

### Multiply

### Program Status

### Branching

### Single Memory Transfer

### Block Memory Transfer

## Pseudo Instructions
