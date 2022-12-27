---@meta
--
-- Ignore (W212) unused argument, and (W122) setting read-only field warnings
-- this is a meta file and stubbing all the internal configz functions
--
-- luacheck:ignore 122
-- luacheck:ignore 212

-------------------------------------------------------------------------------
--                                LOGGING
-------------------------------------------------------------------------------

--- Log and debug message to the console
---
---@param message string
---@return boolean
configz.debug = function(message) end

--- Log and info message to the console
---
---@param message string
---@return boolean
configz.info = function(message) end

--- Log and error message to the console
---
---@param message string
---@return boolean
configz.error = function(message) end

-------------------------------------------------------------------------------
--                                RESOURCES
-------------------------------------------------------------------------------

---@class FileConfig
---@field source string The source path of the file

--- Copy a file from a source to a destination. Will return true of false if
--- the operation was successful
---
---@param destination string
---@param config FileConfig
---@return boolean
configz.file = function(destination, config) end

--- Creates a symlink from a source to a destination
---
---@param destination string
---@param config FileConfig
---@return boolean
configz.link = function(destination, config) end

--- Creates the full directory path provided. Will return true of false if the
--- operation was successful.
---
---@param destination string
---@return boolean
configz.directory = function(destination) end

--- Run a command in the "sh" shell
---
---@param command string The shell command to run
---@return boolean, string
configz.run = function(command) end

---@class DownloadConfig
---@field url string The url to the file that will be downloaded

--- Download a file from a URL
---
---@param destination string The file the content of the URL will be put into
---@param config DownloadConfig
---@return boolean, string
configz.download = function(destination, config) end

-------------------------------------------------------------------------------
--                                HELPERS
-------------------------------------------------------------------------------

--- Finds the path to a programme
---
---@param programme string The name of the programme you want to find the path for
---@return boolean, string
configz.get_executable = function(programme) end

--- Tests to see if the given path is a directory
---
---@param path string The path to test
---@return boolean
configz.is_directory = function(path) end

--- Tests to see if the given path is a file
---
---@param path string The path to test
---@return boolean
configz.is_file = function(path) end
