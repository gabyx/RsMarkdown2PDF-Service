local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "lldb-vscode", -- adjust as needed
    name = "lldb",
}

dap.configurations.rust = {
    {
        name = "converter",
        type = "lldb",
        request = "launch",
        program = "${workspaceFolder}/markdown-to-pdf/target/debug/markdown-to-pdf",
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
    {
        name = "api",
        type = "lldb",
        request = "launch",
        program = "${workspaceFolder}/api/target/debug/api",
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
    {
        name = "web",
        type = "lldb",
        request = "launch",
        program = "${workspaceFolder}/web/target/debug/web",
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
}
