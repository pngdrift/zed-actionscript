[
  "+"
  ":"
  "++"
  "-"
  "--"
  "&"
  "&&"
  "|"
  "||"
  "!"
  "!="
  "=="
  "*"
  "/"
  "%"
  "<"
  "<="
  ">"
  ">="
  "="
  "-="
  "+="
  "*="
  "/="
  "%="
  "^"
  "^="
  "&="
  "|="
  "~"
  ">>"
  ">>>"
  "<<"
  "::"
] @operator


(function_declaration
  name: (identifier) @function)

(interface_declaration
  name: (identifier) @type)

(class_declaration
  name: (identifier) @type)

(class_declaration
  interfaces: (identifier) @type)

(class_declaration
  superclass: (identifier) @type)

(namespace_declaration
  name: (identifier) @type)

(member_expression
  object: (identifier) @type)

(call_expression
  function: (identifier) @type)

(new_expression
  (identifier) @type)

(generic_data_type
  type_parameters: (identifier) @type)

(type_hint
  type: (identifier) @type)

((identifier) @constant
  (#match? @constant "^[A-Z_$][A-Z\\d_$]*$"))

[
  (string)
] @string

[
  (regex)
] @string.regex

[
  (number)
] @number

[
  (true)
  (false)
] @boolean

[
  (null)
  (undefined)
] @type


[
  "class"
  "default"
  "extends"
  "implements"
  "instanceof"
  "as"
  "is"
  "in"
  "interface"
  "with"
  "override"
  "final"
  "internal"
  "private"
  "protected"
  "public"
  "static"
  "get"
  "set"
  "delete"
  "function"
  "const"
  "var"
  "return"
  "if"
  "else"
  "switch"
  "case"
  "for"
  "while"
  "do"
  "continue"
  "break"
  "import"
  "package"
  "throw"
  "finally"
  "try"
  "catch"
] @keyword

"new" @operator

(ternary_expression
  [
    "?"
    ":"
  ] @operator)

[
  ";"
  "."
  "..."
  ","
] @punctuation.delimiter

[
  "{"
  "}"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "("
  ")"
] @punctuation.bracket


(labeled_statement
  (identifier) @label)

[
  (line_comment)
  (block_comment)
] @comment

((block_comment) @comment.doc
  (#match? @comment.doc "^\\/\\*\\*"))