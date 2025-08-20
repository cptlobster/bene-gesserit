local fileutils = require "utils/files"
local cjson = require "cjson"
cjson.decode_array_with_array_mt(true)

-- Functions for handling the client database. At the moment it is stored in a
-- single JSON file. This needs to be improved significantly at some point.
local _M = {}

-- Get a client's ID from their Anubis token
function _M.get_id(ngx)
    local anubis_cookie = ngx.var["cookie_techaro.lol-anubis-auth"]
    return anubis_cookie
end

-- Get a client's record
function _M.get_client(ngx)
    local id = _M.get_id(ngx)
    local json = fileutils.read_json(ngx, "/etc/nginx/bg_conf/clients.json")

    if json[id] ~= nil then
        return json[id]
    else
        local t = {}
        -- this is the sole reason we import CJSON in this file, to use an array
        -- metatable if the client is not found in the client DB
        setmetatable(t, cjson.array_mt)
        return { violations = 0, requests = t }
    end
end

-- Get the current request information
function _M.get_current_request(ngx)
    return { endpoint = ngx.var.uri, timestamp = ngx.time() }
end

-- Log a request
function _M.log(ngx, client)
    table.insert(client["requests"], _M.get_current_request(ngx))
    return client
end

-- Increment violations for a client
function _M.increment_violations(client)
    client["violations"] = client["violations"] + 1
    return client
end

-- Update a client's record in persistent storage
function _M.update_record(ngx, client)
    local id = _M.get_id(ngx)
    local json = fileutils.read_json(ngx, "/etc/nginx/bg_conf/clients.json")
    
    json[id] = client

    fileutils.write_json(ngx, "/etc/nginx/bg_conf/clients.json", json)
end

return _M