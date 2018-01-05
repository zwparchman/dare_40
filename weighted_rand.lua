--
--------------------------------------------------------------------------------
--         File:  weighted_rand.lua
--
--        Usage:  ./weighted_rand.lua
--
--  Description:  
--
--      Options:  ---
-- Requirements:  ---
--         Bugs:  ---
--        Notes:  ---
--       Author:  YOUR NAME (), <>
-- Organization:  
--      Version:  1.0
--      Created:  01/05/18
--     Revision:  ---
--------------------------------------------------------------------------------
--

local weighted_rand = {
}

function weighted_rand:new()
    o = {
        items = {}
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function weighted_rand:add(weight, item)
    for i=1,weight do
        table.insert(self.items, item)
    end
end

function weighted_rand:get()
    local index = math.tointeger(math.random(1,#self.items))
    return self.items[index]
end

return weighted_rand
