local fileutils = require "utils/files"
local jsontable = require "utils/jsontable"
local cjson = require "cjson"
local ck = require "resty.cookie"
cjson.decode_array_with_array_mt(true)

-- Functions for handling the client database. At the moment it is stored in a
-- single JSON file. This needs to be improved significantly at some point.
local _M = {}

-- Get a client's ID from their Anubis token
function _M.get_id(ngx)
    -- instantiate the cookie library
    local cookie, err = ck:new()
    if not cookie then
        ngx.log(ngx.ERR, err)
        return
    end
    
    -- get all the cookies
    local fields, err = cookie:get_all()
    if not fields then
        ngx.log(ngx.ERR, err)
        return
    end

    -- find the first anubis cookie that matches (since Anubis rotates cookies)
    for k, v in pairs(fields) do
        if k:match("^techaro%.lol%-anubis") then
            return v
        end
    end
end

-- Get a client's record
function _M.get_client(ngx)
    local ip = ngx.header["X-Real-Ip"]
    local t = {}
    -- this is the sole reason we import CJSON in this file, to use an array
    -- metatable if the client is not found in the client DB
    setmetatable(t, cjson.array_mt)
    local default = { ip = ip, violations = 0, last_violation = 0, requests = t }
    local id = _M.get_id(ngx)
    return jsontable.read_by_id(ngx, "/etc/nginx/bg_conf/clients", id, default)
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

-- Determine if a client can have a violation logged.
function _M.can_violate(client, now, grace)
    return client["last_violation"] + grace <= now
end

-- Increment violations for a client
function _M.record_increment(client, now)
    client["violations"] = client["violations"] + 1
    client["last_violation"] = now
    return client
end

-- Increment violations for a client if it is allowed by the delay policy.
function _M.increment_violations(ngx, client, grace)
    local time = ngx.time()
    if _M.can_violate(client, time, grace) then
        return _M.record_increment(client, time)
    else
        return client
    end
end

-- Update a client's record in persistent storage
function _M.update_record(ngx, client)
    local id = _M.get_id(ngx)
    return jsontable.update_by_id(ngx, "/etc/nginx/bg_conf/clients", id, client)
end

return _M
