local ok = configz.file("/tmp/configz/Cargo.toml", { source = "Cargo.toml" })
if not ok then
  error [[file("/tmp/configz/Cargo.toml") should have returned true]]
end

if not configz.is_file "/tmp/configz/Cargo.toml" then
  error [["/tmp/configz/Cargo.toml" is not a file]]
end
