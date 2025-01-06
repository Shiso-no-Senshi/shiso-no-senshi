# Architecture

## Class Diagram

```mermaid
classDiagram
	direction LR
	namespace shiso {
		class Node {
			<<abstract>>
			+name: string
			+url: Url
		}

		class Collection {
		}

		class Folder {
		}

		class Document {
			+content: string | byte[]
		}

		class Reference {
			+url: Url
		}

		class Configuration {
			+darkMode: boolean
			+theme: string
		}
	}

	Folder --|> Node
	Collection --|> Folder

	Document --|> Node

	Reference --|> Node

	Node "1"*--"*" Node: children

	Collection "1"*--"1" Configuration: +configuration
```

### Linking

```mermaid
graph TB
	subgraph coll[Collection]
		collection[Collection]@{shape: docs}
		a@{shape: docs}
		b@{shape: docs}
		a.1@{shape: docs}
		a.2.md@{shape: doc}
		a.3.md@{shape: doc}
		a.1.1.md@{shape: doc}
		a.1.2.md@{shape: doc}
		b.1.md@{shape: doc}
		b.2.md@{shape: doc}
	end
	subgraph remote["Remote (e.g. Internet)"]
		web
	end
	subgraph local[Local Files]
		image
	end

	collection --> a
	collection --> b

	a --> a.1
	a --> a.2.md
	a --> a.3.md

	a.1 --> a.1.1.md
	a.1 --> a.1.2.md

	b --> b.1.md
	b --> b.2.md

	a.1 -.- web[Web Page]@{shape: doc}
	a.1 -.- image[Image File]@{shape: lin-doc}
```

1. Each node has zero or more "Child" nodes.
    1. Each node has exactly one Parent.
    2. The Parent owns the Child nodes.
    3. Deleting the Parent will delete all children, recursively.
2. Each node points to exactly one "Document"
    1. Documents can be internal or external to the collection.
    2. Deleting a node will delete

## Relationships

```mermaid
graph
	subgraph collection[Collection]
		parent[Parent]
		current[Current]
		sibling[Sibling]
		child[Child]
	end

	parent --> current
	parent --> sibling
	current <-.-> sibling
	current --> child
```

1. Every node has exactly one parent.
2. A "Child" node is the node pointed to by a "Parent" node.
3. A "Sibling" node is a node that either refers to or is referred to by the current node.
