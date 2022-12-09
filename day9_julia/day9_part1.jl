function main()
    deltas = Dict([("U", [0,1]),
                   ("D", [0,-1]),
                   ("R", [1,0]),
                   ("L", [-1,0])])

    head = [0, 0]
    tail = [0, 0]
    all_positions = Set()
    
    push!(all_positions, tail)
    println("Head: " * string(head) * ", Tail: " * string(tail))
    
    while !eof(stdin)
    
        dir, distance = split(readline(), " ")
        
        for i = 1:parse(Int64, distance)
            head += deltas[dir]
            sep = head - tail
            
            if abs(sep[1]) > 1 || abs(sep[2]) > 1
                tail += [sign(sep[1]), sign(sep[2])]
            end
            
            push!(all_positions, tail)
            println("Head: " * string(head) * ", Tail: " * string(tail))
        end
    end
    
    println(length(all_positions))
end

main()

