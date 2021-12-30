# My solutions to leetcode problems to get some practice in Rust

## Setup in VSCode

I use `leetcode.vscode-leetcode` and `matklad.rust-analyzer` extensions for quick turnaround.

Here is my settings of the `leetcode` extension:

```json
{
  "leetcode.defaultLanguage": "rust",
  "leetcode.editor.shortcuts": ["submit", "test", "solution", "description"],
  "leetcode.endpoint": "leetcode",
  "leetcode.filePath": {
    "rust": {
      "folder": "rust/src",
      "filename": "s${id}_${snake_case_name}.${ext}"
    }
  },
  "leetcode.hint.commandShortcut": false,
  "leetcode.hint.commentDescription": false,
  "leetcode.hint.configWebviewMarkdown": false,
  "leetcode.showDescription": "In File Comment",
  "leetcode.showLocked": true,
  "leetcode.useEndpointTranslation": false
}
```
