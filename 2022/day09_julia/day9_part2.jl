function main()
    deltas = Dict([("U", [0,1]),
                   ("D", [0,-1]),
                   ("R", [1,0]),
                   ("L", [-1,0])])
    
    knots = repeat([[0,0]], 10)
    all_positions = Set()
    
    push!(all_positions, knots[end])
    println("Positions: " * string(knots))
    
    while !eof(stdin)
    
        dir, distance = split(readline(), " ")
        
        for i = 1:parse(Int64, distance)
            knots[begin] += deltas[dir]
            
            for k = 2:length(knots)
                sep = knots[k - 1] - knots[k]
                
                if abs(sep[1]) > 1 || abs(sep[2]) > 1
                    knots[k] += [sign(sep[1]), sign(sep[2])]
                end
            end
            
            push!(all_positions, knots[end])
            println("Positions: " * string(knots))
        end
    end
    
    println(length(all_positions))
end

main()