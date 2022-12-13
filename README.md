# CppLaunchFileCreator
Create launch.json vscode files by placing this command before your regular command. Usefull when using cppdbg dap plugin with vscode/neovim/... .

## Example usuage

```
user@/code/cpproject$ clfc ./a.out first_argument "second_argument"
```

Will output to stdout:

```
{
  "configurations": [
    {
      "name": "c++ launch",
      "type": "cppdbg",
      "request": "launch",
      "args": [
        "first_argument",
        "second_argument"
      ],
      "program": "/code/cppProject/a.out",
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

Save this output in a file .vscode/launch.json, so the cppdbg plugin can pick it up.

## Why?
Sometimes you want to quickly debug a command line tool. But it might have quiet a bit of arguments. Or you forgot all the details of how to create a launch.json. This tool automates this process.

## Compile?
```
cd cflc
cargo build --release
```

## Bugs?
Make an issue, I am alway interested in making this better.
