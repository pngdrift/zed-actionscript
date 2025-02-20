# Zed ActionScript
ActionScript language support for [Zed](https://github.com/zed-industries/zed).

## Configuration
In the `settings.json` file, you need to add the path to the SDK:
```jsonc
{
  "lsp": {
    "actionscript": {
      "initialization_options": {
        "sdk_path": "/path/to/sdk"
      }
    }
  }
}
```

## Tree-Sitter
- https://github.com/Rileran/tree-sitter-actionscript

## Language Server
- https://github.com/BowlerHatLLC/vscode-as3mxml/tree/main/language-server