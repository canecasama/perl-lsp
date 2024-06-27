local M = {}

local find_rust_bin = function()
	return "/home/msoares/git_tree/perl-lsp/target/debug/perl-nvim-lsp"
end

M.start = function()
	vim.print("LSP START")
	vim.lsp.start({
		name = "perl-nvim-lsp",
		cmd = { find_rust_bin() },
		root_dir = vim.fs.dirname(vim.fs.find({ "composer.json" }, { upward = true })[1]),
	})
end

local group = vim.api.nvim_create_augroup("perl-nvim-lsp", {})
M.setup = function()
	vim.print("LSP SETUP")
	vim.api.nvim_clear_autocmds({ group = group })
	vim.api.nvim_create_autocmd("FileType", {
		group = group,
		pattern = { "perl" },
		callback = M.start,
	})
end

return M
