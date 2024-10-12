defmodule Day2Part1 do
    def run do
        case IO.gets("") do
            :eof -> 0
            "" -> 0
            line -> 
                [_, move] = String.split(line)
                tline = String.trim(line)

                cond do
                    tline in ["A Y", "B Z", "C X"] -> 6
                    tline in ["A X", "B Y", "C Z"] -> 3
                    true -> 0
                end +
                %{"X" => 1, "Y" => 2, "Z" => 3}[move] +
                run()
        end
    end
end

IO.puts(Day2Part1.run())
