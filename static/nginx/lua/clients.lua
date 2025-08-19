local cjson = require "cjson"
cjson.decode_array_with_array_mt(true)

local _M = {}

-- Get a client's ID from their Anubis token
function _M.get_id(ngx)
    local anubis_cookie = ngx.var["cookie_techaro.lol-anubis-auth"]
    return anubis_cookie
end

-- Get a client's record
function _M.get_client(ngx)
    local id = _M.get_id(ngx)
    local file_path = "/etc/nginx/bg_conf/clients.json"

    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open clients file: ", err)
        return { violations = 0, requests = { } }
    end

    local content = file:read("*all")
    file:close()

    local json = { }
    if content ~= "" then
        json = cjson.decode(content)
    end

    if json[id] ~= nil then
        return json[id]
    else
        local t = {}
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
    local file_path = "/etc/nginx/bg_conf/clients.json"

    local content = ""
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open clients file: ", err)
    else
        content = file:read("*all")
        file:close()
    end

    local json = {}
    if content ~= "" then
        json = cjson.decode(content)
    end
    
    json[id] = client

    local file, err = io.open(file_path, "w")
    if not file then
        ngx.log(ngx.ERR, "Failed to open clients file: ", err)
        return false
    end

    local newstr = cjson.encode(json)
    file:write(newstr)
    file:close()
    return true
end

return _M