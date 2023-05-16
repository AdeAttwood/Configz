local ok = configz.download("/tmp/configz/downloaded-file.liquid", {
  url = "https://raw.githubusercontent.com/AdeAttwood/Configz/b2b580e9e678ba7e8688090d34b8625f77d655c3/tests/template.liquid",
})

assert(ok)

local _, content = configz.run "cat /tmp/configz/downloaded-file.liquid"
assert(content == "This is a template\n")

local sha_download_ok = configz.download("/tmp/configz/downloaded-file.liquid", {
  url = "https://raw.githubusercontent.com/AdeAttwood/Configz/b2b580e9e678ba7e8688090d34b8625f77d655c3/tests/template.liquid",
  sha256 = "dfda673284e59eb4aa0865bb9b019f8b6190153377e0ebd3d1675d537128b0a0",
})

assert(sha_download_ok)
