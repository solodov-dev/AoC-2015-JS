main = interact $ show . foldl (\acc s -> acc + fromEnum (isGood s)) 0 . lines

isGood :: String -> Bool
isGood xs = hasThreeVowels xs 0 && hasDouble xs && not (includesBadElems xs)

hasThreeVowels :: String -> Int -> Bool
hasThreeVowels [] n = n > 2
hasThreeVowels (x : xs) n
  | n > 2 = True
  | x `elem` ['a', 'o', 'e', 'i', 'u'] = hasThreeVowels xs (n + 1)
  | otherwise = hasThreeVowels xs n

hasDouble :: String -> Bool
hasDouble (x : y : zs) = x == y || hasDouble (y : zs)
hasDouble _ = False

includesBadElems :: String -> Bool
includesBadElems (x : y : zs) = [x, y] `elem` ["ab", "cd", "pq", "xy"] || includesBadElems (y : zs)
includesBadElems _ = False