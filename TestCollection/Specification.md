# Advanced Markdown with Layout

## Introduction

### What is AML?

Advanced Markdown with Layout (AML) is a plain text format for writing structured documents based on Markdown[^mark], CommonMark[^com], Mermaid[^mer], Frontmatter[^front], and other similar tooling. AML is mostly backwards compatible with common Markdown.

[^mark]: "The Markdown Guide" <https://www.markdownguide.org>

[^com]: "CommonMNark Spec" <https://spec.commonmark.org>

[^mer]: "Mermaid.js" <https://mermaid.js.org>

[^front]: "Front Matter" <https://frontmatter.codes>

### Why Yet Another Specification?

The design goal for AML is to be a format for Shisō no Senshi that is easy to write, easy to read, renderable in multiple forms, non-proprietary, and extendable.

- Everything can be written using nothing more than a plain text editor.
- Documents are readable without any processing, as raw text.
- Spaces are used for spacing. Tabs are used for indentation.
- Rules and precedences are explicit.
- Some formatting options have been removed, leaving fewer ways to do things.

### About This Document

This document is written in AML. It defines the specification completely. It includes examples of every option, and is a test document for AML.

This specification also details how AML should be rendered in other formats. It provides sufficient information that - theming aside - any rendering should look as close as possible to any other.

## Preliminaries

### Characters and Lines

- All Unicode characters are supported
- Lines end with "Line Feed" (U+0A) only
- "Carriage Return" is ignored (U+0D).
- Tab is treated as tab.
- Whitespace (U+Zs) is never used to delineate anything.

#### Tabs and Spaces

- Tabs are not expanded to spaces. They remain tabs. 

#### Escape Sequences

- "Reverse Solidus" ("Backslash") is used to escape special characters. 
- A doubled "Reverse Solidus" is the escape sequence for the "Reverse Solidus"
- An escaped backslash does not escape other characters.
- Backslash escaped don't work in code blocks, inline code, URLs, or raw HTML.

#### HTML Entity Codes

- HTML entity codes can be used to represent any valid literal. HTML entities do not act as AML symbols.

#### Insecure Characters

- U+0000 must be escaped. 

### Uniform Resource Locators (URLs)

- All URLs must be encoded.

## structured

### Blocks

A document is composed of a sequence of blocks - paragraphs, block quotations, lists, list items, headings, etc. 

There are two kinds of block: Contaiers and Leaves. Containers, as the name implies, can contain other blocks; Leaves cannot.

#### Precedence

Containers always take precedence over Leaves. For example, in the following the *list* block will be rendered first so it will not render a code span:

```aml Precedence Example 1
- `one
- two`
```

This will render in, for example, HTML as:

```html
<ul>
	<li>`one</li>
	<li>two`</two>
</ul>
```
### Leaf Blocks

- Horizontal rule (Thematic break): `---`
  - If indented the line will start at the indentation level.
  - The line will continue to the opposite margin.
  - No spaces are permitted.
  - No other characters may be on the same line.
  - It at least three dashes. More are permitted.
- Headings
  - One to six Number Signs followed by a space followed by the text of the heading.
  - Includes all characters after the initial space to EOL.
  - Everything after the first space is parsed as a Leaf.
  - **Deprecated**: Setext Headings are not supported.
- Fenced Code Blocks
  - Begin and end with three Grave Accent or three Tilde characters. Two alternatives are provided to allow the other fence to be included as code.
	
	~~~markdown
	```
	code here
	```
	~~~

	```markdown
	~~~
	code here
	~~~
	```

  - The closing fence must match the opening fence.
  - May optionally have a language specifier immediately after the opening Grave Accents.
  - May optionally have a space followed by a title after the opening Grave Accent marks/language specifier.
  - **Deprecated**: Indented code blocks are not supported.
- HTML Blocks
  - Everything from the start to the matching end HTML tag is an HTML block.
  - Markdown formatting is not supported inside HTML blocks.
- Link Reference Definition (e.g. Footnote)
  - `[name]: url "Title"`
  - The title starts and ends with the quotation mark.
  - Line breaks between the quotation marks are interpreted literally?
- Paragraphs
  - Sequences of non-blank lines that are not other blocks are paragraphs.
  - Paragraph contents are treated as inline Leaves.
- Blank Lines
  - Ignored except as separators between blocks.

### Container Blocks

- Block Quotes
  - `>` followed by a space and text.
  - Can be nested. Spaces between `>` are optional.
- Callouts
  - Block Quote with `[!type] Title` on the first line.
- Lists
  - List items
    - `- ` for unordered
    - '1. ' for ordered
    - Indent to nest
    - Blank lines between items are ignored.
