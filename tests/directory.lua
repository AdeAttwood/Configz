local ok = configz.directory "/tmp/configz/some/nested/directory"
assert(ok, [[directory("/tmp/configz/some/nested/directory") did not return successfully]])

local is_dir = configz.is_directory "/tmp/configz/some/nested/directory"
assert(is_dir, [["/tmp/configz/some/nested/directory" should be a directory]])
