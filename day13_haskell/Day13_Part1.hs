module Main where

import System.IO
import Data.Char (ord)

-- Rolling my own integer parser, because why not. It returns both the actual number AND the suffix
-- string after the number ends, which I'll need for the rest of the parsing.
--
-- (In fact it returns a third value -- the number of digits -- but this is just for internal use.)
atoi :: [Char] -> (Int, Int, [Char])
atoi (ch:end:trailing) | (end == ',' || end == ']') =
    ((ord ch) - (ord '0'), 1, end:trailing)
    
atoi (ch:rest) =
    let (nextValue, nextDigits, trailing) = atoi rest
    in (
        ((ord ch) - (ord '0')) * (10 ^ nextDigits) + nextValue,
        nextDigits + 1,
        trailing
    )
    

-- Parse a string into a hierarchical list of numbers/lists. Similar to atoi, parsePacket (which
-- calls itself recursively) returns both the actual parsed structure or sub-structure (Packet) as
-- well as the suffix string containing the rest of the unparsed text.
data Packet = One Int | Many [Packet]

parsePacket :: [Char] -> ([Packet], [Char])


-- Strings starting with '[' represent the start of a packet (or nested packet).
parsePacket ('[':rest) =
    let (nested, rest2) = parsePacket rest
    in
        let (suffix, rest3) = parsePacket rest2
        in
            ((Many nested):suffix, rest3)
            
-- When we get to ']', we've reached the end of a packet / nested packet.
parsePacket (']':rest) = ([], rest)

-- ',' is basically ignored (except by atoi where it serves to terminate a number)
parsePacket (',':rest) = parsePacket rest

-- In all other cases, we assume we're looking at a number (plus the remainder of the packet text).
parsePacket str =
    let (value, _, rest) = atoi str
    in
        let (suffix, rest2) = parsePacket rest
        in
            ((One value):suffix, rest2)

-- Convenience function to discard the suffix text return value from parsePacket (which is only for
-- internal use).
parsePacketLine line = let (packet:_, _) = parsePacket(line) in packet


-- Auxilliary code to visualise & validate a parsed 'packet'.
packetToStr (One i) = show i
packetToStr (Many list) = "{" ++ (packetListToStr list) ++ "}"
packetListToStr [] = ""
packetListToStr [only] = packetToStr only
packetListToStr (first:rest) = (packetToStr first) ++ ";" ++ packetListToStr rest



-- Compare two packets to determine whether they are 'in order'.
inOrder :: Packet -> Packet -> Maybe Bool

-- Compare individual integers
inOrder (One a) (One b) =
    if a == b
        then Nothing
        else Just (a < b)
        
-- Translate int-list comparisons to list-list comparisons
inOrder (One a) (Many b) = inOrder (Many[One a]) (Many b)
inOrder (Many a) (One b) = inOrder (Many a) (Many[One b])

-- Handle terminal cases for list comparisons
inOrder (Many[]) (Many[]) = Nothing -- Lists are equivalent (once we get to their ends)
inOrder (Many[]) _ = Just True      -- Left list runs out of items first
inOrder _ (Many[]) = Just False     -- Right list runs out of items first

-- For a list-to-list comparison, compare the first items in the list, then recurse if they're equal
inOrder (Many (aFirst:aRest)) (Many (bFirst:bRest)) =
    let res = inOrder aFirst bFirst
    in
        if res == Nothing
            then inOrder (Many aRest) (Many bRest)
            else res
            
            
-- Iterate over the pairs of packets in the file, test the ordering of each pair, and add up
-- in-order indexes.
testPairs :: Int -> IO Int
testPairs firstIndex = do
    end <- isEOF
    if end
        then return 0
        else do
            line1 <- getLine
            line2 <- getLine
            end <- isEOF
            if end then do return "" else do getLine -- discard blank line if it exists
            let thisResult = inOrder (parsePacketLine line1) (parsePacketLine line2)
            case thisResult of
                Nothing -> putStrLn(show(firstIndex) ++ ": Packets are identical")
                Just True -> putStrLn(show(firstIndex) ++ ": Packets are in order")
                Just False -> putStrLn(show(firstIndex) ++ ": Packets are out of order")

            nextResult <- testPairs (firstIndex + 1)
            return ((if thisResult == Just True then firstIndex else 0) + nextResult)


main = do
    result <- testPairs 1
    putStrLn(show(result))


