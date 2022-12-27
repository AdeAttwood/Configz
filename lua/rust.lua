local is_installed = function(package)
  local ok, output = configz.run("dpkg-query --show --showformat='${db:Status-Status}' " .. package)
  return ok and output == "installed"
end

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

install_packages { "ca-certificates", "curl" }

local ok, path = configz.get_executable "rustc"
if not ok then
  configz.download("/tmp/rustup.sh", { url = "https://sh.rustup.rs" })
  configz.run "chmod +x /tmp/rustup.sh && /tmp/rustup.sh -y"
else
  configz.info("Skipping installing rustc found in " .. path)
end
