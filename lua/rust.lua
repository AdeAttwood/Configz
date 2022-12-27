-- Example configz module that will install and setup the rust toolchain

--- Query the local DPKG database to see if a package is installed
---
---@param package string The package you would like to query
---@return boolean
local is_installed = function(package)
  local ok, output = configz.run("dpkg-query --show --showformat='${db:Status-Status}' " .. package)
  return ok and output == "installed"
end

--- Install a list of packages with APT if they are not already installed
---
---@param package_list string[] The list of packages
local install_packages = function(package_list)
  local packages_to_install = {}
  for _, package in ipairs(package_list) do
    if not is_installed(package) then
      table.insert(packages_to_install, package)
    end
  end

  if #packages_to_install > 0 then
    configz.run "apt update"
    configz.run("apt install --no-install-recommends --yes " .. table.concat(packages_to_install, " "))
  end
end

-- Install the required packages, ca-certificates is required for
-- configz.download and curl is required for rustup.sh
install_packages { "ca-certificates", "curl" }

-- Download and and run the rustup.sh script provided by rust to install and
-- setup the rust toolchain
local ok, path = configz.get_executable "rustc"
if not ok then
  configz.download("/tmp/rustup.sh", { url = "https://sh.rustup.rs" })
  configz.run "chmod +x /tmp/rustup.sh && /tmp/rustup.sh -y"
else
  configz.info("Skipping installing rustc found in " .. path)
end
