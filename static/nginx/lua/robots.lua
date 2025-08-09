local _M = {}

local honeypots = require("honeypots")

function _M.generate_robots_list(ngx)
    local pots = honeypots.get_honeypots(ngx)
    local content = "User-Agent: *\n"

    for _, pot in pairs(pots) do
        if pot:sub(1, 1) == "^" then
            pot = pot:sub(2)
        end

        content = content .. "Disallow: " .. pot .. "\n"
    end

    return content
end

return _M