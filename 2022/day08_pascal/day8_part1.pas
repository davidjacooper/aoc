program Day8_Part1;
var
    grid: array[1..99, 1..99] of shortint;
    invisibility: array[1..99, 1..99] of byte; 

    line: string;
    i: integer;
    j: integer;
    maxHeight: shortint;
    nCols: integer;
    nRows: integer;
    nInvisible: integer;

procedure checkVisible; begin
    if grid[i,j] > maxHeight then 
        maxHeight := grid[i,j]
    else 
        inc(invisibility[i,j]);
end;

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

    for i := 2 to nRows - 1 do begin
        maxHeight := grid[i,1];
        for j := 2 to nCols - 1 do checkVisible;
        
        maxHeight := grid[i,nCols];
        for j := nCols - 1 downto 2 do checkVisible;
        
    end;

    for j := 2 to nCols - 1 do begin
        maxHeight := grid[1,j];
        for i := 2 to nRows - 1 do checkVisible;

        maxHeight := grid[nRows,j];
        for i := nRows - 1 downto 2 do checkVisible;
    end;


    nInvisible := 0;
    for i := 2 to nRows - 1 do
        for j := 2 to nCols - 1 do
            if invisibility[i,j] = 4 then
                inc(nInvisible);

    writeln((nCols * nRows) - nInvisible);
end.
