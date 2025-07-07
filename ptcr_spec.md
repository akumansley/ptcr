**PTCR-mini — Plain-Text Code-Review Format (block-only variant)**
*Version 0.4.1  ·  1 July 2025*

---

## 1 Purpose & scope

PTCR-mini is a human-friendly, diff-friendly text format for exchanging code-review
comments.  Every record consists of

1. a **header line** that pinpoints a span of source code, and
2. a **comment body** (one or more lines of free-form text).

No other syntax (severity levels, inline suggestions, etc.) is defined in this
revision.

---

## 2 File structure

[top-level header block]    ; optional metadata & overview, any text
                            ; ends at the first record header
<record 1>
<record 2>
…


A **record** is:

<header line>
<comment body lines …>


A new record starts whenever the parser encounters a line that matches the
*header-line grammar* in § 3.  The file ends after the last record; no
terminator is required.

---

## 3 Header-line grammar

<path> ":" <span>


* **<path>** – UTF-8 file path (no spaces or additional colons).
* **<span>** – one of the forms in Table 1.
* The line **must end** immediately after <span>; **no trailing colon, summary
  text, or whitespace is allowed.**

### Table 1 Valid <span> forms

| Kind                     | Syntax        | Meaning                                     |
| ------------------------ | ------------- | ------------------------------------------- |
| Whole file               | *           | Applies to the entire file.                 |
| Single line              | L           | Line *L*.                                   |
| Single point             | L.C         | Line *L*, column *C*.                       |
| Line range               | L₁-L₂       | All of lines *L₁* through *L₂* (inclusive). |
| Column range (one line)  | L.C₁-L.C₂   | Column span on one line.                    |
| Multi-line, multi-column | L₁.C₁-L₂.C₂ | Span from (*L₁*, *C₁*) to (*L₂*, *C₂*).     |

All line and column numbers are 1-based.
Absent end coordinates default to the corresponding start coordinate.

---

## 4 Comment body

* Consists of all lines **after** the header line **up to** (but not including)
  the next header line or end-of-file.
* May contain blank lines and any UTF-8 text.
* If a body line would otherwise parse as a header (rare), prefix it with a
  single space.

---

## 5 Parsing guide (informative)

A parser MAY use the following regular expression (anchor ^…\$) to identify
header lines:

^([^\s:]+):                              # (1) path
(                                        # ----- span -----
    \*                                   #   whole-file
  |                                       #   – or –
    (\d+)                                 # (2) start line
    (?:\.(\d+))?                          # (3) start col   (opt)
    (?:-                                  #     range dash
        (\d+)                             # (4) end line    (opt)
        (?:\.(\d+))?                      # (5) end col     (opt)
    )?
)$


*If group 2 is absent, group 1 captured “*”.
Otherwise groups 2–5 contain numeric coordinates.\*

Pseudocode outline:

records = []
current = None
for each input line:
    if matches HEADER_REGEX:
        if current: records.append(current)
        current = {path, span, body: []}
    else:
        if current is None:
            file_header.append(line)         # still in top-level header
        else:
            current.body.append(line)
if current: records.append(current)


---

## 6 Example

text
ReviewedBy: AndrewKumansley
Date: 2025-07-01
Title: Initial API refactor review

src/lexer.rs:*             ← whole-file remark
Overall lexer structure is clearer after the refactor.

src/lexer.rs:15.7-15.25
`token` is reassigned here, hiding the outer variable.

src/lexer.rs:42
Remove stray `println!` used for debugging.

parser.y:100.4-120.1
Duplicates `expression_list`.  Suggest factoring out common parts.

lib/db.c:78.12-78.14
Uninitialised pointer dereference; see issue #231.


---
