-- Load debug adapters in this project
local nvimdap = require("nvim-dap-projects")
nvimdap.search_project_config()

-- Set make program to 'just'
-- You can inside a `.rs` file just run
-- `:make` which will execute the build
-- for that component.
vim.o.makeprg = "just build-component %<"
