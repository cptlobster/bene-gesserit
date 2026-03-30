local bit = require("bit")

-- IP address operation tools.
local _M = {}

local IPV4_MATCH_STR = "^(%d+)%.(%d+)%.(%d+)%.(%d+)$"
local IPV6_MATCH_STR = "^(%x%x%x%x):(%x%x%x%x):(%x%x%x%x):(%x%x%x%x):(%x%x%x%x):(%x%x%x%x)$"

function make_bitmask(n)
    if n <= 0 then
        return 0
    if n == 1 then
        return 1
    else
        return bit.bor(bit.lshift(1, n), make_bitmask(n - 1))
    end
end

-- Determine if an IP address is IPv4.
function is_ipv4(addr)
    return not not string.find(addr, IPV4_MATCH_STR)
end

-- Determine if an IP address is IPv6.
function is_ipv6(addr)
    return not not string.find(addr, IPV6_MATCH_STR)
end

-- convert an IPv4 address string to an array of integers for CIDR clipping purposes
function ipv4_to_array(addr)
    local arr = string.match(IPV4_MATCH_STR)
    if arr == nil then
        return
    end
    arr[1] = tonumber(arr[1])
    arr[2] = tonumber(arr[2])
    arr[3] = tonumber(arr[3])
    arr[4] = tonumber(arr[4])
    return arr
end

-- convert an array of integers to an IPv4 address string
function array_to_ipv4(addr)
    return tostring(addr[1]) .. "." .. tostring(addr[2]) .. "." .. tostring(addr[3]) .. "." .. tostring(addr[4])
end

-- convert an IPv6 address string to an array of integers for CIDR clipping purposes
function ipv6_to_array(addr)
    local arr = string.match(IPV6_MATCH_STR)
    if arr == nil then
        return
    end
    arr[1] = tonumber(arr[1], 16)
    arr[2] = tonumber(arr[2], 16)
    arr[3] = tonumber(arr[3], 16)
    arr[4] = tonumber(arr[4], 16)
    arr[5] = tonumber(arr[5], 16)
    arr[6] = tonumber(arr[6], 16)
    return arr
end

-- convert an array of integers to an IPv6 address string
function array_to_ipv6(addr)
    return tostring(addr[1]) .. ":" .. tostring(addr[2]) .. ":" .. tostring(addr[3]) .. ":" .. tostring(addr[4])  .. ":" .. tostring(addr[5]) .. ":" .. tostring(addr[6])
end

-- Reduce an IPv4 address's precision to a specific CIDR level.
function clip_cidr_v4(addr, level)
    -- bit twiddling fuckery
    -- if we're trying to get /32, just return the address as-is
    if level == 32 then
        return addr
    -- if 0 or negative return 0.0.0.0/0
    if level <= 0 then
        addr[1] = 0
        addr[2] = 0
        addr[3] = 0
        addr[4] = 0
        return addr
    -- if we're below /32, start doing bit shift stuff
    else if level < 32 then
        -- figure out how much of each digit we have to cover
        local sublevel = level % 8
        -- create a bit mask to AND with once we deal with each sublevel
        local mask = bit.bnot(make_bitmask(8 - sublevel))
        -- if it's 0, return the whole thing
        -- might not be the right behavior with how modulus behaves? will have to test
        if sublevel == 0 then
            mask = 0xffffffff
        end
        if level <= 8 then
            addr[2] = 0
            addr[3] = 0
            addr[4] = 0
            addr[1] = bit.band(addr[1], mask)
        else if level <= 16 then
            addr[3] = 0
            addr[4] = 0
            addr[2] = bit.band(addr[2], mask)
        else if level <= 24 then
            addr[4] = 0
            addr[3] = bit.band(addr[3], mask)
        else
            addr[4] = bit.band(addr[4], mask)
        end
        -- return the masked address
        return addr
    end
end

-- Reduce an IPv6 address's precision to a specific CIDR level.
function clip_cidr_v6(addr, level)
    if level <= 128 then

    end
end

-- Determine if a string is a valid IP address.
function _M.is_ip(addr)
    return is_ipv4(addr) or is_ipv6(addr)
end

-- Reduce an IP address's precision to the region level (CIDR /24 for IPv4, /48 for IPv6)
function _M.to_region(addr)
    if is_ipv4(addr) then
        return clip_cidr_v4(addr, 24)
    elseif is_ipv6(addr) then
        return clip_cidr_v6(addr, 48)
    end
end

return _M