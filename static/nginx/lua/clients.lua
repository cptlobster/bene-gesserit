local fileutils = require "utils/files"
local jsontable = require "utils/jsontable"
local cjson = require "cjson"
local ck = require "resty.cookie"
cjson.decode_array_with_array_mt(true)

ANUBIS_VERIFICATION_COOKIE = "techaro.lol-anubis-cookie-verification"
ANUBIS_JWT = "techaro.lol-anubis-auth"

-- Functions for handling the client database. At the moment it is stored in a
-- single JSON file. This needs to be improved significantly at some point.
local _M = {}

function _M.fallback_id(ngx)
    ngx.log(ngx.WARN, "Falling back to IP-based identification...")
    local headers = ngx.req.get_headers()
    local ip = headers["x-real-ip"]
    if not ip then
        ngx.log(ngx.ERR, "No IP address found. Dumping headers")
        ngx.log(ngx.ERR, cjson.encode(headers))
        return
    end
    return "ip-" .. string.gsub(ip, "%.", "-")
end

-- Get a client's ID from their Anubis token
function _M.get_id(ngx)
    -- instantiate the cookie library
    local cookie, err = ck:new()
    if not cookie then
        ngx.log(ngx.ERR, err)
        return _M.fallback_id(ngx)
    end
    
    -- get all the cookies
    local fields, err = cookie:get_all()
    if not fields then
        ngx.log(ngx.ERR, err)
        return _M.fallback_id(ngx)
    end

    -- This is a confusing section, since Anubis's cookie logic is weird. I thought that they used the same cookie for a
    -- while (they did not at one point, I had a couple instances where Anubis used different named cookies), but then
    -- I also thought that they rotated cookies (they do not?). They might have just changed them between updates. Here
    -- is an explanation of Anubis's cookie logic as I understand it now.
    -- - Anubis uses a signed JWT to store user information and challenge results. This contains a UUID that corresponds
    --   to each unique client. This is the UUID that I want to use. As of this writing it is stored in the cookie
    --   "techaro.lol-anubis-auth".
    -- - Sometimes Anubis adds a cookie called "techaro.lol-anubis-cookie-verification" which also contains that UUID in
    --   plaintext. If this cookie is present, it saves (a minimal amount of) processing that would go into decoding the
    --   JWT, so we check for that as well.
    -- There are probably many better ways to do this. This will hopefully work for now, and I am 99% sure that this
    -- will fallback properly to IP based identification on any failure. If it doesn't, THAT IS A PROBLEM!
    local cookie_uuid = nil
    for k, v in pairs(fields) do
        -- verification cookie has the same UUID as the JWT. if this is present, use that and don't worry about decoding
        -- the JWT
        if k == ANUBIS_VERIFICATION_COOKIE then
            cookie_uuid = v
            break
        end

        -- THIS DOES NOT VERIFY THE SIGNATURE OF THE JWT PAYLOAD. THIS ASSUMES THAT ANUBIS HAS CAUGHT ANY JWT ERRORS AND
        -- REJECTED THOSE REQUESTS. IF ANUBIS DOES NOT VERIFY ITS OWN JWTS THIS COULD BE ABUSED. THIS IS A HORRIBLE WAY
        -- TO GO ABOUT THIS BUT I DON'T WANT TO FIGURE OUT HOW TO SYNCHRONIZE THE PUBLIC KEY BETWEEN ANUBIS AND BENE
        -- GESSERIT IF I DON'T HAVE TO.
        -- if the JWT is here, manually decode it
        if k == ANUBIS_JWT then
            -- get boundary of JWT payload
            local payload_start = string.find(v, ".", 1, true)
            local payload_end = string.find(v, ".", start + 1, true)

            -- decode the payload
            local payload = cjson:decode(string.sub(v, payload_start, payload_end))

            if payload ~= nil then
                if payload["challenge"] ~= nil then
                    cookie_uuid = payload["challenge"]
                    break
                end
            end
        end
    end

    if cookie_uuid ~= nil then
        return "anb-" .. cookie_uuid
    else
        -- if there is no anubis cookie (i.e. curl), use the X-Real-Ip header as the ID. Less precise, but should work
        return _M.fallback_id(ngx)
    end
end

-- Get a client's record
function _M.get_client(ngx)
    local headers = ngx.req.get_headers()
    local ip = headers["x-real-ip"]
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
