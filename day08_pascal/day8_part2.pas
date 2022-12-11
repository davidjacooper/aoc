program Day8_Part1;
var
    grid: array[1..99, 1..99] of shortint;

    line: string;
    i: integer;
    j: integer;
    nCols: integer;
    nRows: integer;
    
    distanceUp: int64;
    distanceDown: int64;
    distanceLeft: int64;
    distanceRight: int64;
    
    scenicScore: int64;
    maxScenicScore: int64;

begin
    readln(line);
    nCols := length(line);
    nRows := 0;
    i := 1;
    while length(line) > 0 do begin
        nRows := nRows + 1;
        for j := 1 to length(line) do begin
            grid[i,j] := ord(line[j]) - ord('0');
        end;
        i := i + 1;
        readln(line);
    end;


    maxScenicScore := 0;
    
    for i := 2 to nRows - 1 do
        for j := 2 to nCols - 1 do begin
        
            { Look up }
            distanceUp := 0;
            repeat
                inc(distanceUp)
            until (i - distanceUp = 1) or (grid[i - distanceUp,j] >= grid[i,j]);

            { Look down }
            distanceDown := 0;
            repeat
                inc(distanceDown)
            until (i + distanceDown = nRows) or (grid[i + distanceDown,j] >= grid[i,j]);

            { Look left }
            distanceLeft := 0;
            repeat
                inc(distanceLeft)
            until (j - distanceLeft = 1) or (grid[i,j - distanceLeft] >= grid[i,j]);

            { Look right }
            distanceRight := 0;
            repeat
                inc(distanceRight)
            until (j + distanceRight = nCols) or (grid[i,j + distanceRight] >= grid[i,j]);

            scenicScore := distanceUp * distanceDown * distanceLeft * distanceRight;
            if maxScenicScore < scenicScore then
                maxScenicScore := scenicScore;
        end;
        
    writeln(maxScenicScore);
end.
