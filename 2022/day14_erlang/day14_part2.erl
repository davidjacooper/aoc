#!/usr/bin/env escript

% Note: could use some optimisation. Takes a little while to arrive at the correct answer, and
% unfortunately I had to disable the data structures that would allow the progress to be
% animated (or else the process would run out of resources and get killed).

-define(SandOriginX, 500).
-define(SandOriginY, 0).
-define(FloorDepth, 2).
-define(ViewRangeX, 100).
-define(ViewRangeY, 30).
-define(DelayMS, 0).


parseCoords(S) ->
    [XStr,YStr] = string:split(S, ","),
    {X,_} = string:to_integer(XStr),
    {Y,_} = string:to_integer(YStr),
    {X,Y}.


parseLine(Line) ->
    case string:split(Line, " -> ") of
        [FirstPair,Rest] -> [parseCoords(FirstPair)] ++ parseLine(Rest);
        [FirstPair]      -> [parseCoords(FirstPair)]
    end.
    
plotLine({X1, Y1}, {X2, Y2}) when X1 == X2 ->
    sets:from_list(lists:map(fun(Y) -> {X1,Y} end, lists:seq(min(Y1, Y2), max(Y1, Y2))));
    
plotLine({X1, Y1}, {X2, Y2}) when Y1 == Y2 ->
    sets:from_list(lists:map(fun(X) -> {X,Y1} end, lists:seq(min(X1, X2), max(X1, X2)))).
    
    
plotStructure([C1, C2 | Rest]) ->
    case Rest of
        [] -> plotLine(C1, C2);
        _  -> sets:union(plotLine(C1, C2), plotStructure([C2] ++ Rest))
    end.
    
readInput() ->
    case io:get_line("") of
        eof  -> sets:new();
        Line -> sets:union(plotStructure(parseLine(Line)),
                           readInput())
    end.

getBounds(SList) ->
    case SList of
        [{X, Y}]        -> {X, Y, X, Y};
        [{X, Y} | Rest] ->
            {XMin, YMin, XMax, YMax} = getBounds(Rest),
            {min(X, XMin), min(Y, YMin), max(X, XMax), max(Y, YMax)}
    end.
    
makeView({XMin, YMin, XMax, YMax}, SList, Char, NextSList) ->
    Width = XMax - XMin + 1,
    Height = YMax - YMin + 1,
    case SList of
        [] ->
            case NextSList of
                [] -> array:new(Width * Height, {default, $.});
                _  -> NextSList
            end;
            
        [{X, Y} | Rest] ->
            Next = makeView({XMin, YMin, XMax, YMax}, Rest, Char, NextSList),
            case (XMin =< X) and (X =< XMax) and (YMin =< Y) and (Y =< YMax) of
                true -> array:set((X - XMin) + (Width * (Y - YMin)), Char, Next);
                false -> Next
            end
    end.
    
printView({XMin, YMin, XMax, YMax}, ViewList) ->
    case ViewList of
        [] -> undefined;
        _  ->
            Width = XMax - XMin + 1,
            {Row, Rest} = lists:split(Width, ViewList),
            io:format("~s\n", [Row]),
            printView({XMin, YMin, XMax, YMax}, Rest)
    end.
    

printView({XMin, YMin, XMax, YMax}, RockList, SandList, SandX, SandY) ->
    ViewXMin = max(XMin, SandX - (?ViewRangeX div 2)),
    ViewYMin = max(YMin, SandY - (?ViewRangeY div 2)),
    ViewXMax = min(XMax, ViewXMin + ?ViewRangeX),
    ViewYMax = min(YMax, ViewYMin + ?ViewRangeY),
    ViewBounds = {ViewXMin, ViewYMin, ViewXMax, ViewYMax},
    
    ViewList = array:to_list(
        makeView(ViewBounds, [{SandX, SandY}], $O,
            makeView(ViewBounds, SandList, $o,
                makeView(ViewBounds, [{500,0}], $+,
                    makeView(ViewBounds, RockList, $#, [])
                )
            )
        )
    ),
    io:format("X\033[2J\n"),
    printView(ViewBounds, ViewList).
        
    

depositAndRespawn(Bounds, Set, RockList, SandList, SandX, SandY) ->
    % printView(Bounds, RockList, SandList, SandX, SandY),
    case (SandX == ?SandOriginX) and (SandY == ?SandOriginY) of
        true -> 1;
        false ->
            io:format("@"),
            1 + simulate(Bounds,
                         sets:add_element({SandX, SandY}, Set),
                         [], %RockList,
                         [], %SandList ++ [{SandX, SandY}],
                         ?SandOriginX,
                         ?SandOriginY)
    end.
    
    
simulate(Bounds, Set, RockList, SandList, SandX, SandY) ->
    timer:sleep(?DelayMS),
    {_, _, _, YMax} = Bounds,
    if
        % Sand has reached the floor.
        SandY >= YMax ->
            depositAndRespawn(Bounds, Set, RockList, SandList, SandX, SandY);
        
        true -> case sets:is_element({SandX, SandY + 1}, Set) of
            false ->
                % Sand is able to move down
                simulate(Bounds, Set, RockList, SandList, SandX, SandY + 1);
                    
            true -> case sets:is_element({SandX - 1, SandY + 1}, Set) of
                false ->
                    % Sand is able to move down/left.
                    simulate(Bounds, Set, RockList, SandList, SandX - 1, SandY + 1);
                            
                true -> case sets:is_element({SandX + 1, SandY + 1}, Set) of
                    false ->
                        % Sand is able to move down/right.
                        simulate(Bounds, Set, RockList, SandList, SandX + 1, SandY + 1);
            
                    % Blocked by an obstacle.
                    true ->
                        depositAndRespawn(Bounds, Set, RockList, SandList, SandX, SandY)
                end
            end
        end
    end.



main(_) ->
    RockSet = readInput(),
    RockList = sets:to_list(RockSet),
    
    % Must modify bounds to allow for horizontal spill-over.
    {StrictXMin, YMin, StrictXMax, StrictYMax} = getBounds([{?SandOriginX,?SandOriginY}] ++ RockList),
    
    Spread = StrictYMax + ?FloorDepth - 1 - ?SandOriginY,
    Bounds = {min(StrictXMin, ?SandOriginX - Spread), YMin,
              max(StrictXMax, ?SandOriginX + Spread), StrictYMax + ?FloorDepth - 1},

    % SandUnits = simulate(Bounds, RockSet, RockList, [], ?SandOriginX, ?SandOriginY),
    io:format("\033[2J"),
    SandUnits = simulate(Bounds, RockSet, [], [], ?SandOriginX, ?SandOriginY),
    io:format("\033[~wA~w sand units were deposited\n", [?ViewRangeY, SandUnits])
    .
    