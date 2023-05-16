local function test_template(dest, config)
  local ok = configz.template(dest, config)
  if not ok then
    error(string.format([[file("%s") should have returned true]], dest))
  end

  if not configz.is_file "/tmp/configz/template.txt" then
    error(string.format([["%s" is not a file]], dest))
  end
end

local function read_file(file_path)
  local file = io.open(file_path, "rb")
  if file == nil then
    return false, ""
  end

  local content = file:read "*all"
  file:close()

  return true, content
end

test_template("/tmp/configz/template.txt", { source = "tests/template.liquid" })
test_template("/tmp/configz/template.data.txt", {
  source = "tests/template.data.liquid",
  data = {
    name = "World",
    bool_one = false,
    bool_two = true,
    user = { name = "The user name" },
    list = { "one", "two", "three" },
  },
})

local file_ok, content = read_file "/tmp/configz/template.data.txt"

assert(file_ok)
assert(content:find "Hello World!")
assert(not content:find "Bool one is true")
assert(content:find "UserName: The user name")
assert(content:find "UserNameLower: the user name")
assert(content:find "List: one, two, three")

test_template("/tmp/configz/template.include.txt", { source = "tests/template.include.liquid" })
local include_ok, include_content = read_file "/tmp/configz/template.include.txt"
assert(include_ok)
assert(include_content:find "INCLUDE: This is a template")
