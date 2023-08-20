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
hasDouble [] = False
hasDouble [_] = False
hasDouble (x : y : xs)
  | x == y = True
  | otherwise = hasDouble (y : xs)

includesBadElems :: String -> Bool
includesBadElems [] = False
includesBadElems [_] = False
includesBadElems (x : y : xs)
  | [x, y] `elem` ["ab", "cd", "pq", "xy"] = True
  | otherwise = includesBadElems (y : xs)