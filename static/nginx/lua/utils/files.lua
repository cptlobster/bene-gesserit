local cjson = require "cjson"
cjson.decode_array_with_array_mt(true)

-- Common file management utilities for data storage in Bene Gesserit. Designed
-- to handle various file formats (JSON, line-by-line reading). In future
-- iterations, this will also handle locking files to protect against race
-- conditions.
local _M = {}

-- Read a plaintext file as a string.
function _M.read_file(ngx, file_path)
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file " .. file_path ..": ", err)
        return ""
    end
    
    local content = file:read("*all")
    file:close()

    return content
end

-- Read a JSON file as a table.
function _M.read_json(ngx, file_path)
    local content = _M.read_file(ngx, file_path)

    local json = { }
    if content ~= "" then
        json = cjson.decode(content)
    end

    return json
end

-- Read a file line by line into a table.
function _M.read_lines(ngx, file_path)
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file " .. file_path ..": ", err)
        return {}
    end

    local line_table = {}
    for line in file:lines() do
        table.insert(line_table, line)
    end

    file:close()
    return line_table
end

-- Write a string to a file.
function _M.write_file(ngx, file_path, content)
    local file, err = io.open(file_path, "w")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file " .. file_path ..": ", err)
        return false
    end

    file:write(content)
    file:close()
    return true
end

-- Serialize a table into JSON and then write that to a file.
function _M.write_json(ngx, file_path, table)
    local newstr = cjson.encode(table)
    _M.write_file(ngx, file_path, newstr)
end

return _M