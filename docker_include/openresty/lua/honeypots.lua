local _M = {}

function _M.is_honeypot(ngx)
    local file_path = "/etc/nginx/bg_conf/honeypots"
    local file, err = io.open(file_path, "r")
    if not file then
        ngx.log(ngx.ERR, "Failed to open honeypots file: ", err)
        return false
    end

    local is_honeypot = false
    local uri = ngx.var.uri
    for line in file:lines() do
        ngx.log(ngx.INFO, "checking ", line)
        if uri:match(line) then
            ngx.log(ngx.INFO, "matched! path: ", line)
            is_honeypot = true
        end
    end
    io.close(file)
    return is_honeypot
end

return _M