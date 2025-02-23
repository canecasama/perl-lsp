local M = {}

local find_rust_bin = function()
	return "/home/msoares/git_tree/perl-lsp/target/debug/perl-nvim-lsp"
end

M.start = function()
	vim.lsp.start({
		name = "perl-nvim-lsp",
		filetypes = { "perl" },
		cmd = { find_rust_bin() },
		root_dir = vim.fs.dirname(vim.fs.find({ "composer.json" }, { upward = true })[1]),
	})
end

local group = vim.api.nvim_create_augroup("perl-nvim-lsp", {})
M.setup = function()
	vim.api.nvim_clear_autocmds({ group = group })
	vim.api.nvim_create_autocmd("FileType", {
		group = group,
		pattern = { "perl" },
		callback = M.start,
	})

	vim.api.nvim_clear_autocmd("LspAttach", {
		group = group,
		callback = function(args)
			local client = vim.lsp.get_client_by_id(args.data.client_id)
			if not client or client.name ~= "perl-nvim-lsp" then
				return
			end

			require("perl.lsp").on_attach(client)
		end,
	})
end

return M
