local fileutils = require "utils/files"

local index_table = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_'

-- Helper functions for navigating a JSON filetable. The filetable is a
-- directory that contains a set of JSON files; each client ID is hashed into a
-- value between 0 and 255 and placed in the corresponding JSON file.
local _M = {}

-- Generate a hash between 0 and 255 of a base64-encoded JWT. This just indexes
-- each base64 character, sums them up, and then takes a modulus of 256.
function hash(id)
    local value = 0
    -- TODO: find a better way to do this. openresty uses an old enough Lua
    -- version that bitwise operations are not natively supported, I hate this
    -- so much
    id:gsub(".", function (char)
        if char ~= "." then
            local offset, _ = string.find(index_table, char)
            if offset then
                value = value + offset
            end
        end
    end)

    return string.format("%x", value % 256)
end

-- Read an entire file in the JSON filetable.
function _M.read_file(ngx, directory, id)
    local file_path = directory .. "/" .. hash(id) .. ".json"
    return fileutils.read_json(ngx, file_path)
end

-- Read a record by ID in a JSON filetable.
function _M.read_by_id(ngx, directory, id, default)
    local json = _M.read_file(ngx, directory, id)

    if json[id] ~= nil then
        return json[id]
    else
        return default or {}
    end
end

-- Overwrite the entire JSON filetable.
function _M.write_file(ngx, directory, id, table)
    local file_path = directory .. "/" .. hash(id) .. ".json"
    return fileutils.write_json(ngx, file_path, table)
end

-- Update a single record in the JSON filetable.
function _M.update_by_id(ngx, directory, id, content)
    local table = _M.read_file(ngx, directory, id)

    table[id] = content

    return _M.write_file(ngx, directory, id, table)
end

return _M