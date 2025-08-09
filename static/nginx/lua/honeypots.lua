local _M = {}

-- get a table containing all configured honeypots
function _M.get_honeypots(ngx)
    local file_path = "/etc/nginx/bg_conf/honeypots"
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open honeypots file: ", err)
        return {}
    end

    local endpoints = {}
    for endpoint in file:lines() do
        table.insert(endpoints, endpoint)
    end

    file:close()
    return endpoints
end

-- Check if a client endpoint is a honeypot
function _M.is_honeypot(ngx)
    -- read the honeypot file
    local file_path = "/etc/nginx/bg_conf/honeypots"
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open honeypots file: ", err)
        return false
    end

    local is_honeypot = false
    local uri = ngx.var.uri

    for line in file:lines() do
        -- check if the line is non-empty and match the path against it
        if (line ~= "") then
            ngx.log(ngx.INFO, "checking ", line)
            if uri:match(line) then
                -- we matched! set to true and break out of the loop early
                ngx.log(ngx.INFO, "matched! path: ", line)
                is_honeypot = true
                break
            end
        end
    end

    file:close()
    return is_honeypot
end

return _M