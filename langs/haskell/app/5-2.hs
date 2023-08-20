import qualified Data.List as L

main = interact $ show . foldl (\acc s -> acc + fromEnum (isGood s)) 0 . lines

isGood :: String -> Bool
isGood xs = hasPair xs && hasRepeat xs

hasPair :: String -> Bool
hasPair (x : y : zs) = L.isInfixOf [x, y] zs || hasPair (y : zs)
hasPair _ = False

hasRepeat :: String -> Bool
hasRepeat (x : y : z : xs) = x == z || hasRepeat (y : z : xs)
hasRepeat _ = False
