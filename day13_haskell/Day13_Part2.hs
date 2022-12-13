module Main where

import System.IO
import Data.Char (ord)
import Data.List (elemIndex)

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

-- Define how to compare two Packets for equality; needed to make elemIndex() work later on.
instance Eq Packet where
    One a == One b   = a == b
    Many a == Many b = a == b
    _ == _           = False

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
printPackets [] = do return ()
printPackets (packet:rest) = do
    putStrLn(packetToStr packet)
    printPackets rest


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
            
-- True iff a is strictly "less than" b; false if they are equal, or a is greater.
strictlyInOrder a b =
    case inOrder a b of
        Nothing -> False
        Just r -> r
            
            
-- Read all packets into a list, ignoring blank lines
readPackets = do
    end <- isEOF
    if end
        then return []
        else do
            line <- getLine
            next <- readPackets
            return (if line == ""
                    then next
                    else (parsePacketLine line):next)
             
             
-- Quicksort a list of packets
sortPackets :: [Packet] -> [Packet]
sortPackets [] = []
sortPackets [a] = [a]
sortPackets (pivot:rest) =
    let small = filter (\p -> strictlyInOrder p pivot) rest
        large = filter (\q -> strictlyInOrder pivot q) rest
    in
        (sortPackets small) ++ [pivot] ++ (sortPackets large)
        

main = do
    packets <- readPackets
    let divider1 = Many[Many[One 2]]
        divider2 = Many[Many[One 6]]
        sortedPackets = sortPackets (divider1:divider2:packets)
        index1 = case (elemIndex divider1 sortedPackets) of
            Nothing -> -1
            Just i -> i + 1
        index2 = case (elemIndex divider2 sortedPackets) of
            Nothing -> -1
            Just i -> i + 1
    
    printPackets sortedPackets
    putStrLn("Divider 1 index == " ++ show(index1))
    putStrLn("Divider 2 index == " ++ show(index2))
    putStrLn("Decoder key == " ++ show(index1 * index2))

