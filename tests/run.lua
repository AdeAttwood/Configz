local ls_ok, ls_output = configz.run "ls"
assert(ls_ok, [['ls' did not exit successfully]])
assert(ls_output:find "Cargo.toml", [['Cargo.toml' should be found in the output]])

local invalid_ok, invalid_output = configz.run "not-a-command"
assert(not invalid_ok, [['not-a-command' should not exit ok]])
assert(invalid_output:find ": not found")
