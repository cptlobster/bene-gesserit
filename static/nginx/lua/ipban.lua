local fileutils = require "utils/files"
local iputils = require "utils/ip"

local _M = {}

-- Check if an IP address is banned, either at the address or region level.
-- TODO: implement checks for ASNs
function _M.is_banned(ngx, client, limit, region_limit = 0)
    local iplist = fileutils.read_json(ngx, "/etc/nginx/bg_conf/ipban.json")
    local region = iputils.to_region(client.ip)
    if iplist[region] then
        -- check individual IP address limit
        if iplist[region][client.ip] >= limit then
            return true
        end
        -- region limit is disabled if set to 0
        if region_limit > 0 then
            local sum = 0
            for k, v in pairs(iplist[region]) do
                sum += v
            end
            if sum >= region_limit then
                return true
            end
        end
    end
    return false
end

-- Increment the violations for an IP address.
function _M.increment_violations(ngx, client)
    local iplist = fileutils.read_json(ngx, "/etc/nginx/bg_conf/ipban.json")
    local region = iputils.to_region(client.ip)
    if iplist[region] == nil then
        iplist[region] = { client.ip = 1 }
    else if iplist[region][client.ip] == nil then
        iplist[region][client.ip] = 1
    else
        iplist[region][client.ip] += 1
    end
    fileutils.write_json(ngx, "/etc/nginx/bg_conf/ipban.json", iplist)
end

return _M