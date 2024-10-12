defmodule Day2Part2 do
    def run do
        case IO.gets("") do
            :eof -> 0
            "" -> 0
            line -> 
                [_, result] = String.split(line)
                tline = String.trim(line)
                
                move = cond do
                    # Choose rock  to draw (Y) against rock (A), lose (X) against paper (B), or win  (Z) against scissors.
                    tline in ["A Y", "B X", "C Z"] -> 1 
                    
                    # Choose paper to win  (Z) against rock (A), draw (Y) against paper (B), or lose (X) against scissors.
                    tline in ["A Z", "B Y", "C X"] -> 2 
                    
                    # Choose scissors in all other cases
                    true -> 3                           
                end
                
                move + %{"X" => 0, "Y" => 3, "Z" => 6}[result] + run()
        end
    end
end

IO.puts(Day2Part2.run())
