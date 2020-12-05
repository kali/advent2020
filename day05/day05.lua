local ids = {}
for seat in io.lines("input") do
    local id = 0
     for i = 1, #seat do
         local c = seat:sub(i,i)
         id = id * 2 + ((c == 'R' or c == 'B') and 1 or 0)
     end
     table.insert(ids, id)
end
table.sort(ids)
print(ids[#ids])
for i = 1, #ids do
    if ids[i] + 2 == ids[i+1] then
        print(ids[i] + 1)
    end
end
