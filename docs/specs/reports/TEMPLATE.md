---
date: {YYYY-MM-DD}
requirement: {Requirement-ID}
status: {`COMPLETE`|`PARTIALLY COMPLETE`|`NOT STARTED`}
---
# Implementation Report: {Requirement-ID} - {Requirement Name}

## Implementation Summary

For completed and partially completed requirements, 1-2 paragraphs explaining:
    - How the implementation works overall
    - Specific behaviours of note
    - Control and data flow(s)
    - Other significant details as appropriate

## Work Remaining

(`N/A` for `Complete` requirements) Itemised list of specific work required for the
requirement to be completed.

## Architecture

One or more [Mermaid](https://mermaid.js.org/intro/syntax-reference.html) diagrams, include ALL
applicable to the requirement:

- [Sequence diagrams](https://mermaid.js.org/syntax/sequenceDiagram.html) (e.g. IPC, user
    interactions)
- [State diagrams](https://mermaid.js.org/syntax/stateDiagram.html) (e.g. system state
    transitions)
- [Entity relationships](https://mermaid.js.org/syntax/entityRelationshipDiagram.html) (e.g.
  data entities)
- [Class diagrams](https://mermaid.js.org/syntax/classDiagram.html)
- [Flowcharts](https://mermaid.js.org/syntax/flowchart.html) (e.g. process/control flows)
- _Any other diagram type that best describes the information_

Each diagram should be preceded by a `### Title` and a short summary of what the diagram shows,
and any clarifying remarks (if anything is not self-evident from the diagram). Diagrams should be
embedded using a `mermaid` code fence.

## Noteworthy

(Discretionary section, `N/A` if not relevant) Discussion about any especially interesting details
about the implementation, or insights related to it.

## Related Requirements

- [REQ1-ID](../ID-NAME.md) Related Requirement 1 Name
- [REQ2-ID](../ID-NAME.md) Related Requirement 2 Name
- ...

## References

- [Reference 1](https://example.com)
- [Reference 2](https://example.com)
- ...
