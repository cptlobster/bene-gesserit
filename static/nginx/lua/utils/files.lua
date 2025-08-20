local cjson = require "cjson"
cjson.decode_array_with_array_mt(true)
local resty_lock = require "resty.lock"

-- Common file management utilities for data storage in Bene Gesserit. Designed
-- to handle various file formats (JSON, line-by-line reading). This will handle
-- locking files while they are being read/written to avoid race conditions
-- between OpenResty workers.
local _M = {}

function _M.lock_file(ngx, file_path)
    -- create a lock for the file while reading it
    local lock, err = resty_lock:new("locks")
    if not lock then
        ngx.say("failed to create lock: ", err)
        return {}
    end

    -- lock the file before we start reading it
    local elapsed, err = lock:lock(file_path)
    if not elapsed then
        ngx.say(ngx.ERR, "failed to create lock: ", err)
        return {}
    end

    return lock
end

function _M.unlock_file(ngx, lock)
    local ok, err = lock:unlock()
    if not ok then
        ngx.say(ngx.ERR, "failed to release lock: ", err)
    end
end

-- Read a plaintext file as a string.
function _M.read_file(ngx, file_path)
    -- create a lock for the file while reading it
    local lock = _M.lock_file(ngx, file_path)
    if not lock then
        return ""
    end

    -- open the file
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file: ", err)
        -- if we fail to open the file, release the lock
        _M.unlock_file(ngx, lock)
        return ""
    end
    
    -- read contents
    local content = file:read("*all")

    -- close the file and unlock it after we finish reading it
    file:close()
    _M.unlock_file(ngx, lock)

    -- return the content
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
    -- create a lock for the file while reading it
    local lock = _M.lock_file(ngx, file_path)
    if not lock then
        return {}
    end

    -- open the file
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file: ", err)
        -- if we fail to open the file, release the lock
        _M.unlock_file(ngx, lock)
        return {}
    end

    -- read the lines into a table
    local line_table = {}
    for line in file:lines() do
        table.insert(line_table, line)
    end

    -- close the file and unlock it after we finish reading it
    file:close()
    _M.unlock_file(ngx, lock)

    -- return the table
    return line_table
end

-- Write a string to a file.
function _M.write_file(ngx, file_path, content)
    -- create a lock for the file while reading it
    local lock = _M.lock_file(ngx, file_path)
    if not lock then
        return ""
    end
    
    -- open the file
    local file, err = io.open(file_path, "w")
    if not file then
        ngx.log(ngx.ERR, "Failed to open file: ", err)
        -- if we fail to open the file, release the lock
        _M.unlock_file(ngx, lock)
        return false
    end

    -- write the string to the file
    file:write(content)
    
    -- close the file and unlock it after we finish reading it
    file:close()
    _M.unlock_file(ngx, lock)

    return true
end

-- Serialize a table into JSON and then write that to a file.
function _M.write_json(ngx, file_path, table)
    -- encode as JSON and then call write_file
    local newstr = cjson.encode(table)
    return _M.write_file(ngx, file_path, newstr)
end

return _M